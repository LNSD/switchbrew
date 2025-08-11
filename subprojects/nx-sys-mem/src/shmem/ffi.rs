//! C FFI bindings for shared memory operations
//!
//! These bindings provide `#[no_mangle]` C-callable functions whose
//! signatures align with the declarations in `nx_shmem.h`.

use core::{
    ffi::c_void,
    ptr::{self, NonNull},
};

use nx_svc::{
    error::{KernelError, ToRawResultCode},
    mem::shmem::Handle,
    raw::{Handle as RawHandle, INVALID_HANDLE},
};

use super::sys::{self, Mapped, Unmapped};

/// Shared memory object (C-compatible wrapper)
#[repr(C)]
struct SharedMemory {
    handle: RawHandle,
    size: usize,
    perm: u32,
    map_addr: *mut c_void,
}

/// Creates a shared memory object.
///
/// Corresponds to `shmemCreate()` in `shmem.h`.
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_shmem_create(
    s: *mut SharedMemory,
    size: usize,
    local_perm: u32,
    remote_perm: u32,
) -> u32 {
    if s.is_null() {
        return KernelError::InvalidPointer.to_rc();
    }

    match unsafe {
        sys::create(
            size,
            sys::LocalPermissions::from_bits_retain(local_perm),
            sys::RemotePermissions::from_bits_retain(remote_perm),
        )
    } {
        Ok(shm) => {
            let sm = unsafe { &mut *s };
            sm.handle = shm.handle().to_raw();
            sm.size = shm.size();
            sm.perm = shm.perm().bits();
            sm.map_addr = ptr::null_mut();
            0
        }
        Err(err) => err.into_rc(),
    }
}

/// Loads a remote shared memory object (pure struct copy).
///
/// Corresponds to `shmemLoadRemote()` in `shmem.h`.
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_shmem_load_remote(
    s: *mut SharedMemory,
    handle: u32,
    size: usize,
    perm: u32,
) {
    if s.is_null() {
        return;
    }

    let sm = unsafe { &mut *s };
    sm.handle = handle;
    sm.size = size;
    sm.perm = perm;
    sm.map_addr = ptr::null_mut();
}

/// Maps a shared memory object into the current process.
///
/// Corresponds to `shmemMap()` in `shmem.h`.
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_shmem_map(s: *mut SharedMemory) -> u32 {
    if s.is_null() {
        return KernelError::InvalidPointer.to_rc();
    }
    let sm = unsafe { &mut *s };

    // Prevent double-mapping (behaves like libnx).
    if !sm.map_addr.is_null() {
        return LIBNX_ERR_ALREADY_MAPPED;
    }

    let unmapped = unsafe {
        {
            sys::SharedMemory::<Unmapped>::from_parts(
                Handle::from_raw(sm.handle),
                sm.size,
                sys::Permissions::from_bits_retain(sm.perm),
            )
        }
    };
    match unsafe { sys::map(unmapped) } {
        Ok(mapped) => {
            sm.map_addr = mapped.addr().unwrap_or(ptr::null_mut());
            0
        }
        Err(err) => match err {
            sys::MapError::VirtAddressAllocFailed => LIBNX_ERR_OUT_OF_MEMORY,
            sys::MapError::Svc(svc_err) => svc_err.to_rc(),
        },
    }
}

/// Unmaps a shared memory object.
///
/// Corresponds to `shmemUnmap()` in `shmem.h`.
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_shmem_unmap(s: *mut SharedMemory) -> u32 {
    if s.is_null() {
        return KernelError::InvalidPointer.to_rc();
    }

    let sm = unsafe { &mut *s };
    let Some(map_addr) = NonNull::new(sm.map_addr) else {
        // Nothing mapped – treat as success per libnx semantics.
        return 0;
    };

    let mapped = unsafe {
        sys::SharedMemory::<Mapped>::from_parts(
            Handle::from_raw(sm.handle),
            sm.size,
            sys::Permissions::from_bits_retain(sm.perm),
            map_addr,
        )
    };

    match unsafe { sys::unmap(mapped) } {
        Ok(_unmapped) => {
            sm.map_addr = ptr::null_mut();
            0
        }
        Err(err) => err.reason.to_rc(),
    }
}

/// Returns the mapped address of the shared memory object.
///
/// Corresponds to `shmemGetAddr()` in `shmem.h`.
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_shmem_get_addr(s: *mut SharedMemory) -> *mut c_void {
    if s.is_null() {
        return ptr::null_mut();
    }

    let sm = unsafe { &*s };
    sm.map_addr
}

/// Frees resources (unmap + close).
///
/// Corresponds to `shmemClose()` in `shmem.h`.
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_shmem_close(s: *mut SharedMemory) -> u32 {
    if s.is_null() {
        return KernelError::InvalidPointer.to_rc();
    }

    let sm = unsafe { &mut *s };

    // If mapped, unmap first.
    if !sm.map_addr.is_null() {
        let rc = unsafe { __nx_shmem_unmap(s) };
        if rc != 0 {
            return rc;
        }
    }

    let unmapped = unsafe {
        {
            sys::SharedMemory::<Unmapped>::from_parts(
                Handle::from_raw(sm.handle),
                sm.size,
                sys::Permissions::from_bits_retain(sm.perm),
            )
        }
    };
    match unsafe { sys::close(unmapped) } {
        Ok(()) => {
            sm.handle = INVALID_HANDLE;
            0
        }
        Err(err) => err.reason.to_rc(),
    }
}

// Helper: builds a libnx-style result-code from a description value.
#[inline]
const fn libnx_rc(desc: u32) -> u32 {
    const MODULE_LIBNX: u32 = 345; // 0x159
    (MODULE_LIBNX & 0x1FF) | (desc << 9)
}

const LIBNX_ERR_OUT_OF_MEMORY: u32 = libnx_rc(2);
const LIBNX_ERR_ALREADY_MAPPED: u32 = libnx_rc(3);
