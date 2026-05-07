use crate::error::JSYMk194Error;
use crate::hal::*;
use crate::jsy_mk_194g::JsyMk194g;
use crate::registers::channel_one_measuring_electrical_paramaters::{
    FirstChannelNegativeActiveEnergyRegister, FirstChannelPositiveActiveEnergyRegister,
};
use crate::registers::channel_two_measuring_electrical_paramaters::{
    SecondChannelNegativeActiveEnergyRegister, SecondChannelPositiveActiveEnergyRegister,
};
use crate::registers::system_configuration_paramater::{
    Baudrate, Id, SystemConfigurationParameterRegister,
};
use crate::types::Channel;
use crate::units::*;
impl<Serial: Read + Write, D: DelayNs> JsyMk194g<Serial, D> {
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
