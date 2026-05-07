use crate::{
    error::{ConversionError, JSYMk194Error},
    modbus::{protocol::calculate_crc, types::FunctionCode},
    registers::RegisterAddress,
    types::Id,
};

pub struct ReadRequest {
    device_address: Id,
    function_code: FunctionCode,
    starting_address: RegisterAddress,
    quantity_of_registers: u16,
    crc: u16,
}
impl ReadRequest {
    const REQUEST_SIZE: usize = 8; // Device address, function code, starting address (2 bytes), quantity of registers (2 bytes), and CRC (2 bytes)
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
    pub fn to_bytes(&self) -> [u8; Self::REQUEST_SIZE] {
        let mut buff = [0u8; Self::REQUEST_SIZE];
        buff[0] = self.device_address.clone().into();
        buff[1] = self.function_code.clone().into();
        buff[2..4].copy_from_slice(&u16::from(self.starting_address.clone()).to_be_bytes());
        buff[4..6].copy_from_slice(&self.quantity_of_registers.to_be_bytes());
        buff[6..8].copy_from_slice(&self.crc.to_le_bytes());
        buff
    }
}

pub struct WriteRequest<'a> {
    device_address: Id,
    function_code: FunctionCode,
    starting_address: RegisterAddress,
    quantity_of_registers: u16,
    byte_count: u8,
    register_data: &'a [u8],
    _crc: u16,
}

impl<'a> WriteRequest<'a> {
    const REQUEST_SIZE: usize = 7; // Device address, function code, starting address (2 bytes), quantity of registers (2 bytes), and byte count
    const HEADER_SIZE: usize = Self::REQUEST_SIZE + 2; // Device address, function code, starting address (2 bytes), quantity of registers (2 bytes), and byte count plus CRC (2 bytes)
    pub fn new(
        device_address: Id,
        starting_address: RegisterAddress,
        register_data: &'a [u8],
    ) -> Result<Self, JSYMk194Error> {
        let function_code = FunctionCode::WriteOneOrMoreRegisters;
        let starting_address_bytes = u16::from(starting_address.clone()).to_be_bytes();
        if register_data.len() % 2 != 0 {
            return Err(JSYMk194Error::ConversionError(
                ConversionError::InvalidRegisterDataLength {
                    given_length: register_data.len(),
                    address: starting_address.clone(),
                },
            ));
        }
        let byte_count = u8::try_from(register_data.len()).map_err(|err| {
            JSYMk194Error::ConversionError(ConversionError::InvalidByteCount(err))
        })?;
        let quantity_of_registers = u16::try_from(register_data.len() / 2).map_err(|err| {
            JSYMk194Error::ConversionError(ConversionError::InvalidQuantityOfRegisters(err))
        })?;
        let quantity_of_registers_bytes = quantity_of_registers.to_be_bytes();

        Ok(Self {
            device_address: device_address.clone(),
            function_code: function_code.clone(),
            starting_address,
            quantity_of_registers,
            byte_count,
            register_data,
            _crc: calculate_crc(&[
                device_address.into(),
                function_code.into(),
                starting_address_bytes[0],
                starting_address_bytes[1],
                quantity_of_registers_bytes[0],
                quantity_of_registers_bytes[1],
                byte_count,
            ]),
        })
    }
    pub fn to_bytes(&self, buff: &mut [u8]) -> Result<(), JSYMk194Error> {
        if buff.len() < (Self::HEADER_SIZE + self.register_data.len()) {
            return Err(JSYMk194Error::ConversionError(
                ConversionError::InvalidRegisterDataLength {
                    given_length: buff.len(),
                    address: self.starting_address.clone(),
                },
            ));
        }
        buff[0] = self.device_address.clone().into();
        buff[1] = self.function_code.clone().into();
        buff[2..4].copy_from_slice(&u16::from(self.starting_address.clone()).to_be_bytes());
        buff[4..6].copy_from_slice(&self.quantity_of_registers.to_be_bytes());
        buff[6] = self.byte_count;
        buff[7..(Self::REQUEST_SIZE + self.register_data.len())]
            .copy_from_slice(self.register_data);
        let crc = calculate_crc(&buff[0..(Self::REQUEST_SIZE + self.register_data.len())]);
        buff[(Self::REQUEST_SIZE + self.register_data.len())
            ..(Self::REQUEST_SIZE + self.register_data.len() + 2)]
            .copy_from_slice(&crc.to_le_bytes());
        Ok(())
    }
}
