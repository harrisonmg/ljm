use std::{ffi::CString, os::raw::c_int};

use crate::{ConnectionType, DeviceType, Error, Identifier};

fn check_ret<T>(ret: c_int, out: T) -> Result<T, Error> {
    if ret == 0 {
        Ok(out)
    } else {
        Err(ret.into())
    }
}

/// A LabJack device handle.
#[derive(Debug)]
pub struct Handle(pub(crate) c_int);

impl Handle {
    /// Opens a desired LabJack and associates a device handle number (connection ID).
    ///
    /// See it in the [LJM User Guide](https://labjack.com/pages/support?doc=%2Fsoftware-driver%2Fljm-users-guide%2Fopen%2F).
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
    /// See it in the [LJM User Guide](https://labjack.com/pages/support?doc=%2Fsoftware-driver%2Fljm-users-guide%2Fclose%2F).
    pub fn close(self) -> Result<(), Error> {
        self.raw_close()
    }

    /// Write one value, specified by name.
    ///
    /// See it in the [LJM User Guide](https://labjack.com/pages/support?doc=/software-driver/ljm-users-guide/ewritename/).
    pub fn write_name(&self, name: CString, value: f64) -> Result<(), Error> {
        unsafe {
            let ret = ffi::LJM_eWriteName(self.into(), name.as_ptr(), value);
            check_ret(ret, ())
        }
    }

    /// Read one device register value, specified by name.
    ///
    /// See it in the [LJM User Guide](https://labjack.com/pages/support?doc=/software-driver/ljm-users-guide/ereadname/).
    pub fn read_name(&self, name: CString) -> Result<f64, Error> {
        let mut value: f64 = 0.0;
        unsafe {
            let ret = ffi::LJM_eReadName(self.into(), name.as_ptr(), &mut value);
            check_ret(ret, value)
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
