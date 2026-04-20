use crate::{hal, modbus::ErrorCode};

#[derive(Debug)]
pub enum JSYMk194Error {
    /// An error occurred during I/O operations, such as reading from or writing to the device.
    Io(hal::ErrorKind),
    /// The device returned an unexpected response or data format.
    InvalidResponse,
    /// The Write failded to write the expected number of bytes to the device. The usize value indicates the number of bytes that were actually written.
    FailedToWrite(usize),
    /// The Read failed to read the expected number of bytes from the device. The usize value indicates the number of bytes that were actually read.
    FailedToRead(usize),
    /// An error occurred during a conversion process, this could mean data is corrupted, or this library has
    /// not implemented the correct conversion for a specific type. That type should then be seen in the error string.
    ConversionError(String),
    /// The device returned an error code in response to a Modbus request. The ErrorCode value indicates the specific error returned by the device.
    DeviceError(ErrorCode),
}

impl<E: hal::Error> From<E> for JSYMk194Error {
    fn from(e: E) -> Self {
        JSYMk194Error::Io(e.kind())
    }
}
