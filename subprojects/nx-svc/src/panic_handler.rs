//! Custom panic handler for Nintendo Switch homebrew applications.
//!
//! This module provides a panic handler that calls the Switch's debug break
//! system call with a Panic reason, allowing for better debugging and error
//! reporting in homebrew applications.
//!
//! The panic handler formats messages using Rust's standard "panicked at" format
//! and passes them to `svcBreak` via a 512-byte static buffer, following the same
//! approach as libnx's `fatalThrow` and `diagAbortWithResult` functions.

use core::{fmt::Write as _, panic::PanicInfo};

use crate::debug::{self, BreakReason};

/// Maximum size for panic message buffer
const PANIC_MSG_BUF_SIZE: usize = 512;

/// Static buffer for storing panic messages
///
/// This buffer is used to store the formatted panic message so it can be
/// passed to svcBreak via a pointer. The buffer is static to ensure it
/// remains valid for the duration of the break event.
static mut PANIC_MSG_BUF: [u8; PANIC_MSG_BUF_SIZE] = [0; PANIC_MSG_BUF_SIZE];

/// Custom panic handler that calls the Switch debug break system call.
///
/// When a panic occurs, this handler will:
/// 1. Format the panic message using Rust's standard "panicked at" format
/// 2. Call `svcBreak` with `BreakReason::Panic`
/// 3. Pass the formatted message buffer address and size to svcBreak
///
/// This follows the same approach as libnx's fatalThrow and diagAbortWithResult,
/// and uses Rust's standard panic message format for consistency.
#[panic_handler]
pub fn panic_handler(info: &PanicInfo) -> ! {
    // Format the panic message using Rust's standard Display implementation
    // This gives us the standard "panicked at '<message>', <file>:<line>:<column>" format

    // SAFETY: Taking a raw pointer to static mut and creating a slice from it is safe.
    // The pointer is valid, properly aligned, and we have exclusive access during panic.
    let (buf_slice, buf_ptr) = unsafe {
        let raw_ptr = &raw mut PANIC_MSG_BUF;
        let slice = core::slice::from_raw_parts_mut(raw_ptr as *mut u8, PANIC_MSG_BUF_SIZE);
        (slice, raw_ptr)
    };

    // Create a cursor to write into the buffer
    let mut cursor = Cursor::new(buf_slice);

    // Write the panic info using Rust's standard Display format
    // This automatically handles the "panicked at" formatting
    let _ = write!(cursor, "{}", info);

    let written = cursor.position();
    let (msg_ptr, msg_len) = (buf_ptr as usize, written);

    // Call the debug break system call with panic reason.
    // Pass the panic message buffer address and size, following the same
    // pattern as libnx's `fatalThrow` and `diagAbortWithResult` functions.
    debug::break_event(BreakReason::Panic, msg_ptr, msg_len);
}

/// A cursor implementation for writing to a byte buffer in no_std environments.
///
/// Wraps a mutable byte slice and tracks the current write position.
/// Provides `Write` trait implementation for formatting operations.
struct Cursor<'a> {
    buf: &'a mut [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    /// Creates a new cursor wrapping the provided buffer.
    fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    /// Returns the current write position in the buffer.
    fn position(&self) -> usize {
        self.pos
    }
}

impl<'a> core::fmt::Write for Cursor<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let remaining = self.buf.len().saturating_sub(self.pos);
        let to_write = bytes.len().min(remaining);

        if to_write > 0 {
            self.buf[self.pos..self.pos + to_write].copy_from_slice(&bytes[..to_write]);
            self.pos += to_write;
        }

        Ok(())
    }
}
