snooze-rs
=========

*Experimental* library for sleeping periodically in Rust code.

[![Build Status](https://travis-ci.org/Gekkio/snooze-rs.svg?branch=master)](https://travis-ci.org/Gekkio/snooze-rs)

## Usage:

Cargo.toml:

```toml
[dependencies]
snooze-rs = "0.0.3"
```

Crate root:

```rust
extern crate snooze;
extern crate time;
```

Basic usage:

```rust
use snooze::{Snooze, SnoozeError};
use time::duration::Duration;

fn sleep_and_work() -> Result<(), SnoozeError> {
  let mut snooze = try!(Snooze::new(Duration::milliseconds(42)));
  while should_continue() {
    try!(snooze.wait());
    do_things();
  }
  Ok(())
}
```

The function `do_things()` will be called approximately every 42 ms, depending on
system timer accuracy and assuming do_things() takes less than 42 ms.
