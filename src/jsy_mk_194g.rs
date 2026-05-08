use crate::error::JSYMk194Error;
use crate::hal::*;
use crate::registers::system_configuration_parameter::Id;

/// Driver for communicating with a JSY MK-194 power monitor over Modbus RTU.
///
/// The driver is generic over a serial transport and a delay provider so it can
/// be used in desktop (`std-sync`) and embedded (`sync` / `async`) contexts.
/// Construct with [`Self::new`] when you already know the device ID, or
/// [`Self::new_default`] to probe connectivity using the default address.
pub struct JsyMk194g<Serial: Read + Write, D: DelayNs> {
    pub(crate) device_address: Id,
    pub(crate) serial: Serial,
    pub(crate) delay: D,
}

impl<Serial: Read + Write, D: DelayNs> JsyMk194g<Serial, D> {
    /// Creates a driver with an explicit device address.
    ///
    /// This constructor does not perform bus I/O. If you want to verify device
    /// connectivity during construction, use [`Self::new_default`] instead.
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
    /// );
    /// # let _ = driver;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(serial: Serial, device_address: Id, delay: D) -> Self {
        Self {
            serial,
            device_address,
            delay,
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
        };
        // Check if we can get the ID
        device.get_id().await?;
        Ok(device)
    }
}
