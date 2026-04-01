use crate::registers::RegisterAddress;
use arbitrary_int::u48;
use bitbybit::{bitenum, bitfield};

#[derive(Debug)]
#[bitenum(u8, exhaustive = false)]
pub enum FunctionCode {
    ReadOneOrMoreRegisters = 0x03,
    WriteOneOrMoreRegisters = 0x10,
    ReadOutputStatus = 0x01,
    WriteOutputStatus = 0x05,
}

#[derive(Debug)]
pub struct WriteDataFrame<'a> {
    pub device_address: RegisterAddress,
    pub function_code: FunctionCode,
    pub data: &'a [u8],
    pub crc: CylicRedundanyCheck,
}

pub struct ReadDataFrame {
    pub device_address: RegisterAddress,
    pub function_code: FunctionCode,
    pub num_bytes: u8,
    pub crc: CylicRedundanyCheck,
}

#[derive(Debug)]
pub struct CylicRedundanyCheck(pub u16);

impl CylicRedundanyCheck {
    pub fn calculate(data: &[u8]) -> Self {
        // Placeholder for CRC calculation logic
        CylicRedundanyCheck(0)
    }
}
