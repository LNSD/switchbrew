use crate::sys::timespec::Timespec;

// TODO: Add support for retrieving the current time from the RTC service
pub fn gettime() -> Result<Timespec, i32> {
    todo!()
}
