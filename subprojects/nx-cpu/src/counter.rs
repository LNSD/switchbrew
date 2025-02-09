//! CPU counter-timer functions.
//!
//! This module provides functions for interacting with the CPU counter-timer registries.
//!
//! ## References
//! - <https://github.com/switchbrew/libnx/blob/60bf943ec14b1fb2ae169e627e64ab93a24c042b/nx/include/switch/arm/counter.h>

use core::arch::asm;

/// Gets the current system tick.
///
/// This function reads the `cntpct_el0` system register, which holds the current value of the
/// CPU counter-timer.
///
/// Returns the current system tick as a `u64`.
///
/// ## References
/// - <https://developer.arm.com/documentation/ddi0601/2024-12/AArch64-Registers/CNTPCT-EL0--Counter-timer-Physical-Count-Register>
#[inline]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_cpu_get_system_tick() -> u64 {
    let result: u64;
    unsafe {
        asm!(
            "mrs {}, cntpct_el0",   // Move from system register to general-purpose register
            out(reg) result,        // Output: Capture the value of the `cntpct_el0` register
            options(nostack, nomem) // No stack or memory operations
        );
    }
    result
}

/// Gets the system counter-timer frequency.
///
/// This function reads the `cntfrq_el0` system register, which holds the
/// frequency of the system counter-timer.
///
/// Returns the system counter-timer frequency, in Hz.
///
/// ## References
/// - <https://developer.arm.com/documentation/ddi0601/2020-12/AArch64-Registers/CNTFRQ-EL0--Counter-timer-Frequency-register>
#[inline]
#[unsafe(no_mangle)]
pub fn __nx_cpu_get_system_tick_freq() -> u64 {
    let result: u64;
    unsafe {
        asm!(
            "mrs {}, cntfrq_el0",   // Move from system register to general-purpose register
            out(reg) result,        // Output: Capture the value of the `cntfrq_el0` register
            options(nostack, nomem) // No stack or memory operations
        );
    }
    result
}

/// Converts time from nanoseconds to CPU ticks.
///
/// Returns the equivalent CPU ticks for a given time in nanoseconds, based on the
/// system counter frequency.
#[inline]
#[unsafe(no_mangle)]
pub fn __nx_cpu_ns_to_ticks(ns: u64) -> u64 {
    (ns * 12) / 625
}

/// Converts from CPU ticks to nanoseconds.
///
/// Returns the equivalent time in nanoseconds for a given number of CPU ticks.
#[inline]
#[unsafe(no_mangle)]
pub fn __nx_cpu_ticks_to_ns(tick: u64) -> u64 {
    (tick * 625) / 12
}
