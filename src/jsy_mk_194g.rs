use core::time::Duration;

use crate::error::JSYMk194Error;
use crate::hal::*;
use crate::modbus::DEFAULT_CHANNEL_REQUEST_RESPONSE_DELAY;
use crate::modbus::DEFAULT_REQUEST_RESPONSE_DELAY;
use crate::registers::system_configuration_parameter::Id;

/// Driver for communicating with a JSY MK-194 power monitor over Modbus RTU.
///
/// The driver is generic over a serial transport and a delay provider so it can
/// be used in desktop (`std-sync`) and embedded (`sync` / `async`) contexts.
/// Construct with [`Self::new`] when you already know the device ID, or
/// [`Self::new_default`] to probe connectivity using the default address.
pub struct JsyMk194g<Serial: ReadWrite, D: DelayNs> {
    pub(crate) device_address: Id,
    pub(crate) serial: Serial,
    pub(crate) delay: D,
    pub(crate) response_delay: Duration,
    pub(crate) channel_response_delay: Duration,
}

impl<Serial: ReadWrite, D: DelayNs> JsyMk194g<Serial, D> {
    /// Creates a driver with an explicit device address.
    ///
    /// This constructor does not perform bus I/O. If you want to verify device
    /// connectivity during construction, use [`Self::new_default`] instead.
    ///
    /// Note that if the provided `response_delay` and `channel_response_delay` are too short, the
    /// driver may not wait long enough for the device to respond, which can lead to errors such as timeouts or CRC errors.
    /// If you encounter such issues, consider increasing these delays to ensure reliable communication.
    ///
    /// Due to the `embedded-hal` traits used for DelayNs, the underlying duration implementation uses u32
    /// where core uses u64, If an invalid duration is provided (e.g. one that exceeds the maximum value of u32 in nanoseconds), the driver will panic when it attempts to apply the delay. It's the caller's responsibility to ensure that the provided durations are valid and won't cause overflow issues.
    /// the driver will throw a [`ConversionError`](crate::error::ConversionError) during a request.
    ///
    /// # Examples
    /// ```rust
    /// # fn example<S, D>(serial: S, delay: D) -> Result<(), jsy_mk_194_rs::error::JSYMk194Error>
    /// # where
    /// #     S: std::io::Read + std::io::Write,
    /// #     D: embedded_hal::delay::DelayNs,
    /// # {
    /// let driver = jsy_mk_194_rs::jsy_mk_194g::JsyMk194g::new(
    ///     serial,
    ///     jsy_mk_194_rs::types::Id::new(1)?,
    ///     delay,
    ///     jsy_mk_194_rs::DEFAULT_REQUEST_RESPONSE_DELAY,
    ///     jsy_mk_194_rs::DEFAULT_CHANNEL_REQUEST_RESPONSE_DELAY,
    /// );
    /// # let _ = driver;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(
        serial: Serial,
        device_address: Id,
        delay: D,
        response_delay: Duration,
        channel_response_delay: Duration,
    ) -> Self {
        Self {
            serial,
            device_address,
            delay,
            response_delay,
            channel_response_delay,
        }
    }
    /// Creates a driver using the default device address and validates
    /// communication by reading the device ID.
    ///
    /// # Examples
    /// ```rust
    /// # fn example<S, D>(serial: S, delay: D) -> Result<(), jsy_mk_194_rs::error::JSYMk194Error>
    /// # where
    /// #     S: std::io::Read + std::io::Write,
    /// #     D: embedded_hal::delay::DelayNs,
    /// # {
    /// let driver = jsy_mk_194_rs::jsy_mk_194g::JsyMk194g::new_default(serial, delay)?;
    /// # let _ = driver;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// For full runnable examples, see:
    /// - [`examples/getters.rs`](https://github.com/ScottGibb/JSY-MK-194-rs/blob/main/examples/getters.rs)
    /// - [`examples/setters.rs`](https://github.com/ScottGibb/JSY-MK-194-rs/blob/main/examples/setters.rs)
    #[maybe_async::maybe_async]
    pub async fn new_default(serial: Serial, delay: D) -> Result<Self, JSYMk194Error> {
        // Check if device is on the bus
        let mut device = Self {
            serial,
            device_address: Id::default(),
            delay,
            response_delay: DEFAULT_REQUEST_RESPONSE_DELAY,
            channel_response_delay: DEFAULT_CHANNEL_REQUEST_RESPONSE_DELAY,
        };
        // Check if we can get the ID
        device.get_id().await?;
        Ok(device)
    }
}
