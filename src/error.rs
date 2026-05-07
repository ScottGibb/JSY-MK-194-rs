use core::num::TryFromIntError;

use crate::hal;
use crate::modbus::ModbusErrorResponse;
use crate::modbus::types::FunctionCode;
use crate::registers::RegisterAddress;
#[derive(Debug)]
#[non_exhaustive]
pub enum JSYMk194Error {
    /// An error occurred during I/O operations, such as reading from or writing to the device.
    Io(hal::ErrorKind),
    /// The device returned an unexpected response or data format.
    InvalidHeader,
    /// The Write failed to write the expected number of bytes to the device.
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
    ConversionError(ConversionError),
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

#[derive(Debug)]
#[non_exhaustive]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConversionError {
    InvalidRegisterDataLength {
        given_length: usize,
        address: RegisterAddress,
    },
    InvalidQuantityOfRegisters(TryFromIntError),
    InvalidByteCount(TryFromIntError),
    UnknownRegister {
        address: u16,
    },
    InvalidValue,
}

#[cfg(feature = "defmt")]
impl defmt::Format for JSYMk194Error {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            JSYMk194Error::Io(_io_error) => defmt::write!(fmt, "I/O error occurred"),
            JSYMk194Error::InvalidHeader => defmt::write!(fmt, "Invalid response header"),
            JSYMk194Error::FailedToWrite { written, expected } => defmt::write!(
                fmt,
                "Failed to write the expected number of bytes. Written: {}, Expected: {}",
                written,
                expected
            ),
            JSYMk194Error::FailedToRead { read, expected } => defmt::write!(
                fmt,
                "Failed to read the expected number of bytes. Read: {}, Expected: {}",
                read,
                expected
            ),
            JSYMk194Error::ConversionError(conversion_error) => {
                defmt::write!(fmt, "Conversion error: {:?}", conversion_error)
            }
            JSYMk194Error::CrcError { actual, expected } => defmt::write!(
                fmt,
                "CRC error: Actual CRC: {:04X}, Expected CRC: {:04X}",
                actual,
                expected
            ),
            JSYMk194Error::DeviceErrorResponse(function_code) => defmt::write!(
                fmt,
                "Device responded with an error function code: {:02X}",
                u8::from(function_code.clone())
            ),
            JSYMk194Error::ModBusDeviceError(error_response) => defmt::write!(
                fmt,
                "Device responded with an error: Function Code: {:02X}, Error Code: {:02X}",
                u8::from(error_response.function_code.clone()),
                u8::from(error_response.error_code.clone())
            ),
        }
    }
}
