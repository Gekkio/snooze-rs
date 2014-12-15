snooze-rs
=========

*Experimental* library for sleeping periodically in Rust code.

[![Build Status](https://travis-ci.org/Gekkio/snooze-rs.svg?branch=master)](https://travis-ci.org/Gekkio/snooze-rs)

## Usage:

Cargo.toml:

```toml
[dependencies.snooze-rs]
git = "https://github.com/Gekkio/snooze-rs.git"
```

Crate root:

```rust
extern crate snooze;
```

Basic usage:

```rust
use snooze::Snooze;

let mut snooze = try!(Snooze::new(Duration::milliseconds(42)));
loop {
  try!(snooze.wait());
  do_things();
}
```

The function `do_things()` will be called approximately every 42 ms, depending on
system timer accuracy and assuming do_things() takes less than 42 ms.
