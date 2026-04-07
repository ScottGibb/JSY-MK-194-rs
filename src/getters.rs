use crate::jsy_mk_194g::JsyMk194g;
use crate::registers::system_configuration_paramater::Baudrate;
use crate::types::SystemParameters;
use crate::{
    error::JSYMk194Error,
    types::{Channel, ChannelStatistics, Statistics},
};
use crate::{hal::*, units::*};

impl<Serial: Read + Write> JsyMk194g<Serial> {
    #[maybe_async::maybe_async]
    pub async fn get_all_channels(&mut self) -> Result<Statistics, JSYMk194Error> {
        Ok(Statistics {
            channel_one: todo!(),
            channel_two: todo!(),
            frequency: todo!(),
        })
    }

    #[maybe_async::maybe_async]
    pub async fn get_channel(
        &mut self,
        channel: Channel,
    ) -> Result<ChannelStatistics, JSYMk194Error> {
        match channel {
            Channel::One => todo!(),
            Channel::Two => todo!(),
        }
        Ok(ChannelStatistics {
            voltage: todo!(),
            current: todo!(),
            active_power: todo!(),
            positive_active_energy: todo!(),
            negative_active_energy: todo!(),
            power_factor: todo!(),
            power_direction: todo!(),
        })
    }
    #[maybe_async::maybe_async]
    pub async fn get_frequency(&mut self) -> Result<Frequency, JSYMk194Error> {
        todo!();
        Ok(Frequency::new::<hertz>(10.0))
    }
}

impl<Serial: Read + Write> JsyMk194g<Serial> {
    #[maybe_async::maybe_async]
    pub async fn get_baudrate(&mut self) -> Result<Baudrate, JSYMk194Error> {
        todo!();
        Ok(Baudrate::_1200)
    }

    #[maybe_async::maybe_async]
    pub async fn get_system_parameters(&mut self) -> Result<SystemParameters, JSYMk194Error> {
        todo!();
        Ok(SystemParameters {
            ModelOne: 0,
            voltage_range: ElectricPotential::new::<volt>(0.0),
            current_range: ElectricCurrent::new::<ampere>(0.0),
        })
    }
}
