use std::ffi::{c_int, CStr, CString};

use crate::{ConnectionType, DeviceType, Error, Identifier, LjmString};

fn check_ret<T>(ret: c_int, out: T) -> Result<T, Error> {
    if ret == 0 {
        Ok(out)
    } else {
        Err(ret.into())
    }
}

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

    use super::*;

    #[test]
    fn rw_float() -> anyhow::Result<()> {
        let handle = Handle::open(DeviceType::Any, ConnectionType::Any, Identifier::Any)?;
        let name = CString::new("TEST_FLOAT32")?;
        let float = 1.23;
        handle.write_name(&name, float)?;
        let delta = (float - handle.read_name(&name)?).abs();
        dbg!(&delta);
        assert!(delta < 2e-8);
        handle.close()?;
        Ok(())
    }

    #[test]
    fn rw_string() -> anyhow::Result<()> {
        let handle = Handle::open(DeviceType::Any, ConnectionType::Any, Identifier::Any)?;
        let name = CString::new("WIFI_SSID_DEFAULT")?;
        let string = LjmString::new(uuid::Uuid::new_v4().to_string())?;
        handle.write_name_string(&name, &string)?;
        assert_eq!(handle.read_name_string(&name)?, CString::from(string));
        Ok(())
    }
}
