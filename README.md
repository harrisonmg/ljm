# ljm
An idiomatic Rust wrapper for the [LabJack LJM library](https://labjack.com/pages/support?doc=%2Fsoftware-driver%2Fljm-users-guide).

This library is far from complete and functionality is added as needed. Please feel free to submit PRs expanding support for the LJM library.

## Usage Requirements
* At least the [minimal version of LJM](https://labjack.com/pages/support/?doc=/software-driver/installer-downloads/minimal-ljm-installers/#section-header-two-haxma) is installed

## Example
```rust
use std::ffi::CString;

fn main() -> Result<(), ljm::Error> {

  let handle = ljm::Handle::open(
    ljm::DeviceType::Any,
    ljm::ConnectionType::Any,
    ljm::Identifier::DemoMode
  )?;

  let name = CString::new("TEST_FLOAT32").unwrap();

  handle.write_name(&name, 3.14)?;

  let value = handle.read_name(&name)?;

  handle.close()
}
```

## Alternative Crates
- [ljmrs](https://crates.io/crates/ljmrs) offers an API through libloading that more closely mirrors the C library. The goal of this crate is to be somewhat more idiomatic and ergonomic, such as providing a [Handle](crate::Handle) type that attempts to use RAII for device handles.
