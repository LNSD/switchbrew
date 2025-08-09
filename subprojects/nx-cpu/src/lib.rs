//! # nx-cpu
//!
//! A Rust library for interacting with the Nintendo Switch's ARM Cortex-A57 (aarch64) CPU.

#![no_std]

#[cfg(not(target_arch = "aarch64"))]
compile_error!("nx-cpu only supports aarch64 CPUs");

// Import nx-svc to ensure panic handler is linked
#[allow(unused_imports)]
use nx_svc as _;

pub mod barrier;
pub mod control_regs;
