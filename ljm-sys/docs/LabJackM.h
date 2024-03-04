// NOTE: This header is intended only for the use of generating Rust crate documentation.
//       Most of the inline documentation has been removed and it has been reformatted.
/**
 * Name: LabJackM.h
 * Desc: Header file describing C-style exposed
 *       API functions for the LabJackM Library
 * Auth: LabJack Corp.
**/
#ifndef LAB_JACK_M_HEADER
#define LAB_JACK_M_HEADER
#define LJM_VERSION 1.2000
#define LJM_ERROR_CODE static const int
#ifdef __cplusplus
extern "C" {
#endif
#ifdef WIN32
	#define LJM_ERROR_RETURN int __stdcall
	#define LJM_LONG_LONG_RETURN long long __stdcall
	#define LJM_VOID_RETURN void __stdcall
	#define LJM_ERROR_STRING const char * __stdcall
	#define LJM_DOUBLE_RETURN double __stdcall
#else
	#ifdef __APPLE__
		#define LJM_ERROR_RETURN int
		#define LJM_LONG_LONG_RETURN long long
		#define LJM_VOID_RETURN void
		#define LJM_ERROR_STRING const char *
		#define LJM_DOUBLE_RETURN double
	#else
		#define LJM_ERROR_RETURN __attribute__((__visibility__("default"))) int
		#define LJM_LONG_LONG_RETURN __attribute__((__visibility__("default"))) long long
		#define LJM_VOID_RETURN __attribute__((__visibility__("default"))) void
		#define LJM_ERROR_STRING __attribute__((__visibility__("default"))) const char *
		#define LJM_DOUBLE_RETURN __attribute__((__visibility__("default"))) double
	#endif
#endif
static const int LJM_READ = 0;
static const int LJM_WRITE = 1;
static const int LJM_UINT16 = 0;
static const int LJM_UINT32 = 1;
static const int LJM_INT32 = 2;
static const int LJM_FLOAT32 = 3;
static const int LJM_BYTE = 99;
static const int LJM_STRING = 98;
static const unsigned int LJM_STRING_MAX_SIZE = 49;
enum { LJM_STRING_ALLOCATION_SIZE = 50 };
static const int LJM_INVALID_NAME_ADDRESS = -1;
enum { LJM_MAX_NAME_SIZE = 256 };
enum { LJM_MAC_STRING_SIZE = 18 };
enum { LJM_IPv4_STRING_SIZE = 16 };
static const int LJM_BYTES_PER_REGISTER = 2;
enum {
	LJM_dtANY = 0,
	LJM_dtT4 = 4,
	LJM_dtT7 = 7,
	LJM_dtDIGIT = 200,
	LJM_dtTSERIES = 84
};
enum {
	LJM_ctANY = 0,
	LJM_ctANY_TCP = LJM_ctANY,
	LJM_ctUSB = 1,
	LJM_ctTCP = 2,
	LJM_ctNETWORK_TCP = LJM_ctTCP,
	LJM_ctETHERNET = 3,
	LJM_ctETHERNET_TCP = LJM_ctETHERNET,
	LJM_ctWIFI = 4,
	LJM_ctWIFI_TCP = LJM_ctWIFI,
	LJM_ctNETWORK_UDP = 5,
	LJM_ctETHERNET_UDP = 6,
	LJM_ctWIFI_UDP = 7,
	LJM_ctNETWORK_ANY = 8,
	LJM_ctETHERNET_ANY = 9,
	LJM_ctWIFI_ANY = 10
};
static const int LJM_TCP_PORT = 502;
static const int LJM_ETHERNET_UDP_PORT = 52362;
static const int LJM_WIFI_UDP_PORT = 502;
static const int LJM_NO_IP_ADDRESS = 0;
static const int LJM_NO_PORT = 0;
static const char * const LJM_DEMO_MODE = "-2";
static const int LJM_idANY = 0;
enum { LJM_DEFAULT_FEEDBACK_ALLOCATION_SIZE = 62 };
static const int LJM_USE_DEFAULT_MAXBYTESPERMBFB = 0;
static const int LJM_DEFAULT_UNIT_ID = 1;
enum { LJM_LIST_ALL_SIZE = 128 };
static const int LJM_NO_TIMEOUT = 0;
static const int LJM_DEFAULT_USB_SEND_RECEIVE_TIMEOUT_MS = 2600;
static const int LJM_DEFAULT_ETHERNET_OPEN_TIMEOUT_MS = 1000;
static const int LJM_DEFAULT_ETHERNET_SEND_RECEIVE_TIMEOUT_MS = 2600;
static const int LJM_DEFAULT_WIFI_OPEN_TIMEOUT_MS = 1000;
static const int LJM_DEFAULT_WIFI_SEND_RECEIVE_TIMEOUT_MS = 4000;
static const int LJM_DUMMY_VALUE = -9999;
static const int LJM_SCAN_NOT_READ = -8888;
static const int LJM_GND = 199;
LJM_ERROR_CODE LJME_NOERROR = 0;
LJM_ERROR_CODE LJME_WARNINGS_BEGIN = 200;
LJM_ERROR_CODE LJME_WARNINGS_END = 399;
LJM_ERROR_CODE LJME_FRAMES_OMITTED_DUE_TO_PACKET_SIZE = 201;
LJM_ERROR_CODE LJME_DEBUG_LOG_FAILURE = 202;
LJM_ERROR_CODE LJME_USING_DEFAULT_CALIBRATION = 203;
LJM_ERROR_CODE LJME_DEBUG_LOG_FILE_NOT_OPEN = 204;
LJM_ERROR_CODE LJME_MODBUS_ERRORS_BEGIN = 1200;
LJM_ERROR_CODE LJME_MODBUS_ERRORS_END = 1216;
LJM_ERROR_CODE LJME_MBE1_ILLEGAL_FUNCTION = 1201;
LJM_ERROR_CODE LJME_MBE2_ILLEGAL_DATA_ADDRESS = 1202;
LJM_ERROR_CODE LJME_MBE3_ILLEGAL_DATA_VALUE = 1203;
LJM_ERROR_CODE LJME_MBE4_SLAVE_DEVICE_FAILURE = 1204;
LJM_ERROR_CODE LJME_MBE5_ACKNOWLEDGE = 1205;
LJM_ERROR_CODE LJME_MBE6_SLAVE_DEVICE_BUSY = 1206;
LJM_ERROR_CODE LJME_MBE8_MEMORY_PARITY_ERROR = 1208;
LJM_ERROR_CODE LJME_MBE10_GATEWAY_PATH_UNAVAILABLE = 1210;
LJM_ERROR_CODE LJME_MBE11_GATEWAY_TARGET_NO_RESPONSE = 1211;
LJM_ERROR_CODE LJME_LIBRARY_ERRORS_BEGIN = 1220;
LJM_ERROR_CODE LJME_LIBRARY_ERRORS_END = 1399;
LJM_ERROR_CODE LJME_UNKNOWN_ERROR = 1221;
LJM_ERROR_CODE LJME_INVALID_DEVICE_TYPE = 1222;
LJM_ERROR_CODE LJME_INVALID_HANDLE = 1223;
LJM_ERROR_CODE LJME_DEVICE_NOT_OPEN = 1224;
LJM_ERROR_CODE LJME_STREAM_NOT_INITIALIZED = 1225;
LJM_ERROR_CODE LJME_DEVICE_DISCONNECTED = 1226;
LJM_ERROR_CODE LJME_DEVICE_NOT_FOUND = 1227;
LJM_ERROR_CODE LJME_DEVICE_ALREADY_OPEN = 1229;
LJM_ERROR_CODE LJME_DEVICE_CURRENTLY_CLAIMED_BY_ANOTHER_PROCESS = 1230;
LJM_ERROR_CODE LJME_CANNOT_CONNECT = 1231;
LJM_ERROR_CODE LJME_SOCKET_LEVEL_ERROR = 1233;
LJM_ERROR_CODE LJME_CANNOT_OPEN_DEVICE = 1236;
LJM_ERROR_CODE LJME_CANNOT_DISCONNECT = 1237;
LJM_ERROR_CODE LJME_WINSOCK_FAILURE = 1238;
LJM_ERROR_CODE LJME_RECONNECT_FAILED = 1239;
LJM_ERROR_CODE LJME_CONNECTION_HAS_YIELDED_RECONNECT_FAILED = 1240;
LJM_ERROR_CODE LJME_USB_FAILURE = 1241;
LJM_ERROR_CODE LJME_U3_NOT_SUPPORTED_BY_LJM = 1243;
LJM_ERROR_CODE LJME_U6_NOT_SUPPORTED_BY_LJM = 1246;
LJM_ERROR_CODE LJME_UE9_NOT_SUPPORTED_BY_LJM = 1249;
LJM_ERROR_CODE LJME_INVALID_ADDRESS = 1250;
LJM_ERROR_CODE LJME_INVALID_CONNECTION_TYPE = 1251;
LJM_ERROR_CODE LJME_INVALID_DIRECTION = 1252;
LJM_ERROR_CODE LJME_INVALID_FUNCTION = 1253;
LJM_ERROR_CODE LJME_INVALID_NUM_REGISTERS = 1254;
LJM_ERROR_CODE LJME_INVALID_PARAMETER = 1255;
LJM_ERROR_CODE LJME_INVALID_PROTOCOL_ID = 1256;
LJM_ERROR_CODE LJME_INVALID_TRANSACTION_ID = 1257;
LJM_ERROR_CODE LJME_UNKNOWN_VALUE_TYPE = 1259;
LJM_ERROR_CODE LJME_MEMORY_ALLOCATION_FAILURE = 1260;
LJM_ERROR_CODE LJME_NO_COMMAND_BYTES_SENT = 1261;
LJM_ERROR_CODE LJME_INCORRECT_NUM_COMMAND_BYTES_SENT = 1262;
LJM_ERROR_CODE LJME_NO_RESPONSE_BYTES_RECEIVED = 1263;
LJM_ERROR_CODE LJME_INCORRECT_NUM_RESPONSE_BYTES_RECEIVED = 1264;
LJM_ERROR_CODE LJME_MIXED_FORMAT_IP_ADDRESS = 1265;
LJM_ERROR_CODE LJME_UNKNOWN_IDENTIFIER = 1266;
LJM_ERROR_CODE LJME_NOT_IMPLEMENTED = 1267;
LJM_ERROR_CODE LJME_INVALID_INDEX = 1268;
LJM_ERROR_CODE LJME_INVALID_LENGTH = 1269;
LJM_ERROR_CODE LJME_ERROR_BIT_SET = 1270;
LJM_ERROR_CODE LJME_INVALID_MAXBYTESPERMBFB = 1271;
LJM_ERROR_CODE LJME_NULL_POINTER = 1272;
LJM_ERROR_CODE LJME_NULL_OBJ = 1273;
LJM_ERROR_CODE LJME_RESERVED_NAME = 1274;
LJM_ERROR_CODE LJME_UNPARSABLE_DEVICE_TYPE = 1275;
LJM_ERROR_CODE LJME_UNPARSABLE_CONNECTION_TYPE = 1276;
LJM_ERROR_CODE LJME_UNPARSABLE_IDENTIFIER = 1277;
LJM_ERROR_CODE LJME_PACKET_SIZE_TOO_LARGE = 1278;
LJM_ERROR_CODE LJME_TRANSACTION_ID_ERR = 1279;
LJM_ERROR_CODE LJME_PROTOCOL_ID_ERR = 1280;
LJM_ERROR_CODE LJME_LENGTH_ERR = 1281;
LJM_ERROR_CODE LJME_UNIT_ID_ERR = 1282;
LJM_ERROR_CODE LJME_FUNCTION_ERR = 1283;
LJM_ERROR_CODE LJME_STARTING_REG_ERR = 1284;
LJM_ERROR_CODE LJME_NUM_REGS_ERR = 1285;
LJM_ERROR_CODE LJME_NUM_BYTES_ERR = 1286;
LJM_ERROR_CODE LJME_CONFIG_FILE_NOT_FOUND = 1289;
LJM_ERROR_CODE LJME_CONFIG_PARSING_ERROR = 1290;
LJM_ERROR_CODE LJME_INVALID_NUM_VALUES = 1291;
LJM_ERROR_CODE LJME_CONSTANTS_FILE_NOT_FOUND = 1292;
LJM_ERROR_CODE LJME_INVALID_CONSTANTS_FILE = 1293;
LJM_ERROR_CODE LJME_INVALID_NAME = 1294;
LJM_ERROR_CODE LJME_OVERSPECIFIED_PORT = 1296;
LJM_ERROR_CODE LJME_INTENT_NOT_READY = 1297;
LJM_ERROR_CODE LJME_ATTR_LOAD_COMM_FAILURE = 1298;
LJM_ERROR_CODE LJME_INVALID_CONFIG_NAME = 1299;
LJM_ERROR_CODE LJME_ERROR_RETRIEVAL_FAILURE = 1300;
LJM_ERROR_CODE LJME_LJM_BUFFER_FULL = 1301;
LJM_ERROR_CODE LJME_COULD_NOT_START_STREAM = 1302;
LJM_ERROR_CODE LJME_STREAM_NOT_RUNNING = 1303;
LJM_ERROR_CODE LJME_UNABLE_TO_STOP_STREAM = 1304;
LJM_ERROR_CODE LJME_INVALID_VALUE = 1305;
LJM_ERROR_CODE LJME_SYNCHRONIZATION_TIMEOUT = 1306;
LJM_ERROR_CODE LJME_OLD_FIRMWARE = 1307;
LJM_ERROR_CODE LJME_CANNOT_READ_OUT_ONLY_STREAM = 1308;
LJM_ERROR_CODE LJME_NO_SCANS_RETURNED = 1309;
LJM_ERROR_CODE LJME_TEMPERATURE_OUT_OF_RANGE = 1310;
LJM_ERROR_CODE LJME_VOLTAGE_OUT_OF_RANGE = 1311;
LJM_ERROR_CODE LJME_FUNCTION_DOES_NOT_SUPPORT_THIS_TYPE = 1312;
LJM_ERROR_CODE LJME_INVALID_INFO_HANDLE = 1313;
LJM_ERROR_CODE LJME_NO_DEVICES_FOUND = 1314;
LJM_ERROR_CODE LJME_AUTO_IPS_FILE_NOT_FOUND = 1316;
LJM_ERROR_CODE LJME_AUTO_IPS_FILE_INVALID = 1317;
LJM_ERROR_CODE LJME_INVALID_INTERVAL_HANDLE = 1318;
LJM_ERROR_CODE LJME_NAMED_MUTEX_PERMISSION_DENIED = 1319;
LJM_ERROR_CODE LJME_DIGITAL_AUTO_RECOVERY_ERROR_DETECTED = 1320;
LJM_ERROR_CODE LJME_NEGATIVE_RECEIVE_BUFFER_SIZE = 1321;
LJM_ERROR_RETURN LJM_ListAll(int DeviceType, int ConnectionType,
	int * NumFound, int * aDeviceTypes, int * aConnectionTypes,
	int * aSerialNumbers, int * aIPAddresses);
LJM_ERROR_RETURN LJM_ListAllS(const char * DeviceType, const char * ConnectionType,
	int * NumFound, int * aDeviceTypes, int * aConnectionTypes,
	int * aSerialNumbers, int * aIPAddresses);
LJM_ERROR_RETURN LJM_ListAllExtended(int DeviceType, int ConnectionType,
	int NumAddresses, const int * aAddresses, const int * aNumRegs,
	int MaxNumFound, int * NumFound, int * aDeviceTypes, int * aConnectionTypes,
	int * aSerialNumbers, int * aIPAddresses, unsigned char * aBytes);
LJM_ERROR_RETURN LJM_OpenS(const char * DeviceType, const char * ConnectionType,
	const char * Identifier, int * Handle);
LJM_ERROR_RETURN LJM_Open(int DeviceType, int ConnectionType,
	const char * Identifier, int * Handle);
LJM_ERROR_RETURN LJM_GetHandleInfo(int Handle, int * DeviceType,
	int * ConnectionType, int * SerialNumber, int * IPAddress, int * Port,
	int * MaxBytesPerMB);
LJM_ERROR_RETURN LJM_Close(int Handle);
LJM_ERROR_RETURN LJM_CloseAll(void);
LJM_ERROR_RETURN LJM_CleanInfo(int InfoHandle);
LJM_ERROR_RETURN LJM_eWriteAddress(int Handle, int Address, int Type, double Value);
LJM_ERROR_RETURN LJM_eReadAddress(int Handle, int Address, int Type, double * Value);
LJM_ERROR_RETURN LJM_eWriteName(int Handle, const char * Name, double Value);
LJM_ERROR_RETURN LJM_eReadName(int Handle, const char * Name, double * Value);
LJM_ERROR_RETURN LJM_eReadAddresses(int Handle, int NumFrames,
	const int * aAddresses, const int * aTypes, double * aValues,
	int * ErrorAddress);
LJM_ERROR_RETURN LJM_eReadNames(int Handle, int NumFrames,
	const char ** aNames, double * aValues, int * ErrorAddress);
LJM_ERROR_RETURN LJM_eWriteAddresses(int Handle, int NumFrames,
	const int * aAddresses, const int * aTypes, const double * aValues,
	int * ErrorAddress);
LJM_ERROR_RETURN LJM_eWriteNames(int Handle, int NumFrames,
	const char ** aNames, const double * aValues, int * ErrorAddress);
LJM_ERROR_RETURN LJM_eReadAddressArray(int Handle, int Address, int Type,
	int NumValues, double * aValues, int * ErrorAddress);
LJM_ERROR_RETURN LJM_eReadNameArray(int Handle, const char * Name,
	int NumValues, double * aValues, int * ErrorAddress);
LJM_ERROR_RETURN LJM_eWriteAddressArray(int Handle, int Address, int Type,
	int NumValues, const double * aValues, int * ErrorAddress);
LJM_ERROR_RETURN LJM_eWriteNameArray(int Handle, const char * Name,
	int NumValues, const double * aValues, int * ErrorAddress);
LJM_ERROR_RETURN LJM_eReadAddressByteArray(int Handle, int Address,
	int NumBytes, char * aBytes, int * ErrorAddress);
LJM_ERROR_RETURN LJM_eReadNameByteArray(int Handle, const char * Name,
	int NumBytes, char * aBytes, int * ErrorAddress);
LJM_ERROR_RETURN LJM_eWriteAddressByteArray(int Handle, int Address,
	int NumBytes, const char * aBytes, int * ErrorAddress);
LJM_ERROR_RETURN LJM_eWriteNameByteArray(int Handle, const char * Name,
	int NumBytes, const char * aBytes, int * ErrorAddress);
LJM_ERROR_RETURN LJM_eAddresses(int Handle, int NumFrames,
	const int * aAddresses, const int * aTypes, const int * aWrites,
	const int * aNumValues, double * aValues, int * ErrorAddress);
LJM_ERROR_RETURN LJM_eNames(int Handle, int NumFrames, const char ** aNames,
	const int * aWrites, const int * aNumValues, double * aValues,
	int * ErrorAddress);
LJM_ERROR_RETURN LJM_eReadNameString(int Handle, const char * Name,
	char * String);
LJM_ERROR_RETURN LJM_eReadAddressString(int Handle, int Address,
	char * String);
LJM_ERROR_RETURN LJM_eWriteNameString(int Handle, const char * Name,
	const char * String);
LJM_ERROR_RETURN LJM_eWriteAddressString(int Handle, int Address,
	const char * String);
LJM_ERROR_RETURN LJM_eStreamStart(int Handle, int ScansPerRead,
	int NumAddresses, const int * aScanList, double * ScanRate);
LJM_ERROR_RETURN LJM_eStreamRead(int Handle, double * aData,
	int * DeviceScanBacklog, int * LJMScanBacklog);
typedef void (*LJM_StreamReadCallback)(void *);
LJM_ERROR_RETURN LJM_SetStreamCallback(int Handle,
	LJM_StreamReadCallback Callback, void * Arg);
LJM_ERROR_RETURN LJM_eStreamStop(int Handle);
LJM_ERROR_RETURN LJM_StreamBurst(int Handle, int NumAddresses,
	const int * aScanList, double * ScanRate, unsigned int NumScans,
	double * aData);
LJM_ERROR_RETURN LJM_GetStreamTCPReceiveBufferStatus(int Handle,
	unsigned int * ReceiveBufferBytesSize,
	unsigned int * ReceiveBufferBytesBacklog);
LJM_ERROR_RETURN LJM_WriteRaw(int Handle, const unsigned char * Data,
	int NumBytes);
LJM_ERROR_RETURN LJM_ReadRaw(int Handle, unsigned char * Data, int NumBytes);
LJM_ERROR_RETURN LJM_AddressesToMBFB(int MaxBytesPerMBFB, const int * aAddresses,
	const int * aTypes, const int * aWrites, const int * aNumValues,
	const double * aValues, int * NumFrames, unsigned char * aMBFBCommand);
LJM_ERROR_RETURN LJM_MBFBComm(int Handle, unsigned char UnitID,
	unsigned char * aMBFB, int * ErrorAddress);
LJM_ERROR_RETURN LJM_UpdateValues(unsigned char * aMBFBResponse,
	const int * aTypes, const int * aWrites, const int * aNumValues,
	int NumFrames, double * aValues);
LJM_ERROR_RETURN LJM_NamesToAddresses(int NumFrames, const char ** aNames,
	int * aAddresses, int * aTypes);
LJM_ERROR_RETURN LJM_NameToAddress(const char * Name, int * Address, int * Type);
LJM_ERROR_RETURN LJM_AddressesToTypes(int NumAddresses, int * aAddresses,
	int * aTypes);
LJM_ERROR_RETURN LJM_AddressToType(int Address, int * Type);
LJM_ERROR_RETURN LJM_LookupConstantValue(const char * Scope,
	const char * ConstantName, double * ConstantValue);
LJM_ERROR_RETURN LJM_LookupConstantName(const char * Scope,
	double ConstantValue, char * ConstantName);
LJM_VOID_RETURN LJM_ErrorToString(int ErrorCode, char * ErrorString);
LJM_VOID_RETURN LJM_LoadConstants(void);
LJM_ERROR_RETURN LJM_LoadConstantsFromFile(const char * FileName);
LJM_ERROR_RETURN LJM_LoadConstantsFromString(const char * JsonString);
static const long LJM_ttB = 6001;
static const long LJM_ttE = 6002;
static const long LJM_ttJ = 6003;
static const long LJM_ttK = 6004;
static const long LJM_ttN = 6005;
static const long LJM_ttR = 6006;
static const long LJM_ttS = 6007;
static const long LJM_ttT = 6008;
static const long LJM_ttC = 6009;
LJM_ERROR_RETURN LJM_TCVoltsToTemp(int TCType, double TCVolts, double CJTempK,
	double * pTCTempK);
LJM_VOID_RETURN LJM_FLOAT32ToByteArray(const float * aFLOAT32, int RegisterOffset, int NumFLOAT32, unsigned char * aBytes);
LJM_VOID_RETURN LJM_ByteArrayToFLOAT32(const unsigned char * aBytes, int RegisterOffset, int NumFLOAT32, float * aFLOAT32);
LJM_VOID_RETURN LJM_UINT16ToByteArray(const unsigned short * aUINT16, int RegisterOffset, int NumUINT16, unsigned char * aBytes);
LJM_VOID_RETURN LJM_ByteArrayToUINT16(const unsigned char * aBytes, int RegisterOffset, int NumUINT16, unsigned short * aUINT16);
LJM_VOID_RETURN LJM_UINT32ToByteArray(const unsigned int * aUINT32, int RegisterOffset, int NumUINT32, unsigned char * aBytes);
LJM_VOID_RETURN LJM_ByteArrayToUINT32(const unsigned char * aBytes, int RegisterOffset, int NumUINT32, unsigned int * aUINT32);
LJM_VOID_RETURN LJM_INT32ToByteArray(const int * aINT32, int RegisterOffset, int NumINT32, unsigned char * aBytes);
LJM_VOID_RETURN LJM_ByteArrayToINT32(const unsigned char * aBytes, int RegisterOffset, int NumINT32, int * aINT32);
LJM_ERROR_RETURN LJM_NumberToIP(unsigned int Number, char * IPv4String);
LJM_ERROR_RETURN LJM_IPToNumber(const char * IPv4String, unsigned int * Number);
LJM_ERROR_RETURN LJM_NumberToMAC(unsigned long long Number, char * MACString);
LJM_ERROR_RETURN LJM_MACToNumber(const char * MACString, unsigned long long * Number);
LJM_LONG_LONG_RETURN LJM_GetHostTick(void);
LJM_VOID_RETURN LJM_GetHostTick32Bit(unsigned int * TickUpper, unsigned int * TickLower);
LJM_ERROR_RETURN LJM_StartInterval(int IntervalHandle, int Microseconds);
LJM_ERROR_RETURN LJM_WaitForNextInterval(int IntervalHandle, int * SkippedIntervals);
LJM_ERROR_RETURN LJM_CleanInterval(int IntervalHandle);
static const char * const LJM_USB_SEND_RECEIVE_TIMEOUT_MS = "LJM_USB_SEND_RECEIVE_TIMEOUT_MS";
static const char * const LJM_ETHERNET_SEND_RECEIVE_TIMEOUT_MS = "LJM_ETHERNET_SEND_RECEIVE_TIMEOUT_MS";
static const char * const LJM_WIFI_SEND_RECEIVE_TIMEOUT_MS = "LJM_WIFI_SEND_RECEIVE_TIMEOUT_MS";
static const char * const LJM_SEND_RECEIVE_TIMEOUT_MS = "LJM_SEND_RECEIVE_TIMEOUT_MS";
static const char * const LJM_ETHERNET_OPEN_TIMEOUT_MS = "LJM_ETHERNET_OPEN_TIMEOUT_MS";
static const char * const LJM_WIFI_OPEN_TIMEOUT_MS = "LJM_WIFI_OPEN_TIMEOUT_MS";
static const char * const LJM_OPEN_TCP_DEVICE_TIMEOUT_MS = "LJM_OPEN_TCP_DEVICE_TIMEOUT_MS";
static const char * const LJM_DEBUG_LOG_MODE = "LJM_DEBUG_LOG_MODE";
enum {
	LJM_DEBUG_LOG_MODE_NEVER = 1,
	LJM_DEBUG_LOG_MODE_CONTINUOUS = 2,
	LJM_DEBUG_LOG_MODE_ON_ERROR = 3
};
static const char * const LJM_DEBUG_LOG_LEVEL = "LJM_DEBUG_LOG_LEVEL";
enum {
	LJM_STREAM_PACKET = 1,
	LJM_TRACE = 2,
	LJM_DEBUG = 4,
	LJM_INFO = 6,
	LJM_PACKET = 7,
	LJM_WARNING = 8,
	LJM_USER = 9,
	LJM_ERROR = 10,
	LJM_FATAL = 12
};
static const char * const LJM_DEBUG_LOG_BUFFER_MAX_SIZE = "LJM_DEBUG_LOG_BUFFER_MAX_SIZE";
static const char * const LJM_DEBUG_LOG_SLEEP_TIME_MS = "LJM_DEBUG_LOG_SLEEP_TIME_MS";
static const char * const LJM_LIBRARY_VERSION = "LJM_LIBRARY_VERSION";
static const char * const LJM_ALLOWS_AUTO_MULTIPLE_FEEDBACKS = "LJM_ALLOWS_AUTO_MULTIPLE_FEEDBACKS";
static const char * const LJM_ALLOWS_AUTO_CONDENSE_ADDRESSES = "LJM_ALLOWS_AUTO_CONDENSE_ADDRESSES";
static const char * const LJM_AUTO_IPS_FILE = "LJM_AUTO_IPS_FILE";
static const char * const LJM_AUTO_IPS = "LJM_AUTO_IPS";
static const char * const LJM_AUTO_RECONNECT_STICKY_CONNECTION = "LJM_AUTO_RECONNECT_STICKY_CONNECTION";
static const char * const LJM_AUTO_RECONNECT_STICKY_SERIAL = "LJM_AUTO_RECONNECT_STICKY_SERIAL";
static const char * const LJM_AUTO_RECONNECT_WAIT_MS = "LJM_AUTO_RECONNECT_WAIT_MS";
static const char * const LJM_MODBUS_MAP_CONSTANTS_FILE = "LJM_MODBUS_MAP_CONSTANTS_FILE";
static const char * const LJM_ERROR_CONSTANTS_FILE = "LJM_ERROR_CONSTANTS_FILE";
static const char * const LJM_DEBUG_LOG_FILE = "LJM_DEBUG_LOG_FILE";
static const char * const LJM_CONSTANTS_FILE = "LJM_CONSTANTS_FILE";
static const char * const LJM_DEBUG_LOG_FILE_MAX_SIZE = "LJM_DEBUG_LOG_FILE_MAX_SIZE";
static const char * const LJM_DEEP_SEARCH_FILE = "LJM_DEEP_SEARCH_FILE";
static const char * const LJM_SPECIFIC_IPS_FILE = "LJM_SPECIFIC_IPS_FILE";
static const char * const LJM_STREAM_AIN_BINARY = "LJM_STREAM_AIN_BINARY";
static const char * const LJM_STREAM_DIGITAL_AUTO_RECOVERY_ERROR_DETECTION_DISABLED =
	"LJM_STREAM_DIGITAL_AUTO_RECOVERY_ERROR_DETECTION_DISABLED";
static const char * const LJM_STREAM_SCANS_RETURN = "LJM_STREAM_SCANS_RETURN";
enum {
	LJM_STREAM_SCANS_RETURN_ALL = 1,
	LJM_STREAM_SCANS_RETURN_ALL_OR_NONE = 2
};
static const char * const LJM_STREAM_RECEIVE_TIMEOUT_MODE = "LJM_STREAM_RECEIVE_TIMEOUT_MODE";
enum {
	LJM_STREAM_RECEIVE_TIMEOUT_MODE_CALCULATED = 1,
	LJM_STREAM_RECEIVE_TIMEOUT_MODE_MANUAL = 2
};
static const char * const LJM_STREAM_TCP_RECEIVE_BUFFER_SIZE = "LJM_STREAM_TCP_RECEIVE_BUFFER_SIZE";
static const char * const LJM_STREAM_RECEIVE_TIMEOUT_MS = "LJM_STREAM_RECEIVE_TIMEOUT_MS";
static const char * const LJM_STREAM_TRANSFERS_PER_SECOND = "LJM_STREAM_TRANSFERS_PER_SECOND";
static const char * const LJM_RETRY_ON_TRANSACTION_ID_MISMATCH = "LJM_RETRY_ON_TRANSACTION_ID_MISMATCH";
static const char * const LJM_OLD_FIRMWARE_CHECK = "LJM_OLD_FIRMWARE_CHECK";
static const char * const LJM_USE_TCP_INIT_FOR_T7_WIFI_TCP = "LJM_USE_TCP_INIT_FOR_T7_WIFI_TCP";
static const char * const LJM_ZERO_LENGTH_ARRAY_MODE = "LJM_ZERO_LENGTH_ARRAY_MODE";
enum {
	LJM_ZERO_LENGTH_ARRAY_ERROR = 1,
	LJM_ZERO_LENGTH_ARRAY_IGNORE_OPERATION = 2
};
LJM_ERROR_RETURN LJM_WriteLibraryConfigS(const char * Parameter, double Value);
LJM_ERROR_RETURN LJM_WriteLibraryConfigStringS(const char * Parameter,
	const char * String);
LJM_ERROR_RETURN LJM_ReadLibraryConfigS(const char * Parameter, double * Value);
LJM_ERROR_RETURN LJM_ReadLibraryConfigStringS(const char * Parameter, char * String);
LJM_ERROR_RETURN LJM_LoadConfigurationFile(const char * FileName);
LJM_ERROR_RETURN LJM_GetSpecificIPsInfo(int * InfoHandle, const char ** Info);
LJM_ERROR_RETURN LJM_GetDeepSearchInfo(int * InfoHandle, const char ** Info);
LJM_ERROR_RETURN LJM_Log(int Level, const char * String);
LJM_ERROR_RETURN LJM_ResetLog(void);
typedef void (*LJM_DeviceReconnectCallback)(int);
LJM_ERROR_RETURN LJM_RegisterDeviceReconnectCallback(int Handle,
	LJM_DeviceReconnectCallback Callback);
static const int LJM_DEFAULT_PORT = 502;
static const int LJM_UDP_PORT = 52362;
static const int LJM_MAX_TCP_PACKET_NUM_BYTES_T7 = 1040;
static const int LJM_MAX_USB_PACKET_NUM_BYTES = 64;
static const int LJM_MAX_ETHERNET_PACKET_NUM_BYTES_T7 = 1040;
static const int LJM_MAX_WIFI_PACKET_NUM_BYTES_T7 = 500;
LJM_ERROR_CODE LJME_COULD_NOT_CLAIM_DEVICE = 1230;
LJM_ERROR_CODE LJME_U3_CANNOT_BE_OPENED_BY_LJM = 1243;
LJM_ERROR_CODE LJME_U6_CANNOT_BE_OPENED_BY_LJM = 1246;
LJM_ERROR_CODE LJME_UE9_CANNOT_BE_OPENED_BY_LJM = 1249;
LJM_ERROR_CODE LJME_INVALID_VALUE_TYPE = 1259;
static const char * const LJM_SPECIAL_ADDRESSES_FILE = "LJM_SPECIAL_ADDRESSES_FILE";
static const char * const LJM_SPECIAL_ADDRESSES_STATUS = "LJM_SPECIAL_ADDRESSES_STATUS";
static const char * const LJM_OPEN_MODE = "LJM_OPEN_MODE";
enum { LJM_KEEP_OPEN = 1, LJM_OPEN_CLOSE = 2 };
#ifdef __cplusplus
}
#endif
#endif
