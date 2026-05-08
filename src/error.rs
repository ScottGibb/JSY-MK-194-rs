//! Error types for the JSY MK-194 driver, including I/O errors, conversion errors, and device error responses.
use core::num::TryFromIntError;

use crate::hal;
use crate::modbus::FunctionCode;
use crate::modbus::ModbusErrorResponse;
use crate::registers::RegisterAddress;

/// The error type for all operations in this driver.
#[derive(Debug)]
#[non_exhaustive]
pub enum JSYMk194Error {
    /// A low-level error occurred during I/O operations, such as reading from or writing to the device.
    Io(hal::ErrorKind),
    /// The device returned an unexpected response header that could not be parsed correctly, indicating a
    /// potential communication issue or misalignment in the expected response format.
    InvalidHeader,
    /// The Write failed to write the expected number of bytes to the device.
    FailedToWrite {
        /// Number of bytes actually written.
        written: usize,
        /// Number of bytes expected to be written.
        expected: usize,
    },
    /// The Read failed to read the expected number of bytes from the device.
    FailedToRead {
        /// Number of bytes actually read.
        read: usize,
        /// Number of bytes expected to be read.
        expected: usize,
    },
    /// An error occurred during a conversion process, this could mean data is corrupted.
    /// The specific conversion error is detailed in the [`ConversionError`] enum.
    ConversionError(ConversionError),
    /// The CRC check failed, indicating that the data received from the device may be corrupted or tampered with.
    CrcError {
        /// CRC value computed from the received payload.
        actual: u16,
        /// CRC value received from the device.
        expected: u16,
    },
    /// The device responded with an error [`FunctionCode`], indicating that the requested operation could not be completed successfully.
    DeviceErrorResponse(FunctionCode),

    /// The device returned a [`ModbusErrorResponse`]. Inside this error we can see the full error response,
    /// which includes the [`FunctionCode`] and the specific [`crate::modbus::ErrorCode`] returned by the device.
    /// This allows for more detailed error handling based on the type of Modbus exception that occurred.
    ModBusDeviceError(ModbusErrorResponse),
}

impl<E: hal::Error> From<E> for JSYMk194Error {
    fn from(e: E) -> Self {
        JSYMk194Error::Io(e.kind())
    }
}

/// Errors that occur while parsing, converting, or validating register data.
#[derive(Debug)]
#[non_exhaustive]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConversionError {
    /// Register payload length did not match the expected length for the register.
    InvalidRegisterDataLength {
        /// Number of bytes received.
        given_length: usize,
        /// Register address that failed validation.
        address: RegisterAddress,
    },
    /// Quantity of registers could not be converted to the required integer type.
    InvalidQuantityOfRegisters(TryFromIntError),
    /// Byte count could not be converted to the required integer type.
    InvalidByteCount(TryFromIntError),
    /// Register address is unknown to this library.
    UnknownRegister {
        /// Raw register address value reported by the device.
        address: u16,
    },
    /// Register value is outside supported bounds or an invalid enum discriminant.
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
