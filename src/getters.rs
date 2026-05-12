use core::time::Duration;

use crate::hal::*;
use crate::jsy_mk_194g::JsyMk194g;
use crate::registers::misc_registers::{FrequencyRegister, PowerDirection, PowerDirectionRegister};
use crate::registers::system_configuration_parameter::{
    Baudrate, SystemConfigurationParameterRegister,
};
use crate::registers::system_parameters::{
    CurrentRangeRegister, ModelOneRegister, VoltageRangeRegister,
};
use crate::types::{Id, SystemParameters};
use crate::units::*;
use crate::{
    error::JSYMk194Error,
    types::{Channel, ChannelStatistics, Statistics},
};
impl<Serial: ReadWrite, D: DelayNs> JsyMk194g<Serial, D> {
    /// Reads the configured Modbus device ID.
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
    /// let id = driver.get_id()?;
    /// println!("Device ID: {id:?}");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// For a full runnable example, see
    /// [`examples/getters.rs`](https://github.com/ScottGibb/JSY-MK-194-rs/blob/main/examples/getters.rs).
    #[maybe_async::maybe_async]
    pub async fn get_id(&mut self) -> Result<Id, JSYMk194Error> {
        let configuration_register = self
            .read_register::<SystemConfigurationParameterRegister>()
            .await?;
        let id = configuration_register.id;
        Ok(id)
    }
    /// Reads the configured baud rate from the system configuration register.
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
    /// let baudrate = driver.get_baudrate()?;
    /// println!("Baudrate: {baudrate:?}");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// For a full runnable example, see
    /// [`examples/getters.rs`](https://github.com/ScottGibb/JSY-MK-194-rs/blob/main/examples/getters.rs).
    #[maybe_async::maybe_async]
    pub async fn get_baudrate(&mut self) -> Result<Baudrate, JSYMk194Error> {
        let configuration_register = self
            .read_register::<SystemConfigurationParameterRegister>()
            .await?;
        let baudrate = configuration_register.baudrate;
        Ok(baudrate)
    }

    /// Gets the current response delay used between request write and response read.
    ///
    /// Returns a tuple containing:
    /// - The general `response_delay` (first element)
    /// - The `channel_response_delay` (second element)
    ///
    /// These values represent the configured wait times applied after Modbus
    /// operations to ensure the device has sufficient time to respond.
    ///
    /// # Examples
    /// ```rust
    /// # fn example<S, D>(
    /// #     driver: &jsy_mk_194_rs::jsy_mk_194g::JsyMk194g<S, D>,
    /// # ) -> Result<(), jsy_mk_194_rs::error::JSYMk194Error>
    /// # where
    /// #     S: std::io::Read + std::io::Write,
    /// #     D: embedded_hal::delay::DelayNs,
    /// # {
    /// let (response_delay, channel_response_delay) = driver.get_response_delay();
    /// println!("Response delay: {:?}", response_delay);
    /// println!("Channel response delay: {:?}", channel_response_delay);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// For a full runnable example, see
    /// [`examples/response_delay.rs`](https://github.com/ScottGibb/JSY-MK-194-rs/blob/main/examples/response_delay.rs).
    pub fn get_response_delay(&self) -> (Duration, Duration) {
        (self.response_delay, self.channel_response_delay)
    }

    /// Reads system-level parameters such as model and hardware ranges.
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
    /// let params = driver.get_system_parameters()?;
    /// println!("{params}");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// For a full runnable example, see
    /// [`examples/getters.rs`](https://github.com/ScottGibb/JSY-MK-194-rs/blob/main/examples/getters.rs).
    #[maybe_async::maybe_async]
    pub async fn get_system_parameters(&mut self) -> Result<SystemParameters, JSYMk194Error> {
        //TODO: replace this with a custom read to get all registers at the same time
        let model_one = self.read_register::<ModelOneRegister>().await?;
        let voltage_range_register = self.read_register::<VoltageRangeRegister>().await?;
        let current_range_register = self.read_register::<CurrentRangeRegister>().await?;

        Ok(SystemParameters {
            model_one: model_one.0,
            voltage_range: ElectricPotential::new::<volt>(
                voltage_range_register.get_scaled_value(),
            ),
            current_range: ElectricCurrent::new::<ampere>(
                current_range_register.get_scaled_value(),
            ),
        })
    }
}

impl<Serial: ReadWrite, D: DelayNs> JsyMk194g<Serial, D> {
    /// Reads measurements for both channels in a single call.
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
    /// let stats = driver.get_all_channels()?;
    /// println!("{stats}");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// For a full runnable example, see
    /// [`examples/getters.rs`](https://github.com/ScottGibb/JSY-MK-194-rs/blob/main/examples/getters.rs).
    #[maybe_async::maybe_async]
    pub async fn get_all_channels(&mut self) -> Result<Statistics, JSYMk194Error> {
        let stats = self.read_statistics().await?;
        Ok(stats)
    }

    /// Reads measurements for a specific channel.
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
    /// let channel_one = driver.get_channel(jsy_mk_194_rs::types::Channel::One)?;
    /// println!("{channel_one}");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// For a full runnable example, see
    /// [`examples/getters.rs`](https://github.com/ScottGibb/JSY-MK-194-rs/blob/main/examples/getters.rs).
    #[maybe_async::maybe_async]
    pub async fn get_channel(
        &mut self,
        channel: Channel,
    ) -> Result<ChannelStatistics, JSYMk194Error> {
        let channel_statistics = self.read_channel_statistics(channel).await?;
        Ok(channel_statistics)
    }

    /// Reads the measured line frequency.
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
    /// let frequency = driver.get_frequency()?;
    /// println!("{} Hz", frequency.get::<jsy_mk_194_rs::units::hertz>());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// For a full runnable example, see
    /// [`examples/getters.rs`](https://github.com/ScottGibb/JSY-MK-194-rs/blob/main/examples/getters.rs).
    #[maybe_async::maybe_async]
    pub async fn get_frequency(&mut self) -> Result<Frequency, JSYMk194Error> {
        let frequency_register = self.read_register::<FrequencyRegister>().await?;
        let scaled_value = frequency_register.get_scaled_value();

        Ok(Frequency::new::<hertz>(scaled_value))
    }
    /// Reads the active power flow direction for a channel.
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
    /// let direction = driver.get_power_direction(jsy_mk_194_rs::types::Channel::One)?;
    /// println!("{direction:?}");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// For a full runnable example, see
    /// [`examples/getters.rs`](https://github.com/ScottGibb/JSY-MK-194-rs/blob/main/examples/getters.rs).
    #[maybe_async::maybe_async]
    pub async fn get_power_direction(
        &mut self,
        channel: Channel,
    ) -> Result<PowerDirection, JSYMk194Error> {
        let power_direction_register = self.read_register::<PowerDirectionRegister>().await?;
        match channel {
            Channel::One => Ok(power_direction_register.first_channel),
            Channel::Two => Ok(power_direction_register.second_channel),
        }
    }
}
