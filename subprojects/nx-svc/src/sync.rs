use super::raw::{
    __nx_svc_arbitrate_lock, __nx_svc_arbitrate_unlock, __nx_svc_signal_process_wide_key,
    __nx_svc_wait_process_wide_key_atomic,
};

// TODO: Document. Mutex related
pub const HANDLE_WAIT_MASK: u32 = 0x40000000;

/// Makes the calling thread wait until a specific memory address, `tag_location`, no longer
/// contains a given value, `self_tag`.
// TODO: Improve documentation
pub unsafe fn arbitrate_lock(
    wait_tag: u32,
    tag_location: *mut u32,
    self_tag: u32,
) -> Result<(), ()> {
    let res = unsafe { __nx_svc_arbitrate_lock(wait_tag, tag_location, self_tag) };
    if res == 0 { Ok(()) } else { Err(()) }
}

// TODO: Document. Mutex related
pub unsafe fn arbitrate_unlock(tag_location: *mut u32) -> Result<(), ()> {
    let res = unsafe { __nx_svc_arbitrate_unlock(tag_location) };
    if res == 0 { Ok(()) } else { Err(()) }
}

// TODO: Document. Condvar related
pub unsafe fn wait_process_wide_key_atomic(
    condvar: *mut u32,
    mutex: *mut u32,
    tag: u32,
    timeout_ns: u64,
) -> Result<(), u32> {
    let res = unsafe { __nx_svc_wait_process_wide_key_atomic(mutex, condvar, tag, timeout_ns) };
    if res == 0 { Ok(()) } else { Err(res) }
}

// TODO: Document. Condvar related
pub unsafe fn signal_process_wide_key(cv_key: *mut u32, count: i32) {
    unsafe { __nx_svc_signal_process_wide_key(cv_key, count) };
}
