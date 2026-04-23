// use crate::registers::RegisterAddress;

use crate::error::JSYMk194Error;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum FunctionCode {
    ReadOneOrMoreRegisters = 0x03,
    WriteOneOrMoreRegisters = 0x10,
    ReadOutputStatus = 0x01,
    WriteOutputStatus = 0x05,

    // Acording to the datasheet when an error, or exception happens,
    // the device will respond with the function code with the most significant bit set to 1
    ExceptionReadResponseCode = 0x83,
    ExceptionWriteResponseCode = 0x90,
    ExceptionReadOutputStatusResponseCode = 0x81,
    ExceptionWriteOutputStatusResponseCode = 0x85,
}

impl From<FunctionCode> for u8 {
    fn from(value: FunctionCode) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for FunctionCode {
    type Error = JSYMk194Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x03 => Ok(FunctionCode::ReadOneOrMoreRegisters),
            0x10 => Ok(FunctionCode::WriteOneOrMoreRegisters),
            0x01 => Ok(FunctionCode::ReadOutputStatus),
            0x05 => Ok(FunctionCode::WriteOutputStatus),
            0x83 => Ok(FunctionCode::ExceptionReadResponseCode),
            0x90 => Ok(FunctionCode::ExceptionWriteResponseCode),
            0x81 => Ok(FunctionCode::ExceptionReadOutputStatusResponseCode),
            0x85 => Ok(FunctionCode::ExceptionWriteOutputStatusResponseCode),
            _ => Err(JSYMk194Error::ConversionError(format!(
                "Invalid function code: 0x{:02X}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ErrorCode {
    /// Illegal function code, that is, the received function code module is not supported.
    IllegalFunction = 0x81,
    /// Read or write illegal data addresses, that is, the data location exceeds the readable or writable address range of the module
    IllegalDataAddress = 0x82,
    /// Illegal data value, that is, the data value sent by the host received by the module exceeds the data range of the corresponding address.
    IllegalDataValue = 0x83,
}

impl From<ErrorCode> for u8 {
    fn from(value: ErrorCode) -> Self {
        value as u8
    }
}
impl TryFrom<u8> for ErrorCode {
    type Error = JSYMk194Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x81 => Ok(ErrorCode::IllegalFunction),
            0x82 => Ok(ErrorCode::IllegalDataAddress),
            0x83 => Ok(ErrorCode::IllegalDataValue),
            _ => Err(JSYMk194Error::ConversionError(format!(
                "Invalid error code: 0x{:02X}",
                value
            ))),
        }
    }
}
