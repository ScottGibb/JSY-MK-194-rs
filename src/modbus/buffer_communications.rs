use crate::error::JSYMk194Error;
use crate::hal::*;
use crate::jsy_mk_194g::JsyMk194g;
use crate::modbus::protocol::extract_modbus_response_header;
use crate::modbus::responses::ModbusErrorResponse;

impl<Serial: ReadWrite, D: DelayNs> JsyMk194g<Serial, D> {
    #[maybe_async::maybe_async]
    pub(crate) async fn write_buffer(&mut self, buffer: &[u8]) -> Result<(), JSYMk194Error> {
        let mut total_written = 0;
        while total_written < buffer.len() {
            let bytes_written = self.serial.write(&buffer[total_written..]).await?;
            if bytes_written == 0 {
                return Err(JSYMk194Error::FailedToWrite {
                    written: total_written,
                    expected: buffer.len(),
                });
            }
            total_written += bytes_written;
        }
        Ok(())
    }
    #[maybe_async::maybe_async]
    pub(crate) async fn read_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, JSYMk194Error> {
        let bytes_read = self.serial.read(buffer).await?;
        // println!(
        //     "[Modbus] Raw response bytes  :  {:02X?}",
        //     &buffer[..bytes_read]
        // );
        if bytes_read == ModbusErrorResponse::RESPONSE_SIZE {
            return Err(JSYMk194Error::ModBusDeviceError(
                ModbusErrorResponse::from_bytes(&buffer[..bytes_read])?,
            ));
        }
        let (_, function_code) = extract_modbus_response_header(&buffer[..bytes_read])?;
        if function_code.is_exception_response() {
            return Err(JSYMk194Error::DeviceErrorResponse(function_code));
        }
        Ok(bytes_read)
    }
}
