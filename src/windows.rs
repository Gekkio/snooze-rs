use libc::{
  FALSE, INFINITE, HANDLE, FILETIME, WAIT_OBJECT_0,
  CloseHandle, GetSystemTimeAsFileTime, WaitForSingleObject
};
use std::io;
use std::mem;
use std::ptr::{null, null_mut};
use std::time::duration::Duration;

use super::{SnoozeError, SnoozeResult};

type Nanos100 = i64;

mod ffi {
  use libc::{BOOL, HANDLE, LARGE_INTEGER, LONG, LPSECURITY_ATTRIBUTES, LPVOID};
  use libc::types::os::arch::extra::{LPCWSTR};

  #[allow(non_camel_case_types)]
  pub type LPCTSTR = LPCWSTR;

  extern "stdcall" {
    #[allow(non_snake_case_functions)]
    pub fn CreateWaitableTimerW(lpTimerAttributes: LPSECURITY_ATTRIBUTES,
                                bManualReset: BOOL,
                                lpTimerName: LPCTSTR) -> HANDLE;

    #[allow(non_snake_case_functions)]
    pub fn SetWaitableTimer(hTimer: HANDLE,
                            pDueTime: *const LARGE_INTEGER,
                            lPeriod: LONG,
                            pfnCompletionRoutine: LPVOID,
                            lpArgToCompletionRoutine: LPVOID,
                            fResume: BOOL) -> BOOL;
  }
}

fn create_waitable_timer() -> SnoozeResult<HANDLE> {
  let result = unsafe {
    ffi::CreateWaitableTimerW(null_mut(), FALSE, null())
  };
  if result.is_null() { Err(SnoozeError::from_last_os_error()) }
  else { Ok(result) }
}

fn set_waitable_timer(timer: HANDLE, due_time: Nanos100) -> SnoozeResult<()> {
  let ret = unsafe {
    ffi::SetWaitableTimer(timer, &due_time, 0, null_mut(), null_mut(), FALSE)
  };
  if ret == 0 { Err(SnoozeError::from_last_os_error()) }
  else { Ok(()) }
}

fn get_system_time() -> Nanos100 {
  let mut time: FILETIME = unsafe { mem::uninitialized() };
  unsafe {
    GetSystemTimeAsFileTime(&mut time);
  }
  (((time.dwHighDateTime as u64) << 32) | time.dwLowDateTime as u64) as Nanos100
}

fn wait_for_single_object(handle: HANDLE) -> SnoozeResult<()> {
  let result = unsafe {
    WaitForSingleObject(handle, INFINITE)
  };
  if result != WAIT_OBJECT_0 { Err(SnoozeError::from_last_os_error()) }
  else { Ok(()) }
}

pub struct Snooze {
  duration: Nanos100,
  last_time: Nanos100,
  timer_handle: HANDLE
}

impl Drop for Snooze {
  fn drop(&mut self) {
    if unsafe { CloseHandle(self.timer_handle) } == 0 {
      // TODO: Figure out if panic! in drop() is in any way acceptable
      panic!("CloseHandle failed: {}", io::Error::last_os_error());
    }
  }
}

impl Snooze {
  pub fn new(duration: Duration) -> SnoozeResult<Snooze> {
    // TODO: Figure out if unwrap() is safe or not
    let duration = duration.num_nanoseconds().unwrap() / 100;
    Ok(Snooze {
      duration: duration,
      timer_handle: try!(create_waitable_timer()),
      last_time: get_system_time()
    })
  }
  pub fn reset(&mut self) -> SnoozeResult<()> {
    self.last_time = get_system_time();
    Ok(())
  }
  pub fn wait(&mut self) -> SnoozeResult<()> {
    let target_time = self.last_time + self.duration;
    self.last_time = target_time;
    try!(set_waitable_timer(self.timer_handle, target_time));
    wait_for_single_object(self.timer_handle)
  }
}
