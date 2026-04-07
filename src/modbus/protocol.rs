use crate::{error::JSYMk194Error, modbus::types::FunctionCode, registers::RegisterAddress};
const SINGLE_READ_REQUEST_HEADER_SIZE: usize = 8;
pub const SINGLE_READ_RESPONSE_HEADER_SIZE: usize = 7;

pub const SINGLE_WRITE_REQUEST_HEADER_SIZE: usize = 10;
pub const SINGLE_WRITE_RESPONSE_HEADER_SIZE: usize = 8;

pub const ERROR_RESPONSE_HEADER_SIZE: usize = 5;

pub fn create_request_modbus_header(
    device_address: u8,
    function_code: FunctionCode,
    starting_address: RegisterAddress,
) -> [u8; 4] {
    let [starting_address_high, starting_address_low] = u16::from(starting_address).to_be_bytes();
    [
        device_address,
        u8::from(function_code),
        starting_address_high,
        starting_address_low,
    ]
}

pub fn construct_single_read_request(
    device_address: u8,
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
    let num_bytes = u16::try_from(register_size).map_err(|_| JSYMk194Error::ConversionError)?; // Fix `This`
    let [num_bytes_high, num_bytes_low] = num_bytes.to_be_bytes();
    buff[4] = num_bytes_high;
    buff[5] = num_bytes_low;
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
