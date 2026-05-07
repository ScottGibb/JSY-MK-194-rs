use core::time::Duration;

use crate::{
    error::JSYMk194Error,
    modbus::{
        constants::{MODBUS_DEVICE_ADDRESS_OFFSET, MODBUS_FUNCTION_CODE_OFFSET},
        types::FunctionCode,
    },
    registers::system_configuration_parameter::Id,
};

pub const REQUEST_RESPONSE_DELAY: Duration = Duration::from_millis(400);
pub const CHANNEL_REQUEST_RESPONSE_DELAY: Duration = Duration::from_millis(800);
// Verify that REQUEST_RESPONSE_DELAY can fit in a u32 when converted to milliseconds, since that's the type used in the driver implementation. This is important to prevent overflow issues when converting the duration to milliseconds.
const _: () = assert!(
    REQUEST_RESPONSE_DELAY.as_millis() <= u32::MAX as u128,
    "REQUEST_RESPONSE_DELAY must be less than or equal to u32::MAX milliseconds"
);

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

pub fn validate_crc(data: &[u8], crc: u16) -> Result<(), JSYMk194Error> {
    let calculated_crc = calculate_crc(data);
    if calculated_crc != crc {
        return Err(JSYMk194Error::CrcError {
            actual: crc,
            expected: calculated_crc,
        });
    }
    Ok(())
}

pub fn extract_modbus_response_header(buffer: &[u8]) -> Result<(Id, FunctionCode), JSYMk194Error> {
    if buffer.len() < 2 {
        return Err(JSYMk194Error::InvalidHeader);
    }
    let id = Id::new(buffer[MODBUS_DEVICE_ADDRESS_OFFSET])?;
    let function_code = FunctionCode::try_from(buffer[MODBUS_FUNCTION_CODE_OFFSET])?;
    Ok((id, function_code))
}
