use crate::error::JSYMk194Error;
use crate::hal::*;
use crate::jsy_mk_194g::JsyMk194g;
use crate::modbus::protocol::extract_modbus_response_header;
use crate::modbus::responses::ModbusErrorResponse;

impl<Serial: Read + Write, D: DelayNs> JsyMk194g<Serial, D> {
    #[maybe_async::maybe_async]
    pub async fn write_buffer(&mut self, buffer: &[u8]) -> Result<(), JSYMk194Error> {
        let bytes_written = self.serial.write(buffer).await?;
        println!(
            "[Modbus] Raw request bytes   :  {:02X?}",
            &buffer[..bytes_written]
        );
        if bytes_written < buffer.len() {
            return Err(JSYMk194Error::FailedToWrite {
                written: bytes_written,
                expected: buffer.len(),
            });
        }
        Ok(())
    }
    #[maybe_async::maybe_async]
    pub async fn read_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, JSYMk194Error> {
        let bytes_read = self.serial.read(buffer).await?;
        println!(
            "[Modbus] Raw response bytes  :  {:02X?}",
            &buffer[..bytes_read]
        );
        if bytes_read == ModbusErrorResponse::ERROR_RESPONSE_HEADER_SIZE {
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
