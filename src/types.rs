use std::{
    ffi::{c_char, CString, NulError},
    os::raw::c_int,
};

/// Some ID that may identify a LabJack device to be connected.
///
/// See it in the [LJM User Guide](https://support.labjack.com/docs/identifier-parameter-ljm-user-s-guide).
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
    #[cfg(not(tarpaulin_include))]
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

/// An LJM data type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Uint16,
    Uint32,
    Int32,
    Float32,
    String,
}

impl From<DataType> for c_int {
    #[cfg(not(tarpaulin_include))]
    fn from(value: DataType) -> Self {
        match value {
            DataType::Uint16 => ffi::LJM_UINT16,
            DataType::Uint32 => ffi::LJM_UINT32,
            DataType::Int32 => ffi::LJM_INT32,
            DataType::Float32 => ffi::LJM_FLOAT32,
            DataType::String => ffi::LJM_STRING,
        }
    }
}

impl TryFrom<c_int> for DataType {
    type Error = &'static str;

    #[cfg(not(tarpaulin_include))]
    fn try_from(value: c_int) -> Result<Self, Self::Error> {
        Ok(match value {
            ffi::LJM_UINT16 => Self::Uint16,
            ffi::LJM_UINT32 => Self::Uint32,
            ffi::LJM_INT32 => Self::Int32,
            ffi::LJM_FLOAT32 => Self::Float32,
            ffi::LJM_STRING => Self::String,
            _ => return Err("Invalid LJM data type"),
        })
    }
}

/// A Modbus register specified by address and data type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Register {
    pub(crate) address: c_int,
    pub data_type: DataType,
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
