use super::protocol::{calculate_crc_bytes, create_request_modbus_header};
use crate::error::JSYMk194Error;
use crate::hal::*;
use crate::jsy_mk_194g::JsyMk194g;
use crate::modbus::protocol::{
    ERROR_RESPONSE_HEADER_SIZE, SINGLE_READ_RESPONSE_HEADER_SIZE, SINGLE_WRITE_REQUEST_HEADER_SIZE,
    construct_single_read_request,
};
use crate::modbus::types::FunctionCode;
use crate::registers::traits::{self, Register};

impl<Serial: Read + Write> JsyMk194g<Serial> {
    #[maybe_async::maybe_async]
    pub async fn read_register<Register>(&mut self) -> Result<Register, JSYMk194Error>
    where
        Register: traits::Register + traits::ReadRegister,
    {
        let buff = construct_single_read_request(
            self.device_address,
            Register::ADDRESS,
            Register::NUM_BYTES,
        )?;
        let bytes_written = self.serial.write(&buff)?;
        let mut response_buff = [0u8; SINGLE_READ_RESPONSE_HEADER_SIZE];
        let bytes_read = self.serial.read(&mut response_buff)?;
        let register_buff = &response_buff[3..(3 + Register::NUM_BYTES)];
        Ok(Register::from_bytes(register_buff))
    }

    #[maybe_async::maybe_async]
    pub async fn write_register(
        &mut self,
        register: impl Register + traits::WriteRegister,
    ) -> Result<(), JSYMk194Error> {
        let address_header = create_request_modbus_header(
            self.device_address,
            FunctionCode::WriteOneOrMoreRegisters,
            register.address(),
        );
        let num_bytes =
            u16::try_from(register.num_bytes()).map_err(|_| JSYMk194Error::ConversionError)?; // Fix `This`
        let [num_bytes_high, num_bytes_low] = num_bytes.to_be_bytes();
        match num_bytes {
            2 => {
                let mut buff = [0u8; SINGLE_WRITE_REQUEST_HEADER_SIZE];
                buff[0..4].copy_from_slice(&address_header);
                buff[4] = num_bytes_high;
                buff[5] = num_bytes_low;
                register.to_bytes(&mut buff[6..8])?;
                let crc = calculate_crc_bytes(&buff[0..8]);
                buff[8..10].copy_from_slice(&crc);
                let bytes_written = self.serial.write(&buff)?;
            }
            4 => {
                let mut buff = [0u8; SINGLE_WRITE_REQUEST_HEADER_SIZE + 2]; // Extra 2 bytes for CRC
                buff[0..4].copy_from_slice(&address_header);
                buff[4] = num_bytes_high;
                buff[5] = num_bytes_low;
                register.to_bytes(&mut buff[6..10])?;
                let crc = calculate_crc_bytes(&buff[0..10]);
                buff[10..12].copy_from_slice(&crc);
                let bytes_written = self.serial.write(&buff)?;
            }
            _ => return Err(JSYMk194Error::ConversionError),
        };
        Ok(())
    }
}
