extern crate libc;

#[cfg(target_os = "linux")]
use self::linux as os_specific;
#[cfg(any(target_os = "macos", target_os = "ios"))]
use self::mach as os_specific;
#[cfg(target_os = "windows")]
use self::windows as os_specific;

use std::error::Error;
use std::os::{errno, error_string};
use std::time::Duration;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(any(target_os = "macos", target_os = "ios"))]
mod mach;
#[cfg(target_os = "windows")]
mod windows;

#[deriving(Show)]
pub enum SnoozeError {
  Unsupported(String),
  Other(uint)
}

#[allow(dead_code)]
impl SnoozeError {
  fn from_last_os_error() -> SnoozeError {
    SnoozeError::Other(errno())
  }
  fn from_errno(error: uint) -> SnoozeError {
    SnoozeError::Other(error)
  }
}

impl Error for SnoozeError {
  fn description(&self) -> &str {
    match *self {
      SnoozeError::Unsupported(..) => "Unsupported system",
      SnoozeError::Other(..) => "System error"
    }
  }
  fn detail(&self) -> Option<String> {
    match *self {
      SnoozeError::Unsupported(ref msg) => Some(msg.clone()),
      SnoozeError::Other(error) => Some(error_string(error))
    }
  }
}

pub type SnoozeResult<T> = Result<T, SnoozeError>;

#[allow(missing_copy_implementations)]
pub struct Snooze(os_specific::Snooze);

impl Snooze {
  pub fn new(duration: Duration) -> SnoozeResult<Snooze> {
    Ok(Snooze(try!(os_specific::Snooze::new(duration))))
  }
  pub fn reset(&mut self) -> SnoozeResult<()> { self.0.reset() }
  pub fn wait(&mut self) -> SnoozeResult<()> { self.0.wait() }
}
