use crate::{
    error::JSYMk194Error,
    modbus::{
        ErrorCode,
        offsets::{
            MODBUS_DEVICE_ADDRESS_OFFSET, MODBUS_ERROR_CODE_OFFSET, MODBUS_FUNCTION_CODE_OFFSET,
        },
        protocol::validate_crc,
        types::FunctionCode,
    },
    registers::RegisterAddress,
    types::Id,
};

pub struct ReadResponse<'a> {
    pub device_address: Id,
    pub function_code: FunctionCode,
    pub byte_count: u8,
    pub register_data: &'a [u8],
    pub crc: u16,
}
impl<'a> ReadResponse<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, JSYMk194Error> {
        if bytes.len() < 5 {
            return Err(JSYMk194Error::FailedToRead {
                read: bytes.len(),
                expected: 5,
            });
        }
        let device_address = Id::new(bytes[0])?;
        let function_code = FunctionCode::try_from(bytes[1])?;
        let byte_count = bytes[2];
        if bytes.len() != (3 + byte_count as usize + 2) {
            // 3 bytes for device address, function code, and byte count, plus byte_count bytes for register data, plus 2 bytes for CRC
            return Err(JSYMk194Error::FailedToRead {
                read: bytes.len(),
                expected: 3 + byte_count as usize + 2,
            });
        }
        let register_data = &bytes[3..(3 + byte_count as usize)];
        let crc = u16::from_le_bytes([
            bytes[3 + byte_count as usize],
            bytes[4 + byte_count as usize],
        ]);
        validate_crc(&bytes[0..(3 + byte_count as usize)], crc)?;
        Ok(Self {
            device_address,
            function_code,
            byte_count,
            register_data,
            crc,
        })
    }
}
// Always 8 Bytes if successful
pub struct WriteResponse {
    pub device_address: Id,
    pub function_code: FunctionCode,
    pub starting_address: RegisterAddress,
    pub quantity_of_registers: u16,
    pub crc: u16,
}

impl WriteResponse {
    pub const RESPONSE_SIZE: usize = 8;

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, JSYMk194Error> {
        if bytes.len() == ModbusErrorResponse::ERROR_RESPONSE_HEADER_SIZE {
            let error_response = ModbusErrorResponse::from_bytes(bytes)?;
            return Err(JSYMk194Error::ModBusDeviceError(error_response));
        }
        if bytes.len() != Self::RESPONSE_SIZE {
            return Err(JSYMk194Error::InvalidResponse);
        }
        let device_address = Id::new(bytes[0])?;
        let function_code = FunctionCode::try_from(bytes[1])?;
        let starting_address = RegisterAddress::try_from(u16::from_be_bytes([bytes[2], bytes[3]]))?;
        let quantity_of_registers = u16::from_be_bytes([bytes[4], bytes[5]]);
        let crc = u16::from_le_bytes([bytes[6], bytes[7]]);
        validate_crc(&bytes[0..6], crc)?;

        Ok(Self {
            device_address,
            function_code,
            starting_address,
            quantity_of_registers,
            crc,
        })
    }
}

#[derive(Debug)]
pub struct ModbusErrorResponse {
    pub id: Id,
    pub function_code: FunctionCode,
    pub error_code: ErrorCode,
    pub crc: u16,
}

impl ModbusErrorResponse {
    pub const ERROR_RESPONSE_HEADER_SIZE: usize = 5;
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, JSYMk194Error> {
        if bytes.len() != Self::ERROR_RESPONSE_HEADER_SIZE {
            return Err(JSYMk194Error::InvalidResponse);
        }
        let crc = u16::from_le_bytes([bytes[bytes.len() - 2], bytes[bytes.len() - 1]]);
        validate_crc(&bytes[0..3], crc)?;

        Ok(Self {
            id: Id::new(bytes[MODBUS_DEVICE_ADDRESS_OFFSET])?,
            function_code: FunctionCode::try_from(bytes[MODBUS_FUNCTION_CODE_OFFSET])?,
            error_code: ErrorCode::try_from(bytes[MODBUS_ERROR_CODE_OFFSET])?,
            crc,
        })
    }
}
