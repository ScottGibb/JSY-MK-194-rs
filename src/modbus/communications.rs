use super::protocol::{calculate_crc_bytes, create_request_modbus_header};
use crate::error::JSYMk194Error;
use crate::hal::*;
use crate::jsy_mk_194g::JsyMk194g;
use crate::modbus::protocol::{
    ModbusErrorResponse, REQUEST_RESPONSE_DELAY, SINGLE_READ_RESPONSE_HEADER_SIZE,
    SINGLE_WRITE_REQUEST_HEADER_SIZE, SINGLE_WRITE_RESPONSE_HEADER_SIZE,
    construct_single_read_request,
};
use crate::modbus::types::FunctionCode;
use crate::registers::traits::{self, Register};

impl<Serial: Read + Write, D: DelayNs> JsyMk194g<Serial, D> {
    #[maybe_async::maybe_async]
    pub async fn read_register<Register>(&mut self) -> Result<Register, JSYMk194Error>
    where
        Register: traits::Register + traits::ReadRegister,
    {
        let buff = construct_single_read_request(
            self.device_address.clone(),
            Register::ADDRESS,
            Register::NUM_BYTES,
        )?;
        self.write_buffer(&buff).await?;
        self.delay
            .delay_ms(REQUEST_RESPONSE_DELAY.as_millis() as u32)
            .await;
        let mut response_buff = [0u8; SINGLE_READ_RESPONSE_HEADER_SIZE];
        self.read_buffer(&mut response_buff).await?;

        let register_buff = &response_buff[3..(3 + Register::NUM_BYTES)];
        Ok(Register::from_bytes(register_buff))
    }

    #[maybe_async::maybe_async]
    pub async fn write_register(
        &mut self,
        register: impl Register + traits::WriteRegister,
    ) -> Result<(), JSYMk194Error> {
        let address_header = create_request_modbus_header(
            self.device_address.clone(),
            FunctionCode::WriteOneOrMoreRegisters,
            register.address(),
        );
        let num_bytes = u16::try_from(register.num_bytes())
            .map_err(|_| JSYMk194Error::ConversionError("Invalid register size".into()))?; // Fix `This`
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
                self.write_buffer(&buff).await?;
            }
            4 => {
                let mut buff = [0u8; SINGLE_WRITE_REQUEST_HEADER_SIZE + 2];
                // Extra 2 bytes for CRC
                buff[0..4].copy_from_slice(&address_header);
                buff[4] = num_bytes_high;
                buff[5] = num_bytes_low;
                register.to_bytes(&mut buff[6..10])?;
                let crc = calculate_crc_bytes(&buff[0..10]);
                buff[10..12].copy_from_slice(&crc);
                self.write_buffer(&buff).await?;
            }
            _ => {
                return Err(JSYMk194Error::ConversionError(
                    "Invalid register size".into(),
                ));
            }
        };
        self.delay
            .delay_ms(REQUEST_RESPONSE_DELAY.as_millis() as u32)
            .await;
        let mut response_buff = [0u8; SINGLE_WRITE_RESPONSE_HEADER_SIZE]; // Error response is smaller than normal response, so this will work for both
        self.read_buffer(&mut response_buff).await?;
        Ok(())
    }

    #[maybe_async::maybe_async]
    pub async fn write_buffer(&mut self, buffer: &[u8]) -> Result<(), JSYMk194Error> {
        let bytes_written = self.serial.write(buffer).await?;
        if bytes_written < buffer.len() {
            return Err(JSYMk194Error::FailedToWrite(bytes_written));
        }
        Ok(())
    }
    #[maybe_async::maybe_async]
    pub async fn read_buffer(&mut self, buffer: &mut [u8]) -> Result<(), JSYMk194Error> {
        let bytes_read = self.serial.read(buffer).await?;
        if bytes_read == ModbusErrorResponse::ERROR_RESPONSE_HEADER_SIZE {
            let error_code = ModbusErrorResponse::from_bytes(
                &buffer[..ModbusErrorResponse::ERROR_RESPONSE_HEADER_SIZE],
            )?
            .error_code;
            return Err(JSYMk194Error::DeviceError(error_code));
        }
        if bytes_read < buffer.len() {
            return Err(JSYMk194Error::FailedToRead(bytes_read));
        }
        Ok(())
    }
}
