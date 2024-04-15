use std::{ffi::CString, os::raw::c_int};

use crate::{DataType, Error, Register};

pub(crate) fn check_ret<T>(ret: c_int, out: T) -> Result<T, Error> {
    if ret == 0 {
        Ok(out)
    } else {
        Err(ret.into())
    }
}

/// Takes a Modbus register name as input and produces the corresponding Modbus register.
///
/// Returns None if LJM is not able to identify the register.
/// See it in the [LJM User Guide](https://support.labjack.com/docs/nametoaddress-ljm-user-s-guide).
pub fn name_to_reg(name: &CString) -> Result<Option<Register>, Error> {
    let mut address: c_int = -1;
    let mut data_type: c_int = ffi::LJM_INVALID_NAME_ADDRESS;
    unsafe {
        let ret = ffi::LJM_NameToAddress(name.as_ptr(), &mut address, &mut data_type);
        let reg = DataType::try_from(data_type)
            .ok()
            .map(|data_type| Register { address, data_type });
        check_ret(ret, reg)
    }
}
