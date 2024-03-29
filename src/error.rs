use std::os::raw::c_int;

/// An LJM error.
///
/// See it in the [LJM User Guide](https://labjack.com/pages/support?doc=/software-driver/ljm-users-guide/error-codes/).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    AttrLoadCommFailure,
    AutoIpsFileInvalid,
    AutoIpsFileNotFound,
    CannotConnect,
    CannotDisconnect,
    CannotOpenDevice,
    CannotReadOutOnlyStream,
    ConfigFileNotFound,
    ConfigParsingError,
    ConnectionHasYieldedReconnectFailed,
    ConstantsFileNotFound,
    CouldNotStartStream,
    DebugLogFailure,
    DebugLogFileNotOpen,
    DeviceAlreadyOpen,
    DeviceCurrentlyClaimedByAnotherProcess,
    DeviceDisconnected,
    DeviceNotFound,
    DeviceNotOpen,
    DigitalAutoRecoveryErrorDetected,
    ErrorBitSet,
    ErrorRetrievalFailure,
    FramesOmittedDueToPacketSize,
    FunctionDoesNotSupportThisType,
    FunctionErr,
    IncorrectNumCommandBytesSent,
    IncorrectNumResponseBytesReceived,
    IntentNotReady,
    InvalidAddress,
    InvalidConfigName,
    InvalidConnectionType,
    InvalidConstantsFile,
    InvalidDeviceType,
    InvalidDirection,
    InvalidFunction,
    InvalidHandle,
    InvalidIndex,
    InvalidInfoHandle,
    InvalidIntervalHandle,
    InvalidLength,
    InvalidMaxbytespermbfb,
    InvalidName,
    InvalidNumRegisters,
    InvalidNumValues,
    InvalidParameter,
    InvalidProtocolId,
    InvalidTransactionId,
    InvalidValue,
    LengthErr,
    LibraryErrorsBegin,
    LibraryErrorsEnd,
    LjmBufferFull,
    Mbe1IllegalFunction,
    Mbe2IllegalDataAddress,
    Mbe3IllegalDataValue,
    Mbe4SlaveDeviceFailure,
    Mbe5Acknowledge,
    Mbe6SlaveDeviceBusy,
    Mbe8MemoryParityError,
    Mbe10GatewayPathUnavailable,
    Mbe11GatewayTargetNoResponse,
    MemoryAllocationFailure,
    MixedFormatIpAddress,
    ModbusErrorsBegin,
    ModbusErrorsEnd,
    NamedMutexPermissionDenied,
    NegativeReceiveBufferSize,
    Noerror,
    NotImplemented,
    NoCommandBytesSent,
    NoDevicesFound,
    NoResponseBytesReceived,
    NoScansReturned,
    NullObj,
    NullPointer,
    NumBytesErr,
    NumRegsErr,
    OldFirmware,
    OverspecifiedPort,
    PacketSizeTooLarge,
    ProtocolIdErr,
    ReconnectFailed,
    ReservedName,
    SocketLevelError,
    StartingRegErr,
    StreamNotInitialized,
    StreamNotRunning,
    SynchronizationTimeout,
    TemperatureOutOfRange,
    TransactionIdErr,
    U3NotSupportedByLjm,
    U6NotSupportedByLjm,
    Ue9NotSupportedByLjm,
    UnableToStopStream,
    UnitIdErr,
    UnknownError,
    UnknownIdentifier,
    UnknownValueType,
    UnparsableConnectionType,
    UnparsableDeviceType,
    UnparsableIdentifier,
    UsbFailure,
    UsingDefaultCalibration,
    VoltageOutOfRange,
    WarningsBegin,
    WarningsEnd,
    WinsockFailure,
    UnknownErrorCode(i32),
}

impl From<c_int> for Error {
    #[cfg(not(tarpaulin_include))]
    fn from(value: c_int) -> Self {
        use Error::*;
        match value {
            ffi::LJME_ATTR_LOAD_COMM_FAILURE => AttrLoadCommFailure,
            ffi::LJME_AUTO_IPS_FILE_INVALID => AutoIpsFileInvalid,
            ffi::LJME_AUTO_IPS_FILE_NOT_FOUND => AutoIpsFileNotFound,
            ffi::LJME_CANNOT_CONNECT => CannotConnect,
            ffi::LJME_CANNOT_DISCONNECT => CannotDisconnect,
            ffi::LJME_CANNOT_OPEN_DEVICE => CannotOpenDevice,
            ffi::LJME_CANNOT_READ_OUT_ONLY_STREAM => CannotReadOutOnlyStream,
            ffi::LJME_CONFIG_FILE_NOT_FOUND => ConfigFileNotFound,
            ffi::LJME_CONFIG_PARSING_ERROR => ConfigParsingError,
            ffi::LJME_CONNECTION_HAS_YIELDED_RECONNECT_FAILED => {
                ConnectionHasYieldedReconnectFailed
            }
            ffi::LJME_CONSTANTS_FILE_NOT_FOUND => ConstantsFileNotFound,
            ffi::LJME_COULD_NOT_START_STREAM => CouldNotStartStream,
            ffi::LJME_DEBUG_LOG_FAILURE => DebugLogFailure,
            ffi::LJME_DEBUG_LOG_FILE_NOT_OPEN => DebugLogFileNotOpen,
            ffi::LJME_DEVICE_ALREADY_OPEN => DeviceAlreadyOpen,
            ffi::LJME_DEVICE_CURRENTLY_CLAIMED_BY_ANOTHER_PROCESS => {
                DeviceCurrentlyClaimedByAnotherProcess
            }
            ffi::LJME_DEVICE_DISCONNECTED => DeviceDisconnected,
            ffi::LJME_DEVICE_NOT_FOUND => DeviceNotFound,
            ffi::LJME_DEVICE_NOT_OPEN => DeviceNotOpen,
            ffi::LJME_DIGITAL_AUTO_RECOVERY_ERROR_DETECTED => DigitalAutoRecoveryErrorDetected,
            ffi::LJME_ERROR_BIT_SET => ErrorBitSet,
            ffi::LJME_ERROR_RETRIEVAL_FAILURE => ErrorRetrievalFailure,
            ffi::LJME_FRAMES_OMITTED_DUE_TO_PACKET_SIZE => FramesOmittedDueToPacketSize,
            ffi::LJME_FUNCTION_DOES_NOT_SUPPORT_THIS_TYPE => FunctionDoesNotSupportThisType,
            ffi::LJME_FUNCTION_ERR => FunctionErr,
            ffi::LJME_INCORRECT_NUM_COMMAND_BYTES_SENT => IncorrectNumCommandBytesSent,
            ffi::LJME_INCORRECT_NUM_RESPONSE_BYTES_RECEIVED => IncorrectNumResponseBytesReceived,
            ffi::LJME_INTENT_NOT_READY => IntentNotReady,
            ffi::LJME_INVALID_ADDRESS => InvalidAddress,
            ffi::LJME_INVALID_CONFIG_NAME => InvalidConfigName,
            ffi::LJME_INVALID_CONNECTION_TYPE => InvalidConnectionType,
            ffi::LJME_INVALID_CONSTANTS_FILE => InvalidConstantsFile,
            ffi::LJME_INVALID_DEVICE_TYPE => InvalidDeviceType,
            ffi::LJME_INVALID_DIRECTION => InvalidDirection,
            ffi::LJME_INVALID_FUNCTION => InvalidFunction,
            ffi::LJME_INVALID_HANDLE => InvalidHandle,
            ffi::LJME_INVALID_INDEX => InvalidIndex,
            ffi::LJME_INVALID_INFO_HANDLE => InvalidInfoHandle,
            ffi::LJME_INVALID_INTERVAL_HANDLE => InvalidIntervalHandle,
            ffi::LJME_INVALID_LENGTH => InvalidLength,
            ffi::LJME_INVALID_MAXBYTESPERMBFB => InvalidMaxbytespermbfb,
            ffi::LJME_INVALID_NAME => InvalidName,
            ffi::LJME_INVALID_NUM_REGISTERS => InvalidNumRegisters,
            ffi::LJME_INVALID_NUM_VALUES => InvalidNumValues,
            ffi::LJME_INVALID_PARAMETER => InvalidParameter,
            ffi::LJME_INVALID_PROTOCOL_ID => InvalidProtocolId,
            ffi::LJME_INVALID_TRANSACTION_ID => InvalidTransactionId,
            ffi::LJME_INVALID_VALUE => InvalidValue,
            ffi::LJME_LENGTH_ERR => LengthErr,
            ffi::LJME_LIBRARY_ERRORS_BEGIN => LibraryErrorsBegin,
            ffi::LJME_LIBRARY_ERRORS_END => LibraryErrorsEnd,
            ffi::LJME_LJM_BUFFER_FULL => LjmBufferFull,
            ffi::LJME_MBE1_ILLEGAL_FUNCTION => Mbe1IllegalFunction,
            ffi::LJME_MBE2_ILLEGAL_DATA_ADDRESS => Mbe2IllegalDataAddress,
            ffi::LJME_MBE3_ILLEGAL_DATA_VALUE => Mbe3IllegalDataValue,
            ffi::LJME_MBE4_SLAVE_DEVICE_FAILURE => Mbe4SlaveDeviceFailure,
            ffi::LJME_MBE5_ACKNOWLEDGE => Mbe5Acknowledge,
            ffi::LJME_MBE6_SLAVE_DEVICE_BUSY => Mbe6SlaveDeviceBusy,
            ffi::LJME_MBE8_MEMORY_PARITY_ERROR => Mbe8MemoryParityError,
            ffi::LJME_MBE10_GATEWAY_PATH_UNAVAILABLE => Mbe10GatewayPathUnavailable,
            ffi::LJME_MBE11_GATEWAY_TARGET_NO_RESPONSE => Mbe11GatewayTargetNoResponse,
            ffi::LJME_MEMORY_ALLOCATION_FAILURE => MemoryAllocationFailure,
            ffi::LJME_MIXED_FORMAT_IP_ADDRESS => MixedFormatIpAddress,
            ffi::LJME_MODBUS_ERRORS_BEGIN => ModbusErrorsBegin,
            ffi::LJME_MODBUS_ERRORS_END => ModbusErrorsEnd,
            ffi::LJME_NAMED_MUTEX_PERMISSION_DENIED => NamedMutexPermissionDenied,
            ffi::LJME_NEGATIVE_RECEIVE_BUFFER_SIZE => NegativeReceiveBufferSize,
            ffi::LJME_NOERROR => Noerror,
            ffi::LJME_NOT_IMPLEMENTED => NotImplemented,
            ffi::LJME_NO_COMMAND_BYTES_SENT => NoCommandBytesSent,
            ffi::LJME_NO_DEVICES_FOUND => NoDevicesFound,
            ffi::LJME_NO_RESPONSE_BYTES_RECEIVED => NoResponseBytesReceived,
            ffi::LJME_NO_SCANS_RETURNED => NoScansReturned,
            ffi::LJME_NULL_OBJ => NullObj,
            ffi::LJME_NULL_POINTER => NullPointer,
            ffi::LJME_NUM_BYTES_ERR => NumBytesErr,
            ffi::LJME_NUM_REGS_ERR => NumRegsErr,
            ffi::LJME_OLD_FIRMWARE => OldFirmware,
            ffi::LJME_OVERSPECIFIED_PORT => OverspecifiedPort,
            ffi::LJME_PACKET_SIZE_TOO_LARGE => PacketSizeTooLarge,
            ffi::LJME_PROTOCOL_ID_ERR => ProtocolIdErr,
            ffi::LJME_RECONNECT_FAILED => ReconnectFailed,
            ffi::LJME_RESERVED_NAME => ReservedName,
            ffi::LJME_SOCKET_LEVEL_ERROR => SocketLevelError,
            ffi::LJME_STARTING_REG_ERR => StartingRegErr,
            ffi::LJME_STREAM_NOT_INITIALIZED => StreamNotInitialized,
            ffi::LJME_STREAM_NOT_RUNNING => StreamNotRunning,
            ffi::LJME_SYNCHRONIZATION_TIMEOUT => SynchronizationTimeout,
            ffi::LJME_TEMPERATURE_OUT_OF_RANGE => TemperatureOutOfRange,
            ffi::LJME_TRANSACTION_ID_ERR => TransactionIdErr,
            ffi::LJME_U3_NOT_SUPPORTED_BY_LJM => U3NotSupportedByLjm,
            ffi::LJME_U6_NOT_SUPPORTED_BY_LJM => U6NotSupportedByLjm,
            ffi::LJME_UE9_NOT_SUPPORTED_BY_LJM => Ue9NotSupportedByLjm,
            ffi::LJME_UNABLE_TO_STOP_STREAM => UnableToStopStream,
            ffi::LJME_UNIT_ID_ERR => UnitIdErr,
            ffi::LJME_UNKNOWN_ERROR => UnknownError,
            ffi::LJME_UNKNOWN_IDENTIFIER => UnknownIdentifier,
            ffi::LJME_UNKNOWN_VALUE_TYPE => UnknownValueType,
            ffi::LJME_UNPARSABLE_CONNECTION_TYPE => UnparsableConnectionType,
            ffi::LJME_UNPARSABLE_DEVICE_TYPE => UnparsableDeviceType,
            ffi::LJME_UNPARSABLE_IDENTIFIER => UnparsableIdentifier,
            ffi::LJME_USB_FAILURE => UsbFailure,
            ffi::LJME_USING_DEFAULT_CALIBRATION => UsingDefaultCalibration,
            ffi::LJME_VOLTAGE_OUT_OF_RANGE => VoltageOutOfRange,
            ffi::LJME_WARNINGS_BEGIN => WarningsBegin,
            ffi::LJME_WARNINGS_END => WarningsEnd,
            ffi::LJME_WINSOCK_FAILURE => WinsockFailure,
            _ => UnknownErrorCode(value),
        }
    }
}

impl std::fmt::Display for Error {
    #[cfg(not(tarpaulin_include))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl std::error::Error for Error {}
