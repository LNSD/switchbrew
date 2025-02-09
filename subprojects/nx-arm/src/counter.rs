use core::arch::asm;

/// Gets the current system tick.
///
/// This function reads the `cntpct_el0` system register, which holds the
/// current value of the system counter.
///
/// # Returns
///
/// The current system tick as a `u64`.
#[inline]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nx_arm_get_system_tick() -> u64 {
    let result: u64;
    unsafe {
        asm!(
            "mrs {}, cntpct_el0", // Move from system register to general-purpose register
            out(reg) result       // Output: Capture the value of the `cntpct_el0` register
        );
    }
    result
}

/// Gets the system counter-timer frequency.
///
/// This function reads the `cntfrq_el0` system register, which holds the
/// frequency of the system counter-timer.
///
/// # Returns
///
/// The system counter-timer frequency, in Hz.
#[inline]
#[unsafe(no_mangle)]
pub fn __nx_arm_get_system_tick_freq() -> u64 {
    let result: u64;
    unsafe {
        asm!(
            "mrs {}, cntfrq_el0", // Move from system register to general-purpose register
            out(reg) result       // Output: Capture the value of the `cntfrq_el0` register
        );
    }
    result
}

/// Converts time from nanoseconds to CPU ticks.
///
/// This function calculates the equivalent CPU ticks for a given time in nanoseconds,
/// based on the system counter frequency.
///
/// # Returns
///
/// Time in CPU ticks.
#[inline]
#[unsafe(no_mangle)]
pub fn __nx_arm_ns_to_ticks(ns: u64) -> u64 {
    (ns * 12) / 625
}

/// Converts from CPU ticks to nanoseconds.
///
/// This function calculates the equivalent time in nanoseconds for a given number of CPU ticks.
///
/// # Returns
///
/// Time in nanoseconds.
#[inline]
#[unsafe(no_mangle)]
pub fn __nx_arm_ticks_to_ns(tick: u64) -> u64 {
    (tick * 625) / 12
}
