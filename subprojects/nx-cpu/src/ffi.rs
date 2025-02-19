//! FFI bindings for the `nx-cpu` crate
//!
//! # References
//!
//! - [switchbrew/libnx: switch/arm/tls.h](https://github.com/switchbrew/libnx/blob/master/nx/include/switch/arm/tls.h)

use core::ffi::c_void;

use crate::tls;

//<editor-fold desc="switch/arm/tls.h">

/// Gets the thread-local storage (TLS) buffer.
///
/// This function reads the `tpidrro_el0` system register, which holds the
/// read-only thread pointer for the current thread.
///
/// Returns a pointer to the thread-local storage buffer.
#[inline]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_cpu_get_tls() -> *mut c_void {
    tls::get_ptr()
}

//</editor-fold>
