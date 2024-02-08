# Usage Requirements
* At least the [minimal version of LJM](https://labjack.com/pages/support/?doc=/software-driver/installer-downloads/minimal-ljm-installers/#section-header-two-haxma) is installed

# Example
```rust
use std::ffi::CString;

fn main() -> Result<(), ljm::Error> {

  let handle = ljm::Handle::open(
    ljm::DeviceType::Any,
    ljm::ConnectionType::Any,
    ljm::Identifier::DemoMode
  )?;

  handle.write_name(CString::new("TEST_FLOAT32").unwrap(), 3.14)?;

  let float_value = handle.read_name(CString::new("TEST_FLOAT32").unwrap())?;

  handle.close()
}
```
