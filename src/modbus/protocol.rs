use core::time::Duration;

use crate::{
    error::JSYMk194Error,
    modbus::{
        ErrorCode,
        offsets::{
            CHANNEL_READ_REQUEST_HEADER_SIZE, FULL_READ_REQUEST_HEADER_SIZE,
            MODBUS_DATA_START_OFFSET, MODBUS_DEVICE_ADDRESS_OFFSET, MODBUS_ERROR_CODE_OFFSET,
            MODBUS_FUNCTION_CODE_OFFSET, NUM_CHANNEL_REGISTERS, SINGLE_READ_REQUEST_HEADER_SIZE,
        },
        types::FunctionCode,
    },
    registers::{
        RegisterAddress, channel_one_measuring_electrical_paramaters::FirstChannelVoltageRegister,
        channel_two_measuring_electrical_paramaters::SecondChannelVoltageRegister,
        system_configuration_paramater::Id, traits::Register,
    },
    types::Channel,
};

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

pub fn calculate_crc(data: &[u8]) -> u16 {
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

pub fn extract_modbus_response_header(buffer: &[u8]) -> Result<(Id, FunctionCode), JSYMk194Error> {
    if buffer.len() < 2 {
        return Err(JSYMk194Error::InvalidResponse);
    }
    let id = Id::new(buffer[MODBUS_DEVICE_ADDRESS_OFFSET])?;
    let function_code = FunctionCode::try_from(buffer[MODBUS_FUNCTION_CODE_OFFSET])?;
    Ok((id, function_code))
}
