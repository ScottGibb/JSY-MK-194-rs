use crate::hal;
use crate::hal::*;
pub enum JSYMk194Error {
    /// An error occurred during I/O operations, such as reading from or writing to the device.
    Io(hal::ErrorKind),
    /// The device returned an unexpected response or data format.
    InvalidResponse,
    /// The Write failded to write the expected number of bytes to the device. The usize value indicates the number of bytes that were actually written.
    FailedToWrite(usize),
}

impl<E: hal::Error> From<E> for JSYMk194Error {
    fn from(e: E) -> Self {
        JSYMk194Error::Io(e.kind())
    }
}
