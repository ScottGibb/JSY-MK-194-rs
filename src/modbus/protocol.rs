use core::time::Duration;

use crate::{
    error::JSYMk194Error,
    modbus::{ErrorCode, types::FunctionCode},
    registers::{
        RegisterAddress, channel_one_measuring_electrical_paramaters::FirstChannelVoltageRegister,
        channel_two_measuring_electrical_paramaters::SecondChannelVoltageRegister,
        system_configuration_paramater::Id, traits::Register,
    },
    types::Channel,
};
const SINGLE_READ_REQUEST_HEADER_SIZE: usize = 8;
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

pub const REQUEST_RESPONSE_DELAY: Duration = Duration::from_millis(100);
// Verify that REQUEST_RESPONSE_DELAY can fit in a u32 when converted to milliseconds, since that's the type used in the driver implementation. This is important to prevent overflow issues when converting the duration to milliseconds.
const _: () = assert!(
    REQUEST_RESPONSE_DELAY.as_millis() <= u32::MAX as u128,
    "REQUEST_RESPONSE_DELAY must be less than or equal to u32::MAX milliseconds"
);
pub fn create_request_modbus_header(
    device_address: Id,
    function_code: FunctionCode,
    starting_address: RegisterAddress,
) -> [u8; 4] {
    let [starting_address_high, starting_address_low] = u16::from(starting_address).to_be_bytes();
    [
        u8::from(device_address),
        u8::from(function_code),
        starting_address_high,
        starting_address_low,
    ]
}

pub fn construct_single_read_request(
    device_address: Id,
    register_address: RegisterAddress,
    register_size: usize,
) -> Result<[u8; SINGLE_READ_REQUEST_HEADER_SIZE], JSYMk194Error> {
    let mut buff = [0u8; SINGLE_READ_REQUEST_HEADER_SIZE];
    let header = create_request_modbus_header(
        device_address,
        FunctionCode::ReadOneOrMoreRegisters,
        register_address,
    );
    buff[0..4].copy_from_slice(&header);
    if register_size % 2 != 0 {
        return Err(JSYMk194Error::ConversionError(
            "Invalid register size: must be a multiple of 2 bytes".into(),
        ));
    }

    let num_registers = u16::try_from(register_size / 2)
        .map_err(|_| JSYMk194Error::ConversionError("Invalid register size".into()))?;
    let [num_registers_high, num_registers_low] = num_registers.to_be_bytes();
    buff[4] = num_registers_high;
    buff[5] = num_registers_low;
    let crc = calculate_crc_bytes(&buff[0..6]);
    buff[6] = crc[0];
    buff[7] = crc[1];
    Ok(buff)
}

pub fn construct_channel_read_request(
    device_address: Id,
    channel: Channel,
) -> Result<[u8; CHANNEL_READ_REQUEST_HEADER_SIZE], JSYMk194Error> {
    let mut buff = [0u8; CHANNEL_READ_REQUEST_HEADER_SIZE];
    let header = create_request_modbus_header(
        device_address,
        FunctionCode::ReadOneOrMoreRegisters,
        if channel == Channel::One {
            FirstChannelVoltageRegister::ADDRESS
        } else {
            SecondChannelVoltageRegister::ADDRESS
        },
    );
    buff[0..4].copy_from_slice(&header);
    let num_registers: u16 = NUM_CHANNEL_REGISTERS as u16;
    let [num_registers_high, num_registers_low] = num_registers.to_be_bytes();
    buff[4] = num_registers_high;
    buff[5] = num_registers_low;
    let crc = calculate_crc_bytes(&buff[0..6]);
    buff[6] = crc[0];
    buff[7] = crc[1];
    Ok(buff)
}

pub fn construct_full_read_request(
    device_address: Id,
) -> Result<[u8; FULL_READ_REQUEST_HEADER_SIZE], JSYMk194Error> {
    let mut buff = [0u8; FULL_READ_REQUEST_HEADER_SIZE];
    let header = create_request_modbus_header(
        device_address,
        FunctionCode::ReadOneOrMoreRegisters,
        FirstChannelVoltageRegister::ADDRESS,
    );
    buff[0..4].copy_from_slice(&header);
    let num_registers: u16 = 14;
    let [num_registers_high, num_registers_low] = num_registers.to_be_bytes();
    buff[4] = num_registers_high;
    buff[5] = num_registers_low;
    let crc = calculate_crc_bytes(&buff[0..6]);
    buff[6] = crc[0];
    buff[7] = crc[1];
    Ok(buff)
}

fn calculate_crc(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &byte in data {
        crc ^= byte as u16;
        for _ in 0..8 {
            if crc & 0x0001 != 0 {
                crc >>= 1;
                crc ^= 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }
    crc
}

pub fn calculate_crc_bytes(data: &[u8]) -> [u8; 2] {
    let crc = calculate_crc(data);
    crc.to_le_bytes()
}

pub struct ModbusErrorResponse {
    pub id: Id,
    pub function_code: FunctionCode,
    pub error_code: ErrorCode,
}

impl ModbusErrorResponse {
    pub const ERROR_RESPONSE_HEADER_SIZE: usize = 5;
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, JSYMk194Error> {
        if bytes.len() != Self::ERROR_RESPONSE_HEADER_SIZE {
            return Err(JSYMk194Error::InvalidResponse);
        }

        Ok(Self {
            id: Id::new(bytes[0])?,
            function_code: FunctionCode::try_from(bytes[1])?,
            error_code: ErrorCode::try_from(bytes[2])?,
        })
    }
}
