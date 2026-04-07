// use crate::registers::RegisterAddress;

#[derive(Debug, Clone)]
#[repr(u8)]
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
