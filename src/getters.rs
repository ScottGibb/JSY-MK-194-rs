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
impl<Serial: Read + Write, D: DelayNs> JsyMk194g<Serial, D> {
    #[maybe_async::maybe_async]
    pub async fn get_id(&mut self) -> Result<Id, JSYMk194Error> {
        let configuration_register = self
            .read_register::<SystemConfigurationParameterRegister>()
            .await?;
        let id = configuration_register.id;
        Ok(id)
    }
    #[maybe_async::maybe_async]
    pub async fn get_baudrate(&mut self) -> Result<Baudrate, JSYMk194Error> {
        let configuration_register = self
            .read_register::<SystemConfigurationParameterRegister>()
            .await?;
        let baudrate = configuration_register.baudrate;
        Ok(baudrate)
    }

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

impl<Serial: Read + Write, D: DelayNs> JsyMk194g<Serial, D> {
    #[maybe_async::maybe_async]
    pub async fn get_all_channels(&mut self) -> Result<Statistics, JSYMk194Error> {
        let stats = self.read_statistics().await?;
        Ok(stats)
    }

    #[maybe_async::maybe_async]
    pub async fn get_channel(
        &mut self,
        channel: Channel,
    ) -> Result<ChannelStatistics, JSYMk194Error> {
        let channel_statistics = self.read_channel_statistics(channel).await?;
        Ok(channel_statistics)
    }

    #[maybe_async::maybe_async]
    pub async fn get_frequency(&mut self) -> Result<Frequency, JSYMk194Error> {
        let frequency_register = self.read_register::<FrequencyRegister>().await?;
        let scaled_value = frequency_register.get_scaled_value();

        Ok(Frequency::new::<hertz>(scaled_value))
    }
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
