use crate::{
    error::JSYMk194Error,
    modbus::{
        ErrorCode,
        constants::{
            MODBUS_DEVICE_ADDRESS_OFFSET, MODBUS_ERROR_CODE_OFFSET, MODBUS_FUNCTION_CODE_OFFSET,
        },
        protocol::validate_crc,
        types::FunctionCode,
    },
    registers::RegisterAddress,
    types::Id,
};

pub struct ReadResponse<'a> {
    pub _device_address: Id,
    pub _function_code: FunctionCode,
    pub byte_count: u8,
    pub register_data: &'a [u8],
    pub _crc: u16,
}
impl<'a> ReadResponse<'a> {
    const FRONT_HEADER_SIZE: usize = 3; // Device address, function code, and byte count
    pub const RESPONSE_SIZE: usize = 5; // Device address, function code, byte count, and CRC
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, JSYMk194Error> {
        if bytes.len() < Self::RESPONSE_SIZE {
            return Err(JSYMk194Error::FailedToRead {
                read: bytes.len(),
                expected: Self::RESPONSE_SIZE,
            });
        }
        let device_address = Id::new(bytes[0])?;
        let function_code = FunctionCode::try_from(bytes[1])?;
        let byte_count = usize::from(bytes[2]);
        if bytes.len() != (Self::FRONT_HEADER_SIZE + byte_count + 2) {
            // 3 bytes for device address, function code, and byte count, plus byte_count bytes for register data, plus 2 bytes for CRC
            return Err(JSYMk194Error::FailedToRead {
                read: bytes.len(),
                expected: Self::FRONT_HEADER_SIZE + byte_count + 2,
            });
        }
        let register_data = &bytes[Self::FRONT_HEADER_SIZE..(Self::FRONT_HEADER_SIZE + byte_count)];
        let crc = u16::from_le_bytes([
            bytes[Self::FRONT_HEADER_SIZE + byte_count],
            bytes[Self::FRONT_HEADER_SIZE + byte_count + 1],
        ]);
        validate_crc(&bytes[0..(Self::FRONT_HEADER_SIZE + byte_count)], crc)?;
        Ok(Self {
            _device_address: device_address,
            _function_code: function_code,
            byte_count: bytes[2],
            register_data,
            _crc: crc,
        })
    }
}
// Always 8 Bytes if successful
pub struct WriteResponse {
    pub _device_address: Id,
    pub _function_code: FunctionCode,
    pub _starting_address: RegisterAddress,
    pub _quantity_of_registers: u16,
    pub _crc: u16,
}

impl WriteResponse {
    pub const RESPONSE_SIZE: usize = 8;

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, JSYMk194Error> {
        if bytes.len() == ModbusErrorResponse::RESPONSE_SIZE {
            let error_response = ModbusErrorResponse::from_bytes(bytes)?;
            return Err(JSYMk194Error::ModBusDeviceError(error_response));
        }
        if bytes.len() != Self::RESPONSE_SIZE {
            return Err(JSYMk194Error::FailedToRead {
                read: bytes.len(),
                expected: Self::RESPONSE_SIZE,
            });
        }
        let device_address = Id::new(bytes[0])?;
        let function_code = FunctionCode::try_from(bytes[1])?;
        let starting_address = RegisterAddress::try_from(u16::from_be_bytes([bytes[2], bytes[3]]))?;
        let quantity_of_registers = u16::from_be_bytes([bytes[4], bytes[5]]);
        let crc = u16::from_le_bytes([bytes[6], bytes[7]]);
        validate_crc(&bytes[0..6], crc)?;

        Ok(Self {
            _device_address: device_address,
            _function_code: function_code,
            _starting_address: starting_address,
            _quantity_of_registers: quantity_of_registers,
            _crc: crc,
        })
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Represents a Modbus exception response returned by the device when a protocol error occurs.
pub struct ModbusErrorResponse {
    /// Device ID that the error response is associated with.
    pub id: Id,
    /// Function code that was attempted when the error occurred. This can be used to identify which operation caused the error.
    pub function_code: FunctionCode,
    /// Specific error code returned by the device, indicating the type of protocol error that occurred.
    pub error_code: ErrorCode,
    /// CRC value for the error response, used to verify data integrity.
    pub crc: u16,
}

impl ModbusErrorResponse {
    /// The expected size of a Modbus error response packet
    pub const RESPONSE_SIZE: usize = 5;
    /// Parses a Modbus error response from a byte slice, validating the CRC and extracting the device ID, function code, and error code.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, JSYMk194Error> {
        if bytes.len() != Self::RESPONSE_SIZE {
            return Err(JSYMk194Error::InvalidHeader);
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
