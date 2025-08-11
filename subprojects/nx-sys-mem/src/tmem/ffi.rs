//! C FFI bindings for transfer memory operations
//!
//! These bindings provide `#[no_mangle]` C-callable functions whose
//! signatures align with the declarations in `nx_tmem.h`.

use core::{
    ffi::c_void,
    ptr::{self, NonNull},
};

use nx_svc::{
    error::{KernelError, ToRawResultCode},
    mem::tmem::Handle,
    raw::{Handle as RawHandle, INVALID_HANDLE},
};

use super::sys;

/// Transfer memory object (C-compatible wrapper)
#[repr(C)]
struct TransferMemory {
    handle: RawHandle,
    size: usize,
    perm: u32,
    src_addr: *mut c_void,
    map_addr: *mut c_void,
}

/// Build a libnx-style result-code from a description value.
#[inline]
const fn libnx_rc(desc: u32) -> u32 {
    const MODULE_LIBNX: u32 = 345; // 0x159
    (MODULE_LIBNX & 0x1FF) | (desc << 9)
}

// Add libnx error constants used below.
const LIBNX_ERR_OUT_OF_MEMORY: u32 = libnx_rc(2);
const LIBNX_ERR_ALREADY_MAPPED: u32 = libnx_rc(3);
const LIBNX_ERR_BAD_INPUT: u32 = libnx_rc(11);

/// Creates a transfer memory object.
///
/// Corresponds to `tmemCreate()` in `tmem.h`.
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_tmem_create(t: *mut TransferMemory, size: usize, perm: u32) -> u32 {
    if t.is_null() {
        return KernelError::InvalidPointer.to_rc();
    }

    match unsafe { sys::create(size, sys::Permissions::from_bits_retain(perm)) } {
        Ok(tm) => {
            let tm_ref = unsafe { &mut *t };
            tm_ref.handle = tm.handle().to_raw();
            tm_ref.size = tm.size();
            tm_ref.perm = tm.perm().bits();
            tm_ref.map_addr = ptr::null_mut();
            tm_ref.src_addr = tm.src_addr().unwrap_or(ptr::null_mut());
            0
        }
        Err(err) => match err {
            sys::CreateError::OutOfMemory => LIBNX_ERR_OUT_OF_MEMORY,
            sys::CreateError::InvalidAddress => LIBNX_ERR_BAD_INPUT,
            sys::CreateError::Svc(svc_err) => svc_err.to_rc(),
        },
    }
}

/// Creates a transfer memory object from an existing, page-aligned buffer.
///
/// Corresponds to `tmemCreateFromMemory()` in `tmem.h`.
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_tmem_create_from_memory(
    t: *mut TransferMemory,
    buf: *mut c_void,
    size: usize,
    perm: u32,
) -> u32 {
    if t.is_null() {
        return KernelError::InvalidPointer.to_rc();
    }

    match unsafe { sys::create_from_memory(buf, size, sys::Permissions::from_bits_retain(perm)) } {
        Ok(tm) => {
            let tm_ref = unsafe { &mut *t };
            tm_ref.handle = tm.handle().to_raw();
            tm_ref.size = tm.size();
            tm_ref.perm = tm.perm().bits();
            tm_ref.map_addr = ptr::null_mut();
            // Per libnx semantics we do not take ownership of the backing buffer.
            tm_ref.src_addr = ptr::null_mut();
            0
        }
        Err(err) => match err {
            sys::CreateError::OutOfMemory => LIBNX_ERR_OUT_OF_MEMORY,
            sys::CreateError::InvalidAddress => LIBNX_ERR_BAD_INPUT,
            sys::CreateError::Svc(svc_err) => svc_err.to_rc(),
        },
    }
}

/// Loads a transfer memory object received from another process (pure struct copy).
///
/// Corresponds to `tmemLoadRemote()` in `tmem.h`.
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_tmem_load_remote(
    t: *mut TransferMemory,
    handle: RawHandle,
    size: usize,
    perm: u32,
) {
    if t.is_null() {
        return;
    }

    let tm_ref = unsafe { &mut *t };
    tm_ref.handle = handle;
    tm_ref.size = size;
    tm_ref.perm = perm;
    tm_ref.map_addr = ptr::null_mut();
    tm_ref.src_addr = ptr::null_mut();
}

/// Maps a transfer memory object into the current process.
///
/// Corresponds to `tmemMap()` in `tmem.h`.
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_tmem_map(t: *mut TransferMemory) -> u32 {
    if t.is_null() {
        return KernelError::InvalidPointer.to_rc();
    }

    let tm_ref = unsafe { &mut *t };

    // Prevent double mapping.
    if !tm_ref.map_addr.is_null() {
        return LIBNX_ERR_ALREADY_MAPPED;
    }

    let src_option = NonNull::new(tm_ref.src_addr);
    let unmapped = unsafe {
        sys::TransferMemory::<sys::Unmapped>::from_parts(
            Handle::from_raw(tm_ref.handle),
            tm_ref.size,
            sys::Permissions::from_bits_retain(tm_ref.perm),
            src_option,
        )
    };

    match unsafe { sys::map(unmapped) } {
        Ok(mapped) => {
            tm_ref.map_addr = mapped.map_addr().unwrap_or(ptr::null_mut());
            0
        }
        Err(err) => match err.kind {
            sys::MapErrorKind::VirtAddressAllocFailed => LIBNX_ERR_OUT_OF_MEMORY,
            sys::MapErrorKind::Svc(svc_err) => svc_err.to_rc(),
        },
    }
}

/// Unmaps a previously mapped transfer memory object.
///
/// Corresponds to `tmemUnmap()` in `tmem.h`.
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_tmem_unmap(t: *mut TransferMemory) -> u32 {
    if t.is_null() {
        return KernelError::InvalidPointer.to_rc();
    }

    let tm_ref = unsafe { &mut *t };
    let Some(addr_nn) = NonNull::new(tm_ref.map_addr) else {
        // Nothing mapped – treat as success.
        return 0;
    };

    let src_option = NonNull::new(tm_ref.src_addr);
    let mapped = unsafe {
        sys::TransferMemory::<sys::Mapped>::from_parts(
            Handle::from_raw(tm_ref.handle),
            tm_ref.size,
            sys::Permissions::from_bits_retain(tm_ref.perm),
            src_option,
            addr_nn,
        )
    };

    match unsafe { sys::unmap(mapped) } {
        Ok(_unmapped) => {
            tm_ref.map_addr = ptr::null_mut();
            0
        }
        Err(err) => err.reason.to_rc(),
    }
}

/// Closes the handle associated with a transfer memory object.
///
/// Corresponds to `tmemCloseHandle()` in `tmem.h`.
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_tmem_close_handle(t: *mut TransferMemory) -> u32 {
    if t.is_null() {
        return KernelError::InvalidPointer.to_rc();
    }

    let tm_ref = unsafe { &mut *t };
    let src_option = NonNull::new(tm_ref.src_addr);
    let unmapped = unsafe {
        sys::TransferMemory::<sys::Unmapped>::from_parts(
            Handle::from_raw(tm_ref.handle),
            tm_ref.size,
            sys::Permissions::from_bits_retain(tm_ref.perm),
            src_option,
        )
    };

    match unsafe { sys::close_handle(unmapped) } {
        Ok(()) => {
            tm_ref.handle = INVALID_HANDLE;
            0
        }
        Err(err) => err.reason.to_rc(),
    }
}

/// Waits until the source backing memory meets the specified permission mask.
///
/// Corresponds to `tmemWaitForPermission()` in `tmem.h`.
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_tmem_wait_for_permission(t: *mut TransferMemory, perm: u32) -> u32 {
    if t.is_null() {
        return KernelError::InvalidPointer.to_rc();
    }

    let tm_ref = unsafe { &mut *t };
    let src_option = NonNull::new(tm_ref.src_addr);
    let unmapped = unsafe {
        sys::TransferMemory::<sys::Unmapped>::from_parts(
            Handle::from_raw(tm_ref.handle),
            tm_ref.size,
            sys::Permissions::from_bits_retain(tm_ref.perm),
            src_option,
        )
    };

    match unsafe { sys::wait_for_permission(unmapped, sys::Permissions::from_bits_retain(perm)) } {
        Ok(_tm) => 0,
        Err(err) => err.reason.to_rc(),
    }
}

/// Frees all resources used by a transfer memory object (unmap + close).
///
/// Corresponds to `tmemClose()` in `tmem.h`.
#[unsafe(no_mangle)]
unsafe extern "C" fn __nx_tmem_close(t: *mut TransferMemory) -> u32 {
    if t.is_null() {
        return KernelError::InvalidPointer.to_rc();
    }

    // SAFETY: We already verified pointer is non-null.
    let tm_ref = unsafe { &mut *t };

    // If mapped, unmap first.
    if !tm_ref.map_addr.is_null() {
        let rc = unsafe { __nx_tmem_unmap(t) };
        if rc != 0 {
            return rc;
        }
    }

    let src_option = NonNull::new(tm_ref.src_addr);
    let unmapped = unsafe {
        sys::TransferMemory::<sys::Unmapped>::from_parts(
            Handle::from_raw(tm_ref.handle),
            tm_ref.size,
            sys::Permissions::from_bits_retain(tm_ref.perm),
            src_option,
        )
    };

    match unsafe { sys::close(unmapped) } {
        Ok(()) => {
            tm_ref.handle = INVALID_HANDLE;
            tm_ref.src_addr = ptr::null_mut();
            0
        }
        Err(err) => err.reason.to_rc(),
    }
}
