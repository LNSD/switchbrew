//! FFI bindings for the `nx-sys-thread` crate

mod newlib_nanosleep;
mod newlib_pthread;
mod newlib_sleep;
mod newlib_usleep;
mod slots;
mod thread_activity;
mod thread_context;
mod thread_info;
mod thread_wait;
mod tls;
