use crate::jsy_mk_194g::JsyMk194g;
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
            power_direction: todo!(),
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
        })
    }

    pub async fn get_frequency(&mut self) -> Result<Frequency, JSYMk194Error> {
        Ok(Frequency::new::<hertz>(10.0))
    }
}
