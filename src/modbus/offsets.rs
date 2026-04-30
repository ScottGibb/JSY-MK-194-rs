// Modbus RTU header/data offsets
pub const MODBUS_DEVICE_ADDRESS_OFFSET: usize = 0;
pub const MODBUS_FUNCTION_CODE_OFFSET: usize = 1;
// pub const MODBUS_BYTE_COUNT_OFFSET: usize = 2;
pub const MODBUS_ERROR_CODE_OFFSET: usize = 2;
pub const MODBUS_DATA_START_OFFSET: usize = 3;

pub use crate::registers::electrical_paramater_registers::ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES;

pub const SINGLE_READ_REQUEST_HEADER_SIZE: usize = 8;
pub const SINGLE_READ_RESPONSE_HEADER_SIZE: usize = 7;

pub const SINGLE_WRITE_REQUEST_HEADER_SIZE: usize = 10;
pub const SINGLE_WRITE_RESPONSE_HEADER_SIZE: usize = 8;

pub const FULL_READ_REQUEST_HEADER_SIZE: usize = 8;
// 1 byte for device address, 1 byte for function code, 1 byte for byte count,
// 14 registers * 4 bytes each = 56 bytes for register data, and 2 bytes for CRC.
pub const FULL_READ_RESPONSE_HEADER_SIZE: usize = 61;

pub const CHANNEL_READ_REQUEST_HEADER_SIZE: usize = 8;

// 1 byte for device address, 1 byte for function code, 1 byte for byte count,
// 7 registers * 4 bytes each = 28 bytes for register data, and 2 bytes for CRC.
pub const CHANNEL_READ_RESPONSE_HEADER_SIZE: usize = 33;
pub const NUM_CHANNEL_REGISTERS: usize = 7;
