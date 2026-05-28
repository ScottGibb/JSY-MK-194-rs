use crate::error::JSYMk194Error;
use crate::jsy_mk_194g::JsyMk194g;
use crate::modbus::protocol::extract_modbus_response_header;
use crate::{ModbusErrorResponse, hal::*};

impl<Serial: ReadWrite, D: DelayNs> JsyMk194g<Serial, D> {
    #[maybe_async::maybe_async]
    pub(crate) async fn write_buffer(&mut self, buffer: &[u8]) -> Result<(), JSYMk194Error> {
        self.serial
            .write_all(&buffer)
            .await
            .map_err(|e| JSYMk194Error::Io(e.kind()))?;
        Ok(())
    }
    #[maybe_async::maybe_async]
    pub(crate) async fn read_buffer(&mut self, buffer: &mut [u8]) -> Result<(), JSYMk194Error> {
        // Read the minimum number of bytes required to determine if the response is an error response or a normal response
        self.serial
            .read_exact(&mut buffer[..ModbusErrorResponse::RESPONSE_SIZE])
            .await?;
        // Check if the response is an error response based on the function code in the header
        let (_, function_code) =
            extract_modbus_response_header(&buffer[..ModbusErrorResponse::RESPONSE_SIZE])?;
        if function_code.is_exception_response() {
            return Err(JSYMk194Error::DeviceErrorResponse(function_code));
        }
        // Now read the rest of the response based on the expected length for a normal response. This will read the remaining bytes for a normal response, or read extra bytes that can be ignored for an error response (since we've already determined it's an error response based on the function code).
        self.serial
            .read_exact(&mut buffer[ModbusErrorResponse::RESPONSE_SIZE..])
            .await?;

        Ok(())
    }
}
