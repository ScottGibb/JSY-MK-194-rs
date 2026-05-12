use core::time::Duration;

use crate::{
    error::JSYMk194Error,
    modbus::{
        constants::{MODBUS_DEVICE_ADDRESS_OFFSET, MODBUS_FUNCTION_CODE_OFFSET},
        types::FunctionCode,
    },
    registers::system_configuration_parameter::Id,
};

/// The JSY-MK-194 device has a minimum response time, this is used to ensure we wait long enoough,
/// that the device can give us the correct information without erroring.
pub const DEFAULT_REQUEST_RESPONSE_DELAY: Duration = Duration::from_millis(400);
// When requesting channel data, the device takes longer to respond, so we need to wait a bit longer.
pub const DEFAULT_CHANNEL_REQUEST_RESPONSE_DELAY: Duration = Duration::from_millis(800);

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
