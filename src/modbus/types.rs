// use crate::registers::RegisterAddress;

use crate::error::JSYMk194Error;

#[derive(Debug, Clone)]
pub enum FunctionCode {
    ReadOneOrMoreRegisters = 0x03,
    WriteOneOrMoreRegisters = 0x10,
    ReadOutputStatus = 0x01,
    WriteOutputStatus = 0x05,
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
            _ => Err(JSYMk194Error::ConversionError(format!(
                "Invalid function code: 0x{:02X}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ErrorCode {
    IllegalFunction = 0x81,
    IllegalDataAddress = 0x82,
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
