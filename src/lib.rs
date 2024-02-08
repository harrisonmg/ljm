#![doc = include_str!("../README.md")]

extern crate ljm_sys as ffi;

mod constants;
pub use constants::*;

mod error;
pub use error::*;

mod identifier;
pub use identifier::*;

mod handle;
pub use handle::*;

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::*;

    #[test]
    fn sanity() -> anyhow::Result<()> {
        let handle = Handle::open(DeviceType::Any, ConnectionType::Any, Identifier::DemoMode)?;
        handle.write_name(CString::new("TEST_FLOAT32")?, 3.14)?;
        handle.read_name(CString::new("TEST_FLOAT32")?)?;
        handle.close()?;
        Ok(())
    }
}
