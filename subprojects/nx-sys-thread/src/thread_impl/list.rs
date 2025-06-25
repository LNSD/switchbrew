//! # Thread registry (process-wide)
//!
//! This module owns a global collection that tracks every [`Thread`] that is
//! alive in the current process.
//!
//! ## Design at a glance
//!
//! • **Storage** – Threads are _not_ inserted directly into the intrusive list
//!   because that would change the C ABI of [`Thread`]. Instead, every entry
//!   is a heap-allocated `Box<Node>` that contains the intrusive
//!   [`LinkedListLink`] and a raw, non-null pointer back to the real [`Thread`]
//!   object. The allocation is created on insertion and destroyed immediately
//!   after removal.
//!
//! • **Global access** – A `static` `Mutex<ThreadList>` called
//!   `THREAD_LIST` serialises all mutations.
//!
//! • **Thread-safety** – User code can only access the underlying [`Thread`]s
//!   while the global mutex is held, guaranteeing data-race freedom even
//!   though the list itself stores raw pointers.
//!
//! The public API purposefully avoids returning raw pointers or references
//! that outlive the mutex guard. If you need to operate on every live thread,
//! pass a closure to [`for_each`] _while holding the guard_ so that borrow
//! rules remain intact.
//!
//! None of the public functions allocate except when `insert` needs to create
//! a new `Node`. All operations are `O(n)` in the number of live threads, but
//! the collections are small in practice so this has not been a concern.
//!
//! [`Thread`]: super::info::Thread
//! [`LinkedListLink`]: intrusive_collections::LinkedListLink

use alloc::boxed::Box;
use core::ptr::NonNull;

use intrusive_collections::{LinkedList, LinkedListLink, intrusive_adapter};
use nx_std_sync::mutex::Mutex;

use super::info::Thread;

/// A mutex-protected lazy-initialised linked list of [`Thread`]s.
static THREAD_LIST: Mutex<ThreadList> = Mutex::new(ThreadList::new_uninit());

/// Registers a freshly initialised [`Thread`] with the global registry.
///
/// The function is **O(1)** except for the initial call, which needs to
/// allocate a [`Node`].  The caller must guarantee that `thread` is fully
/// initialised, unique (i.e. not already present in the registry), and will
/// remain alive until it is later removed via [`remove`].
///
/// # Safety
///
/// Calling this function is **unsafe** because the registry stores the raw
/// pointer derived from `thread` without any lifetime tracking. The caller
/// must guarantee **all** of the following:
///
/// 1. `thread` refers to a fully-initialised `Thread` value.
/// 2. The same `Thread` instance has **not** been inserted before.
/// 3. The pointed-to `Thread` remains alive (is neither moved nor dropped)
///    until it is later removed via [`remove`].
///
/// Breaking any of these requirements results in undefined behaviour.
pub unsafe fn insert(thread: &Thread) {
    unsafe { THREAD_LIST.lock().insert(thread) };
}

/// Unregisters a `Thread` from the global registry.
///
/// In release builds the function is a silent no-op if the thread is not
/// present, because that indicates a logic error but is not otherwise
/// harmful.  In debug builds a breakpoint is triggered to aid debugging.
///
/// # Safety
///
/// The caller must ensure that `thread` was previously inserted with
/// [`insert`] _and_ has not been removed already.  Passing any other pointer
/// (including a non-registered or already-removed thread) triggers a debug
/// breakpoint in debug builds and is treated as _unreachable_ in release
/// builds (invoking `core::hint::unreachable_unchecked`).  Executing the
/// latter constitutes **undefined behaviour**.
pub unsafe fn remove(thread: &Thread) {
    unsafe { THREAD_LIST.lock().remove(thread) };
}

/// Runs `f` once for every live thread, while holding the registry mutex.
///
/// The closure must **not** panic, must **not** attempt to re-enter this
/// module (dead-locking), and must **not** store the received `&mut Thread`
/// beyond the call scope.  Breaking any of these rules results in undefined
/// behaviour.
///
/// # Safety
///
/// The supplied closure **must** adhere to the following constraints:
///
/// * It must **not** panic. Unwinding while the global mutex is locked would
///   poison it and leave the list in an inconsistent state.
/// * It must **not** attempt to re-enter this module (e.g. by calling
///   [`insert`], [`remove`], or another [`for_each`]); doing so would cause a
///   dead-lock because the mutex is already held.
/// * It must **not** persist the received `&mut Thread` reference beyond the
///   lifetime of the call; the reference is only valid while the mutex is
///   held.
pub unsafe fn for_each<F>(f: F)
where
    F: FnMut(&mut Thread),
{
    unsafe { THREAD_LIST.lock().for_each(f) };
}

/// A lazy-initialised linked list of [`Thread`]s.
struct ThreadList(Option<LinkedList<NodeAdapter>>);

impl ThreadList {
    /// Creates an uninitialised list state.
    const fn new_uninit() -> Self {
        Self(None)
    }

    /// Lazily initialises the inner list and returns a mutable reference to it.
    #[inline]
    fn get_or_init(&mut self) -> &mut LinkedList<NodeAdapter> {
        self.0
            .get_or_insert_with(|| LinkedList::new(NodeAdapter::new()))
    }

    /// Inserts a new thread into the global list.
    ///
    /// The caller must guarantee that `thread` points to a live, fully
    /// initialised [`Thread`] value that will stay alive until it is removed
    /// again with [`ThreadList::remove`].
    ///
    /// No duplicate-insertion check is performed for performance reasons:
    /// inserting the same `Thread` more than once corrupts the bookkeeping and
    /// results in **undefined behaviour**.
    ///
    /// # Safety
    ///
    /// The caller **must** uphold the following guarantees:
    /// 1. They hold the global mutex (`THREAD_LIST`) and therefore have
    ///    exclusive access to the underlying intrusive list for the entire
    ///    call.
    /// 2. `thread` refers to a live, fully-initialised [`Thread`] value that is
    ///    *not* already present in the list.
    /// 3. The pointed-to `Thread` will remain valid until it is later removed
    ///    via [`ThreadList::remove`].
    ///
    /// Violating any of these conditions results in **undefined behaviour**.
    #[inline]
    unsafe fn insert(&mut self, thread: &Thread) {
        let list = self.get_or_init();

        // Check if the thread is already present in the list
        #[cfg(debug_assertions)]
        {
            use nx_svc::debug::{BreakReason, break_event};
            if list
                .iter()
                .any(|node| node.thread_ptr() == NonNull::from(thread))
            {
                // The thread is already present in the list.
                // TODO: Add a proper error message here.
                // panic!("Attempted to insert a duplicate thread into ThreadList");
                break_event(BreakReason::Assert, 0, 0);
            }
        }

        // SAFETY: `thread` is a non-null pointer obtained from the caller.
        // The ThreadList guard we hold ensures no other thread can concurrently
        // remove the corresponding node while we create and link it, so the
        // pointer remains valid for the duration of this call.
        let node = unsafe { Node::new(NonNull::from(thread)) };
        list.push_front(node);
    }

    /// Removes a thread from the list.
    ///
    /// # Panics (debug builds only)
    /// Panics if the thread was not found. This usually indicates logical
    /// errors such as removing a thread twice.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// 1. They still hold the global mutex (`THREAD_LIST`).
    /// 2. `thread` is currently present in the list (i.e. was previously
    ///    inserted and not yet removed).
    ///
    /// If the thread is missing the implementation will trigger a debug
    /// breakpoint or invoke `core::hint::unreachable_unchecked` in release
    /// mode, leading to **undefined behaviour**.
    #[inline]
    unsafe fn remove(&mut self, thread: &Thread) {
        let Some(list) = self.0.as_mut() else {
            // The list is not initialised yet. No-op.
            return;
        };

        let thread_ptr = NonNull::from(thread);

        let mut cursor = list.cursor_mut();
        while let Some(node_ref) = cursor.get() {
            if node_ref.thread_ptr() == thread_ptr {
                let _ = cursor.remove();
                return;
            }
            cursor.move_next();
        }

        // In release configuration reaching this point indicates a severe
        // logic error. Mark it as unreachable to allow the optimiser to
        // assume it never happens.
        #[cfg(not(debug_assertions))]
        unsafe {
            core::hint::unreachable_unchecked();
        }
        #[cfg(debug_assertions)]
        {
            use nx_svc::debug::{BreakReason, break_event};
            // Reaching here means the thread was not present in the list.
            // TODO: Add a proper error message here.
            // panic!("Attempted to remove a non-existent thread from ThreadList");
            break_event(BreakReason::Assert, 0, 0);
        }
    }

    /// Runs `f` on each thread currently present in the list.
    ///
    /// The list's mutex is held for the entire duration of the traversal, so
    /// the closure is executed under exclusive access to every `Thread` in
    /// turn.
    ///
    /// # Safety
    /// * The closure **must not** store the `&mut Thread` beyond its
    ///   invocation; the reference is only valid while the mutex is held.
    /// * The closure must not attempt to acquire the global mutex again (e.g.
    ///   by calling `insert`, `remove`, or another `for_each`), otherwise a
    ///   dead-lock will occur.
    /// * The closure must not panic; unwinding while the mutex is locked would
    ///   poison it and could leave the list in an inconsistent state.
    #[inline]
    unsafe fn for_each<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Thread),
    {
        let Some(list) = self.0.as_mut() else {
            // The list is not initialised yet. No-op.
            return;
        };

        let mut cursor = list.cursor_mut();
        while let Some(node_ref) = cursor.get() {
            // SAFETY: We hold the global list mutex so the node cannot be
            // removed concurrently; therefore `thread` is still part of the
            // list and the pointer is valid for the lifetime of this closure
            // invocation.
            let thread = unsafe { &mut *node_ref.thread.as_ptr() };
            f(thread);
            cursor.move_next();
        }
    }
}

// Generate an adapter so the intrusive list knows how to reach `link` inside `Node`.
intrusive_adapter!(NodeAdapter = Box<Node>: Node { link: LinkedListLink });

/// Wrapper stored inside the intrusive linked list.
///
/// The sole purpose of this structure is to attach a [`LinkedListLink`] to the
/// raw [`Thread`] pointer without altering the ABI of `Thread` itself.
struct Node {
    /// Intrusive list link.
    link: LinkedListLink,
    /// Raw pointer to the thread information block.
    thread: NonNull<Thread>,
}

impl Node {
    /// Creates a new boxed node from a [`Thread`] pointer.
    ///
    /// # Safety
    /// The caller must guarantee that `thread` outlives the returned `Node`.
    #[inline]
    unsafe fn new(thread: NonNull<Thread>) -> Box<Self> {
        Box::new(Self {
            link: LinkedListLink::new(),
            thread,
        })
    }

    /// Returns the raw pointer to the underlying [`Thread`].
    #[inline(always)]
    fn thread_ptr(&self) -> NonNull<Thread> {
        self.thread
    }
}

// SAFETY: `Node` only contains a raw pointer and an intrusive link (which is
// essentially a couple of raw pointers as well).  Moving a `Node` between
// threads does not violate safety invariants because the pointed-to `Thread`
// object itself is never accessed concurrently without first taking
// `THREAD_LIST`'s mutex. Therefore it is safe to mark it as `Send`.
unsafe impl Send for Node {}
