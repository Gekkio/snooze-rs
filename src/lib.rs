#![feature(libc, std_misc)]

#![cfg_attr(test, feature(core, test))]

#[cfg(any(target_os = "android",
          target_os = "ios",
          target_os = "linux",
          target_os = "macos"))]
extern crate nix;
extern crate libc;
#[cfg(test)]
extern crate test;

#[cfg(any(target_os = "linux", target_os = "android"))]
use self::linux as os_specific;
#[cfg(any(target_os = "macos", target_os = "ios"))]
use self::mach as os_specific;
#[cfg(target_os = "windows")]
use self::windows as os_specific;

use nix::errno::Errno;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::time::Duration;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod linux;
#[cfg(any(target_os = "macos", target_os = "ios"))]
mod mach;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum SnoozeError {
  Unsupported(String),
  Other(Errno)
}

#[allow(dead_code)]
impl SnoozeError {
  fn from_last_os_error() -> SnoozeError {
    SnoozeError::Other(Errno::last())
  }
  fn from_errno(error: Errno) -> SnoozeError {
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
}

impl Display for SnoozeError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      SnoozeError::Unsupported(ref msg) => f.write_str(msg),
      SnoozeError::Other(error) => f.write_str(error.desc())
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
  /// Puts the current thread to sleep until the next wake-up time
  pub fn wait(&mut self) -> SnoozeResult<()> { self.0.wait() }
}

