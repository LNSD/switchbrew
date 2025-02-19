//! Counter-timer registry
//!
//! This module provides functions for interacting with the CPU counter-timer registries.

use crate::control_regs;

/// Gets the current system tick.
///
/// This function reads the `cntpct_el0` system register, which holds the current value of the
/// CPU counter-timer.
#[inline]
pub fn get_system_tick() -> u64 {
    unsafe { control_regs::cntpct_el0() }
}

/// Gets the system counter-timer frequency.
///
/// This function reads the `cntfrq_el0` system register, which holds the
/// frequency of the system counter-timer.
///
/// Returns the system counter-timer frequency, in Hz.
#[inline]
pub fn get_system_tick_freq() -> u64 {
    unsafe { control_regs::cntfrq_el0() }
}

/// Converts time from nanoseconds to CPU ticks.
///
/// Returns the equivalent CPU ticks for a given time in nanoseconds, based on the
/// system counter frequency.
#[inline]
pub fn ns_to_cpu_ticks(ns: u64) -> u64 {
    (ns * 12) / 625
}

/// Converts from CPU ticks to nanoseconds.
///
/// Returns the equivalent time in nanoseconds for a given number of CPU ticks.
#[inline]
pub fn cpu_ticks_to_ns(tick: u64) -> u64 {
    (tick * 625) / 12
}
