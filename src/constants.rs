use std::os::raw::c_int;

/// A LabJack device type.
///
/// See it in the [LJM User Guide](https://labjack.com/pages/support?doc=/software-driver/ljm-users-guide/open/#header-three-fksj0).
#[derive(Debug, Clone)]
pub enum DeviceType {
    Any,
    T4,
    T7,
    Digit,
    TSeries,
}

impl From<DeviceType> for c_int {
    fn from(value: DeviceType) -> Self {
        (match value {
            DeviceType::Any => ffi::LJM_dtANY,
            DeviceType::T4 => ffi::LJM_dtT4,
            DeviceType::T7 => ffi::LJM_dtT7,
            DeviceType::Digit => ffi::LJM_dtDIGIT,
            DeviceType::TSeries => ffi::LJM_dtTSERIES,
        } as c_int)
    }
}

/// A LabJack device connection type.
#[derive(Debug, Clone)]
pub enum ConnectionType {
    Any,
    AnyTcp,
    Usb,
    Tcp,
    NetworkTcp,
    Ethernet,
    EthernetTcp,
    Wifi,
    WifiTcp,
    NetworkUdp,
    EthernetUdp,
    WifiUdp,
    NetworkAny,
    EthernetAny,
    WifiAny,
}

impl From<ConnectionType> for c_int {
    fn from(value: ConnectionType) -> Self {
        (match value {
            ConnectionType::Any => ffi::LJM_ctANY,
            ConnectionType::AnyTcp => ffi::LJM_ctANY_TCP,
            ConnectionType::Usb => ffi::LJM_ctUSB,
            ConnectionType::Tcp => ffi::LJM_ctTCP,
            ConnectionType::NetworkTcp => ffi::LJM_ctNETWORK_TCP,
            ConnectionType::Ethernet => ffi::LJM_ctETHERNET,
            ConnectionType::EthernetTcp => ffi::LJM_ctETHERNET_TCP,
            ConnectionType::Wifi => ffi::LJM_ctWIFI,
            ConnectionType::WifiTcp => ffi::LJM_ctWIFI_TCP,
            ConnectionType::NetworkUdp => ffi::LJM_ctNETWORK_UDP,
            ConnectionType::EthernetUdp => ffi::LJM_ctETHERNET_UDP,
            ConnectionType::WifiUdp => ffi::LJM_ctWIFI_UDP,
            ConnectionType::NetworkAny => ffi::LJM_ctNETWORK_ANY,
            ConnectionType::EthernetAny => ffi::LJM_ctETHERNET_ANY,
            ConnectionType::WifiAny => ffi::LJM_ctWIFI_ANY,
        } as c_int)
    }
}
