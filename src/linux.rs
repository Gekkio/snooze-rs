use libc::{EINTR, EINVAL, CLOCK_MONOTONIC, c_int, timespec};
use std::mem;
use std::os;
use std::ptr::null_mut;
use std::time::duration::Duration;

use super::{SnoozeError, SnoozeResult};

mod ffi {
  use libc::{c_int, timespec};

  pub const TIMER_ABSTIME: c_int = 1;

  extern "C" {
    pub fn clock_gettime(clock: c_int, tp: *mut timespec) -> c_int;
    pub fn clock_nanosleep(clock: c_int, flags: c_int, req: *const timespec, rem: *mut timespec) -> c_int;
  }
}

fn clock_gettime() -> SnoozeResult<timespec> {
  let mut tp: timespec = unsafe { mem::uninitialized() };
  let ret = unsafe {
    ffi::clock_gettime(CLOCK_MONOTONIC, &mut tp)
  };
  if ret != 0 {
    Err(match os::errno() as c_int {
      EINVAL => SnoozeError::Unsupported("CLOCK_MONOTONIC is not supported".to_string()),
      error => SnoozeError::from_errno(error as uint)
    })
  } else { Ok(tp) }
}

fn clock_nanosleep(time: &timespec) -> SnoozeResult<()> {
  while unsafe {
    ffi::clock_nanosleep(CLOCK_MONOTONIC, ffi::TIMER_ABSTIME, time, null_mut())
  } != 0 {
    match os::errno() as c_int {
      EINTR => (),
      error => return Err(SnoozeError::from_errno(error as uint))
    }
  }
  Ok(())
}

#[allow(missing_copy_implementations)]
pub struct Snooze {
  duration_secs: i64,
  duration_nanos: i64,
  last_time: timespec
}

impl Snooze {
  pub fn new(duration: Duration) -> SnoozeResult<Snooze> {
    // TODO: Figure out if unwrap() is safe or not
    let duration_secs = duration.num_seconds();
    let duration_nanos = (duration - Duration::seconds(duration_secs)).num_nanoseconds().unwrap();
    Ok(Snooze {
      duration_secs: duration_secs,
      duration_nanos: duration_nanos,
      last_time: try!(clock_gettime())
    })
  }
  pub fn reset(&mut self) -> SnoozeResult<()> {
    self.last_time = try!(clock_gettime());
    Ok(())
  }
  pub fn wait(&mut self) -> SnoozeResult<()> {
    let mut seconds =
      self.last_time.tv_sec + self.duration_secs;
    let mut nanos =
      self.last_time.tv_nsec + self.duration_nanos;

    const NANOS_IN_SECOND: i64 = 1000000000;
    if nanos >= NANOS_IN_SECOND {
      seconds += 1;
      nanos -= NANOS_IN_SECOND;
    }

    let target_time = timespec {
      tv_sec: seconds,
      tv_nsec: nanos
    };
    try!(clock_nanosleep(&target_time));
    self.last_time = target_time;
    Ok(())
  }
}
