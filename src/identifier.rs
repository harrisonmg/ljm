use std::ffi::{CString, NulError};

/// Some ID that may identify a LabJack device to be connected.
///
/// See it in the [LJM User Guide](https://labjack.com/pages/support?doc=%2Fsoftware-driver%2Fljm-users-guide%2Fidentifier-parameter%2F).
#[derive(Debug, Clone)]
pub enum Identifier {
    DemoMode,
    Any,
    String(CString),
}

impl Identifier {
    pub fn new(id: &str) -> Result<Self, NulError> {
        Ok(Identifier::String(CString::new(id)?))
    }
}

impl From<Identifier> for CString {
    fn from(value: Identifier) -> Self {
        match value {
            Identifier::DemoMode => CString::new("-2").unwrap(),
            Identifier::Any => CString::new("ANY").unwrap(),
            Identifier::String(c_str) => c_str,
        }
    }
}
