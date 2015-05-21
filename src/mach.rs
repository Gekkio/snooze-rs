use libc::mach_timebase_info;
use std::mem;
use time::Duration;

use super::{SnoozeError, SnoozeResult};

type AbsTime = u64;

mod ffi {
    use libc::{c_int, mach_timebase_info};

    #[allow(non_camel_case_types)]
    pub type kern_return_t = c_int;
    pub const KERN_SUCCESS: kern_return_t = 0;

    extern "C" {
        pub fn mach_absolute_time() -> u64;
        pub fn mach_timebase_info(info: *mut mach_timebase_info) -> kern_return_t;
        pub fn mach_wait_until(deadline: u64) -> kern_return_t;
    }
}

fn absolute_time() -> AbsTime {
    unsafe { ffi::mach_absolute_time() }
}

fn convert_nanos(nanos: u64) -> SnoozeResult<AbsTime> {
    let mut info: mach_timebase_info = unsafe { mem::uninitialized() };
    let ret = unsafe {
        ffi::mach_timebase_info(&mut info)
    };
    if ret != ffi::KERN_SUCCESS {
        Err(SnoozeError::from_last_os_error())
    } else { Ok(nanos * info.numer as u64 / info.denom as u64) }
}

fn wait_until(time: AbsTime) -> SnoozeResult<()> {
    let ret = unsafe { ffi::mach_wait_until(time) };
    if ret != ffi::KERN_SUCCESS {
        Err(SnoozeError::from_last_os_error())
    } else { Ok(()) }
}

#[allow(missing_copy_implementations)]
pub struct Snooze {
    duration: AbsTime,
    last_time: AbsTime
}

impl Snooze {
    pub fn new(duration: Duration) -> SnoozeResult<Snooze> {
        // TODO: Check for unwrap() and u64 cast overflow
        Ok(Snooze {
            duration: try!(convert_nanos(duration.num_nanoseconds().unwrap() as u64)),
            last_time: absolute_time()
        })
    }
    pub fn reset(&mut self) -> SnoozeResult<()> {
        self.last_time = absolute_time();
        Ok(())
    }
    pub fn wait(&mut self) -> SnoozeResult<()> {
        let target_time = self.last_time + self.duration;
        try!(wait_until(target_time));
        self.last_time = target_time;
        Ok(())
    }
}
