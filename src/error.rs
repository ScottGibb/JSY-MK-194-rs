use crate::hal;
use crate::modbus::ModbusErrorResponse;
use crate::modbus::types::FunctionCode;
#[derive(Debug)]
#[non_exhaustive]
pub enum JSYMk194Error {
    /// An error occurred during I/O operations, such as reading from or writing to the device.
    Io(hal::ErrorKind),
    /// The device returned an unexpected response or data format.
    InvalidResponse,
    /// The Write failded to write the expected number of bytes to the device.
    FailedToWrite {
        written: usize,
        expected: usize,
    },
    /// The Read failed to read the expected number of bytes from the device.
    FailedToRead {
        read: usize,
        expected: usize,
    },
    /// An error occurred during a conversion process, this could mean data is corrupted, or this library has
    /// not implemented the correct conversion for a specific type. That type should then be seen in the error string.
    ConversionError(String),
    /// The CRC check failed, indicating that the data received from the device may be corrupted or tampered with.
    CrcError {
        actual: u16,
        expected: u16,
    },
    /// The device responded with an error function code, indicating that the requested operation could not be completed successfully.
    DeviceErrorResponse(FunctionCode),

    ModBusDeviceError(ModbusErrorResponse),
}

impl<E: hal::Error> From<E> for JSYMk194Error {
    fn from(e: E) -> Self {
        JSYMk194Error::Io(e.kind())
    }
}
