use std::ffi::{c_char, CString, NulError};

/// Some ID that may identify a LabJack device to be connected.
///
/// See it in the [LJM User Guide](https://labjack.com/pages/support?doc=%2Fsoftware-driver%2Fljm-users-guide%2Fidentifier-parameter%2F).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Identifier {
    DemoMode,
    Any,
    String(CString),
}

impl Identifier {
    pub fn new(str: impl Into<Vec<u8>>) -> Result<Self, NulError> {
        Ok(Identifier::String(CString::new(str)?))
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

/// An error indicating a failure to construct an [LjmString]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LjmStringError {
    TooLong,
    Nul(NulError),
}

impl std::fmt::Display for LjmStringError {
    #[cfg(not(tarpaulin_include))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl std::error::Error for LjmStringError {}

/// A [CString] that may be no longer than [ljm_sys::LJM_STRING_MAX_SIZE],
/// not including null-termination.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LjmString(CString);

impl LjmString {
    pub fn new(str: impl Into<Vec<u8>>) -> Result<Self, LjmStringError> {
        let vec: Vec<u8> = str.into();
        if vec.len() > ljm_sys::LJM_STRING_MAX_SIZE as usize {
            return Err(LjmStringError::TooLong);
        }
        Ok(Self(CString::new(vec).map_err(LjmStringError::Nul)?))
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.0.as_ptr()
    }
}

impl From<LjmString> for CString {
    fn from(value: LjmString) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifier() -> anyhow::Result<()> {
        let string = "test_id";
        let id = Identifier::new(string)?;
        assert_eq!(CString::new(string)?, id.into());
        Ok(())
    }

    #[test]
    fn ljm_string() -> anyhow::Result<()> {
        let short_enough = "1234567890123456789012345678901234567890123456789";
        let short_c = CString::new(short_enough)?;
        let short_ljm = LjmString::new(short_enough)?;
        assert_eq!(short_c, short_ljm.into());

        let too_long = "12345678901234567890123456789012345678901234567890";
        assert!(matches!(
            LjmString::new(too_long),
            Err(LjmStringError::TooLong)
        ));

        Ok(())
    }
}
