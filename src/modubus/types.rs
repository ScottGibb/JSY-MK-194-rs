use crate::registers::RegisterAddress;

pub enum FunctionCode {
    ReadOneOrMoreRegisters = 0x03,
    WriteOneOrMoreRegisters = 0x10,
    ReadOutputStatus = 0x01,
    WriteOutputStatus = 0x05,
}

pub struct DataFrame<'a> {
    pub device_address: RegisterAddress,
    pub function_code: FunctionCode,
    pub data: &'a [u8],
    pub crc: CylicRedundanyCheck,
}

#[derive(Clone, Copy)]
pub struct CylicRedundanyCheck(pub u16);

impl CylicRedundanyCheck {
    pub fn calculate(data: &[u8]) -> Self {
        // Placeholder for CRC calculation logic
        CylicRedundanyCheck(0)
    }
}
