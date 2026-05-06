use crate::error::JSYMk194Error;
use crate::hal::*;
use crate::jsy_mk_194g::JsyMk194g;
use crate::modbus::protocol::REQUEST_RESPONSE_DELAY;
use crate::modbus::requests::{ReadRequest, WriteRequest};
use crate::modbus::responses::{ReadResponse, WriteResponse};
use crate::registers::traits::{self, Register};

impl<Serial: Read + Write, D: DelayNs> JsyMk194g<Serial, D> {
    #[maybe_async::maybe_async]
    pub async fn read_register<Register>(&mut self) -> Result<Register, JSYMk194Error>
    where
        Register: traits::Register + traits::ReadRegister,
    {
        let read_request = ReadRequest::new(self.device_address.clone(), Register::ADDRESS, 1);
        let buff = read_request.to_bytes();
        self.write_buffer(&buff).await?;
        self.delay
            .delay_ms(
                u32::try_from(REQUEST_RESPONSE_DELAY.as_millis())
                    .expect("This should not fail to convert"),
            )
            .await;
        let mut response_buff = [0u8; 256];
        let bytes_read = self.read_buffer(&mut response_buff).await?;
        let read_response = ReadResponse::from_bytes(&response_buff[..bytes_read])?;
        Ok(Register::from_bytes(read_response.register_data))
    }

    #[maybe_async::maybe_async]
    pub async fn write_register(
        &mut self,
        register: impl Register + traits::WriteRegister,
    ) -> Result<(), JSYMk194Error> {
        match register.num_bytes() {
            2 => {
                let mut register_data = [0u8; 2];
                register.to_bytes(&mut register_data)?;
                let write_request = WriteRequest::new(
                    self.device_address.clone(),
                    register.address(),
                    &register_data,
                )?;
                let mut write_request_buffer = [0u8; 11];
                write_request.to_bytes(&mut write_request_buffer)?;
                self.write_buffer(&write_request_buffer).await?;
            }
            4 => {
                let mut register_data = [0u8; 4];
                register.to_bytes(&mut register_data)?;
                let write_request = WriteRequest::new(
                    self.device_address.clone(),
                    register.address(),
                    &register_data,
                )?;
                let mut write_request_buffer = [0u8; 13]; // 2 extra bytes for the additional register data
                write_request.to_bytes(&mut write_request_buffer)?;
                self.write_buffer(&write_request_buffer).await?;
            }
            _ => {
                return Err(JSYMk194Error::ConversionError(
                    "Invalid register size: must be 2 or 4 bytes".into(),
                ));
            }
        }

        self.delay
            .delay_ms(
                u32::try_from(REQUEST_RESPONSE_DELAY.as_millis())
                    .expect("This should not fail to convert"),
            )
            .await;
        let mut response_buff = [0u8; 256]; // Error response is smaller than normal response, so this will work for both
        let bytes_read = self.read_buffer(&mut response_buff).await?;
        let _write_response = WriteResponse::from_bytes(&response_buff[..bytes_read])?;
        Ok(())
    }
}
