//! FFI bindings for the `nx-cpu` crate
//!
//! # References
//!
//! - [switchbrew/libnx: switch/arm/counter.h](https://github.com/switchbrew/libnx/blob/60bf943ec14b1fb2ae169e627e64ab93a24c042b/nx/include/switch/arm/counter.h)
//! - [switchbrew/libnx: switch/arm/tls.h](https://github.com/switchbrew/libnx/blob/master/nx/include/switch/arm/tls.h)

use core::ffi::c_void;

use crate::{counter, tls};

//<editor-fold desc="switch/arm/counter.h">

/// Gets the current system tick.
///
/// This function reads the `cntpct_el0` system register, which holds the current value of the
/// CPU counter-timer.
///
/// Returns the current system tick as a `u64`.
#[inline]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_cpu_get_system_tick() -> u64 {
    counter::get_system_tick()
}

/// Gets the system counter-timer frequency.
///
/// This function reads the `cntfrq_el0` system register, which holds the
/// frequency of the system counter-timer.
///
/// Returns the system counter-timer frequency, in Hz.
#[inline]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_cpu_get_system_tick_freq() -> u64 {
    counter::get_system_tick_freq()
}

/// Converts time from nanoseconds to CPU ticks.
///
/// Returns the equivalent CPU ticks for a given time in nanoseconds, based on the
/// system counter frequency.
#[inline]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_cpu_ns_to_ticks(ns: u64) -> u64 {
    counter::ns_to_cpu_ticks(ns)
}

/// Converts from CPU ticks to nanoseconds.
///
/// Returns the equivalent time in nanoseconds for a given number of CPU ticks.
#[inline]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_cpu_ticks_to_ns(tick: u64) -> u64 {
    counter::cpu_ticks_to_ns(tick)
}

//</editor-fold>

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
