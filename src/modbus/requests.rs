use crate::{
    modbus::{
        protocol::{calculate_crc, calculate_crc_bytes},
        types::FunctionCode,
    },
    registers::RegisterAddress,
    types::Id,
};

// Always 8 Bytes
struct ReadRequest {
    device_address: Id,
    function_code: FunctionCode,
    starting_address: RegisterAddress,
    quantity_of_registers: u16,
    crc: u16,
}
impl ReadRequest {
    pub fn new(
        device_address: Id,
        starting_address: RegisterAddress,
        quantity_of_registers: u16,
    ) -> Self {
        let function_code = FunctionCode::ReadOneOrMoreRegisters;
        let starting_address_bytes = u16::from(starting_address.clone()).to_be_bytes();
        let quantity_of_registers_bytes = quantity_of_registers.to_be_bytes();
        let mut buff = [0u8; 6];
        buff[0] = device_address.clone().into();
        buff[1] = function_code.clone().into();
        buff[2..4].copy_from_slice(&starting_address_bytes);
        buff[4..6].copy_from_slice(&quantity_of_registers_bytes);
        let crc = calculate_crc(&buff);
        Self {
            device_address,
            function_code,
            starting_address,
            quantity_of_registers,
            crc,
        }
    }
    pub fn to_bytes(&self) -> [u8; 8] {
        let mut buff = [0u8; 8];
        buff[0] = self.device_address.clone().into();
        buff[1] = self.function_code.clone().into();
        buff[2..4].copy_from_slice(&u16::from(self.starting_address.clone()).to_be_bytes());
        buff[4..6].copy_from_slice(&self.quantity_of_registers.to_be_bytes());
        buff[6..8].copy_from_slice(&self.crc.to_le_bytes());
        buff
    }
}

struct WriteRequest<'a> {
    device_address: Id,
    function_code: FunctionCode,
    starting_address: [u8; 2],
    quantity_of_registers: [u8; 2],
    byte_count: u8,
    register_data: &'a [u8],
    crc: [u8; 2],
}
// TODO: Implement a builder type pattern to add registers on.
