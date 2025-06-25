//! Functions to read and write control registers
//!
//! This module provides functions for interacting with the CPU control registers.

use core::arch::asm;

/// Read the `cntpct_el0` system register.
///
/// This function reads the `cntpct_el0` system register, which holds the current value of the
/// CPU counter-timer.
///
/// Returns the current system tick as a `u64`.
///
/// # Counter-timer Physical Count register - EL0
///
/// Holds the 64-bit physical count value.
///
/// # References
///
/// - [ARM CNTPCT-EL0 Register](https://developer.arm.com/documentation/ddi0601/2024-12/AArch64-Registers/CNTPCT-EL0--Counter-timer-Physical-Count-Register)
/// - [rust-embedded/aarch64-cpu: cntpct_el0](https://github.com/rust-embedded/aarch64-cpu/blob/f8bf731f0d0bda084302f04adb5b3a0a2c448d9e/src/registers/cntpct_el0.rs)
#[inline]
pub unsafe fn cntpct_el0() -> u64 {
    let value: u64;
    // SAFETY: Assembly only loads the system counter-timer value
    unsafe {
        asm!(
            "mrs {:x}, cntpct_el0", // Move from system register to general-purpose register
            out(reg) value,         // Output: Capture the value of the `cntpct_el0` register
            options(nostack, nomem, preserves_flags)
        );
    }
    value
}

/// Read the `cntfrq_el0` system register.
///
/// This function reads the `cntfrq_el0` system register, which holds the
/// frequency of the system counter-timer.
///
/// Returns the system counter-timer frequency, in Hz.
///
/// # Counter-timer Frequency register - EL0
///
/// This register is provided so that software can discover the frequency of the system counter.
/// It must be programmed with this value as part of system initialization. The value of the
/// register is not interpreted by hardware.
///
/// # References
///
/// - [ARM CNTFRQ-EL0 Register](https://developer.arm.com/documentation/ddi0601/2020-12/AArch64-Registers/CNTFRQ-EL0--Counter-timer-Frequency-register)
/// - [rust-embedded/aarch64-cpu: cntfrq_el0.rs](https://github.com/rust-embedded/aarch64-cpu/blob/f8bf731f0d0bda084302f04adb5b3a0a2c448d9e/src/registers/cntfrq_el0.rs)
#[inline]
pub unsafe fn cntfrq_el0() -> u64 {
    let value: u64;
    // SAFETY: Assembly only loads the system counter-timer frequency value
    unsafe {
        asm!(
            "mrs {:x}, cntfrq_el0", // Move from system register to general-purpose register
            out(reg) value,         // Output: Capture the value of the `cntfrq_el0` register
            options(nostack, nomem, preserves_flags)
        );
    }
    value
}

/// Read the `tpidrro_el0` system register.
///
/// This function reads the `tpidrro_el0` system register, which holds the read-only thread pointer
/// for the current thread.
///
/// Returns the base address of the Thread-Local Storage (TLS) buffer.
///
/// # References
///
/// - [ARM TPIDRRO_ELO Register](https://developer.arm.com/documentation/ddi0601/2024-12/AArch64-Registers/TPIDRRO-EL0--EL0-Read-Only-Software-Thread-ID-Register)
/// - [rust-embedded/aarch64-cpu: tpidrro_el0.rs](https://github.com/rust-embedded/aarch64-cpu/blob/main/src/registers/tpidrro_el0.rs)
#[inline]
pub unsafe fn tpidrro_el0() -> usize {
    let tls_ptr: usize;
    // SAFETY: Assembly only loads the system register value
    unsafe {
        asm!(
        "mrs {:x}, tpidrro_el0", // Move the value of tpidrro_el0 into tls_ptr
        out(reg) tls_ptr,        // Output: tls_ptr will hold the value of tpidrro_el0
        options(nostack, nomem, preserves_flags)
        );
    }
    tls_ptr
}
