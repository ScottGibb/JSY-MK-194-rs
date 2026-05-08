use crate::error::JSYMk194Error;
use crate::hal::*;
use crate::jsy_mk_194g::JsyMk194g;
use crate::registers::channel_one_measuring_electrical_parameters::{
    FirstChannelNegativeActiveEnergyRegister, FirstChannelPositiveActiveEnergyRegister,
};
use crate::registers::channel_two_measuring_electrical_parameters::{
    SecondChannelNegativeActiveEnergyRegister, SecondChannelPositiveActiveEnergyRegister,
};
use crate::registers::system_configuration_parameter::{
    Baudrate, Id, SystemConfigurationParameterRegister,
};
use crate::types::Channel;
use crate::units::*;
impl<Serial: ReadWrite, D: DelayNs> JsyMk194g<Serial, D> {
    /// Sets the baud rate of the device.
    ///
    /// This method intentionally consumes `self` because changing the baud rate
    /// invalidates the current driver instance — the underlying serial connection
    /// is now configured at the wrong speed. To communicate with the device again,
    /// you must reconstruct the driver with a new serial port opened at the updated
    /// baud rate.
    ///
    /// # Examples
    /// ```rust
    /// # fn example<S, D>(
    /// #     driver: jsy_mk_194_rs::jsy_mk_194g::JsyMk194g<S, D>,
    /// # ) -> Result<(), jsy_mk_194_rs::error::JSYMk194Error>
    /// # where
    /// #     S: std::io::Read + std::io::Write,
    /// #     D: embedded_hal::delay::DelayNs,
    /// # {
    /// driver.set_baudrate(jsy_mk_194_rs::types::Baudrate::_9600)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// For a full runnable example, see
    /// [`examples/setters.rs`](https://github.com/ScottGibb/JSY-MK-194-rs/blob/main/examples/setters.rs).
    #[maybe_async::maybe_async]
    pub async fn set_baudrate(mut self, baudrate: Baudrate) -> Result<(), JSYMk194Error> {
        let previous_register = self
            .read_register::<SystemConfigurationParameterRegister>()
            .await?;
        let register = SystemConfigurationParameterRegister {
            baudrate,
            id: previous_register.id, // Keep the same ID as the previous register
        };
        self.write_register(register).await?;
        Ok(())
    }

    /// Sets the Modbus device ID.
    ///
    /// The driver updates its local `device_address` field after writing the
    /// register so subsequent requests target the new ID.
    ///
    /// # Examples
    /// ```rust
    /// # fn example<S, D>(
    /// #     driver: &mut jsy_mk_194_rs::jsy_mk_194g::JsyMk194g<S, D>,
    /// # ) -> Result<(), jsy_mk_194_rs::error::JSYMk194Error>
    /// # where
    /// #     S: std::io::Read + std::io::Write,
    /// #     D: embedded_hal::delay::DelayNs,
    /// # {
    /// driver.set_id(jsy_mk_194_rs::types::Id::new(2)?)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// For a full runnable example, see
    /// [`examples/setters.rs`](https://github.com/ScottGibb/JSY-MK-194-rs/blob/main/examples/setters.rs).
    #[maybe_async::maybe_async]
    pub async fn set_id(&mut self, id: Id) -> Result<(), JSYMk194Error> {
        let previous_register = self
            .read_register::<SystemConfigurationParameterRegister>()
            .await?;
        let register = SystemConfigurationParameterRegister {
            baudrate: previous_register.baudrate, // Keep the same baudrate as the previous register
            id: id.clone(),                       // Use the new ID provided as an argument
        };
        self.write_register(register).await?;
        self.device_address = id; // Update the device's ID in the struct to reflect the change
        Ok(())
    }

    /// Sets the positive active energy register for a channel.
    ///
    /// The `energy` value is written in kilowatt-hours.
    ///
    /// # Examples
    /// ```rust
    /// # fn example<S, D>(
    /// #     driver: &mut jsy_mk_194_rs::jsy_mk_194g::JsyMk194g<S, D>,
    /// # ) -> Result<(), jsy_mk_194_rs::error::JSYMk194Error>
    /// # where
    /// #     S: std::io::Read + std::io::Write,
    /// #     D: embedded_hal::delay::DelayNs,
    /// # {
    /// driver.set_positive_active_energy(
    ///     jsy_mk_194_rs::types::Channel::One,
    ///     jsy_mk_194_rs::units::Energy::new::<jsy_mk_194_rs::units::kilowatt_hour>(100.0),
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// For a full runnable example, see
    /// [`examples/setters.rs`](https://github.com/ScottGibb/JSY-MK-194-rs/blob/main/examples/setters.rs).
    #[maybe_async::maybe_async]
    pub async fn set_positive_active_energy(
        &mut self,
        channel: Channel,
        energy: Energy,
    ) -> Result<(), JSYMk194Error> {
        let energy = energy.get::<kilowatt_hour>();
        match channel {
            Channel::One => {
                let register = FirstChannelPositiveActiveEnergyRegister::from_scaled_value(energy);
                self.write_register(register).await?;
                Ok(())
            }
            Channel::Two => {
                let register = SecondChannelPositiveActiveEnergyRegister::from_scaled_value(energy);
                self.write_register(register).await?;
                Ok(())
            }
        }
    }

    /// Sets the negative active energy register for a channel.
    ///
    /// The `energy` value is written in kilowatt-hours.
    ///
    /// # Examples
    /// ```rust
    /// # fn example<S, D>(
    /// #     driver: &mut jsy_mk_194_rs::jsy_mk_194g::JsyMk194g<S, D>,
    /// # ) -> Result<(), jsy_mk_194_rs::error::JSYMk194Error>
    /// # where
    /// #     S: std::io::Read + std::io::Write,
    /// #     D: embedded_hal::delay::DelayNs,
    /// # {
    /// driver.set_negative_active_energy(
    ///     jsy_mk_194_rs::types::Channel::One,
    ///     jsy_mk_194_rs::units::Energy::new::<jsy_mk_194_rs::units::kilowatt_hour>(150.0),
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// For a full runnable example, see
    /// [`examples/setters.rs`](https://github.com/ScottGibb/JSY-MK-194-rs/blob/main/examples/setters.rs).
    #[maybe_async::maybe_async]
    pub async fn set_negative_active_energy(
        &mut self,
        channel: Channel,
        energy: Energy,
    ) -> Result<(), JSYMk194Error> {
        let energy = energy.get::<kilowatt_hour>();
        match channel {
            Channel::One => {
                let register = FirstChannelNegativeActiveEnergyRegister::from_scaled_value(energy);
                self.write_register(register).await?;
                Ok(())
            }
            Channel::Two => {
                let register = SecondChannelNegativeActiveEnergyRegister::from_scaled_value(energy);
                self.write_register(register).await?;
                Ok(())
            }
        }
    }
}
