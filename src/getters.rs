use crate::jsy_mk_194g::JsyMk194g;
use crate::registers::misc_registers::FrequencyRegister;
use crate::registers::system_configuration_paramater::{
    Baudrate, SystemConfigurationParamaterRegister,
};
use crate::registers::system_paramaters::{
    CurrentRangeRegister, ModelOneRegister, VoltageRangeRegister,
};
use crate::types::SystemParameters;
use crate::units::*;
use crate::{
    error::JSYMk194Error,
    types::{Channel, ChannelStatistics, Statistics},
};
use crate::hal::*;
impl<Serial: Read + Write> JsyMk194g<Serial> {
    #[maybe_async::maybe_async]
    pub async fn get_all_channels(&mut self) -> Result<Statistics, JSYMk194Error> {
        // TODO: replace this with a custom read to get all registers at the same time
        let frequency = self.get_frequency().await?;
        let channel_one = self.get_channel(Channel::One).await?;
        let channel_two = self.get_channel(Channel::Two).await?;
        Ok(Statistics {
            channel_one,
            channel_two,
            frequency,
        })
    }

    #[maybe_async::maybe_async]
    pub async fn get_channel(
        &mut self,
        channel: Channel,
    ) -> Result<ChannelStatistics, JSYMk194Error> {
        match channel {
            Channel::One => Ok(ChannelStatistics {
                voltage: todo!(),
                current: todo!(),
                active_power: todo!(),
                positive_active_energy: todo!(),
                negative_active_energy: todo!(),
                power_factor: todo!(),
                power_direction: todo!(),
            }),
            Channel::Two => Ok(ChannelStatistics {
                voltage: todo!(),
                current: todo!(),
                active_power: todo!(),
                positive_active_energy: todo!(),
                negative_active_energy: todo!(),
                power_factor: todo!(),
                power_direction: todo!(),
            }),
        }
    }
    #[maybe_async::maybe_async]
    pub async fn get_frequency(&mut self) -> Result<Frequency, JSYMk194Error> {
        let frequency_register = self.read_register::<FrequencyRegister>().await?;
        let scaled_value = frequency_register.get_scaled_value();

        Ok(Frequency::new::<hertz>(scaled_value))
    }
}

impl<Serial: Read + Write> JsyMk194g<Serial> {
    #[maybe_async::maybe_async]
    pub async fn get_baudrate(&mut self) -> Result<Baudrate, JSYMk194Error> {
        let configuration_register = self
            .read_register::<SystemConfigurationParamaterRegister>()
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
