use std::ffi::{c_int, CStr, CString};

use crate::{check_ret, ConnectionType, DeviceType, Error, Identifier, LjmString, Register};

/// A LabJack device handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handle(pub(crate) c_int);

impl Handle {
    /// Opens a desired LabJack and associates a device handle number (connection ID).
    ///
    /// See it in the [LJM User Guide](https://support.labjack.com/docs/open-ljm-user-s-guide).
    ///
    /// If you attempt to open the same device multiple times, LJM will return the same handle.
    /// This means that you can have multiple duplicate [Handle]s, and could close one while the
    /// others are still in scope, resulting in errors when making calls to the LJM library. For
    /// now, it is the responsibility of the caller to ensure that duplicate [Handle]s are avoided
    /// or handled appropriately.
    pub fn open(
        device_type: DeviceType,
        connection_type: ConnectionType,
        identifier: Identifier,
    ) -> Result<Self, Error> {
        let mut raw_handle: c_int = 0;
        unsafe {
            let ret = ffi::LJM_Open(
                device_type.into(),
                connection_type.into(),
                CString::from(identifier).as_ptr(),
                &mut raw_handle,
            );
            check_ret(ret, Handle(raw_handle))
        }
    }

    fn raw_close(&self) -> Result<(), Error> {
        unsafe {
            let ret = ffi::LJM_Close(self.0);
            check_ret(ret, ())
        }
    }

    /// Closing a handle will remove the associated device connection from the LJM library, free
    /// the device to be opened again, and free allocated system resources.
    ///
    /// See it in the [LJM User Guide](https://support.labjack.com/docs/close-ljm-user-s-guide).
    pub fn close(self) -> Result<(), Error> {
        self.raw_close()
    }

    /// Write one value, specified by name.
    ///
    /// See it in the [LJM User Guide](https://support.labjack.com/docs/ewritename-ljm-user-s-guide).
    pub fn write_name(&self, name: &CString, value: f64) -> Result<(), Error> {
        unsafe {
            let ret = ffi::LJM_eWriteName(self.into(), name.as_ptr(), value);
            check_ret(ret, ())
        }
    }

    /// Read one device register value, specified by name.
    ///
    /// See it in the [LJM User Guide](https://support.labjack.com/docs/ereadname-ljm-user-s-guide).
    pub fn read_name(&self, name: &CString) -> Result<f64, Error> {
        let mut value: f64 = 0.0;
        unsafe {
            let ret = ffi::LJM_eReadName(self.into(), name.as_ptr(), &mut value);
            check_ret(ret, value)
        }
    }

    /// Write to a device register that expects a string value, specified by name.
    ///
    /// See it in the [LJM User Guide](https://support.labjack.com/docs/ewritenamestring-ljm-user-s-guide).
    pub fn write_name_string(&self, name: &CString, str: &LjmString) -> Result<(), Error> {
        unsafe {
            let ret = ffi::LJM_eWriteNameString(self.into(), name.as_ptr(), str.as_ptr());
            check_ret(ret, ())
        }
    }

    /// Read a device register that returns a string, specified by name.
    ///
    /// See it in the [LJM User Guide](https://support.labjack.com/docs/ereadnamestring).
    pub fn read_name_string(&self, name: &CString) -> Result<CString, Error> {
        let mut buf = [0; ljm_sys::LJM_STRING_ALLOCATION_SIZE as usize];
        unsafe {
            let ret = ffi::LJM_eReadNameString(self.into(), name.as_ptr(), buf.as_mut_ptr());
            check_ret(ret, ()).map(|_| CStr::from_ptr(buf.as_ptr()).to_owned())
        }
    }

    /// Write one device register value, specified by register.
    ///
    /// See it in the [LJM User Guide](https://support.labjack.com/docs/ewriteaddress).
    pub fn write_reg(&self, reg: &Register, value: f64) -> Result<(), Error> {
        unsafe {
            let ret = ffi::LJM_eWriteAddress(self.into(), reg.address, reg.data_type.into(), value);
            check_ret(ret, ())
        }
    }

    /// Read one value, specified by register.
    ///
    /// See it in the [LJM User Guide](https://support.labjack.com/docs/ereadaddress-ljm-user-s-guide).
    pub fn read_reg(&self, reg: &Register) -> Result<f64, Error> {
        let mut value: f64 = 0.0;
        unsafe {
            let ret =
                ffi::LJM_eReadAddress(self.into(), reg.address, reg.data_type.into(), &mut value);
            check_ret(ret, value)
        }
    }

    /// Write to a device register that expects a string, specified by register.
    ///
    /// See it in the [LJM User Guide](https://support.labjack.com/docs/ewriteaddressstring-ljm-user-s-guide).
    pub fn write_reg_string(&self, reg: &Register, str: &LjmString) -> Result<(), Error> {
        unsafe {
            let ret = ffi::LJM_eWriteAddressString(self.into(), reg.address, str.as_ptr());
            check_ret(ret, ())
        }
    }

    /// Read a device register that returns a string, specified by register.
    ///
    /// See it in the [LJM User Guide](https://support.labjack.com/docs/ereadaddressstring-ljm-user-s-guide).
    pub fn read_reg_string(&self, reg: &Register) -> Result<CString, Error> {
        let mut buf = [0; ljm_sys::LJM_STRING_ALLOCATION_SIZE as usize];
        unsafe {
            let ret = ffi::LJM_eReadAddressString(self.into(), reg.address, buf.as_mut_ptr());
            check_ret(ret, ()).map(|_| CStr::from_ptr(buf.as_ptr()).to_owned())
        }
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        let _ = self.raw_close();
    }
}

impl From<&Handle> for c_int {
    fn from(value: &Handle) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use anyhow::Context;
    use serial_test::serial;

    use crate::name_to_reg;

    use super::*;

    #[test]
    #[serial()]
    fn rw_name_float() -> anyhow::Result<()> {
        let handle = Handle::open(DeviceType::Any, ConnectionType::Any, Identifier::Any)?;
        let name = CString::new("TEST_FLOAT32")?;

        for float in [1.23, 3.42] {
            handle.write_name(&name, float)?;
            let delta = (float - handle.read_name(&name)?).abs();
            assert!(delta < 2e-7);
        }

        handle.close()?;
        Ok(())
    }

    #[test]
    #[serial]
    fn rw_name_string() -> anyhow::Result<()> {
        let handle = Handle::open(DeviceType::Any, ConnectionType::Any, Identifier::Any)?;
        let name = CString::new("WIFI_SSID_DEFAULT")?;

        for string in [LjmString::new("DEADBEEF")?, LjmString::new("CAFEBABE")?] {
            handle.write_name_string(&name, &string)?;
            assert_eq!(handle.read_name_string(&name)?, CString::from(string));
        }

        handle.close()?;
        Ok(())
    }

    #[test]
    #[serial]
    fn rw_reg_float() -> anyhow::Result<()> {
        let handle = Handle::open(DeviceType::Any, ConnectionType::Any, Identifier::Any)?;
        let name = CString::new("TEST_FLOAT32")?;
        let reg = name_to_reg(&name)?.context("Register not found")?;

        for float in [1.23, 3.42] {
            handle.write_reg(&reg, float)?;
            let delta = (float - handle.read_reg(&reg)?).abs();
            assert!(delta < 2e-7);
        }

        handle.close()?;
        Ok(())
    }

    #[test]
    #[serial]
    fn rw_reg_string() -> anyhow::Result<()> {
        let handle = Handle::open(DeviceType::Any, ConnectionType::Any, Identifier::Any)?;
        let name = CString::new("WIFI_SSID_DEFAULT")?;
        let reg = name_to_reg(&name)?.context("Register not found")?;

        for string in [LjmString::new("DEADBEEF")?, LjmString::new("CAFEBABE")?] {
            handle.write_reg_string(&reg, &string)?;
            assert_eq!(handle.read_reg_string(&reg)?, CString::from(string));
        }

        handle.close()?;
        Ok(())
    }
}
