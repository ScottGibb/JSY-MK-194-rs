use crate::error::{ConversionError, JSYMk194Error};

pub mod channel_one_measuring_electrical_paramaters;
pub mod channel_two_measuring_electrical_paramaters;
mod macros;
pub mod misc_registers;
mod scalars;
pub mod system_configuration_paramater;
pub mod system_paramaters;
pub mod traits;

#[derive(Debug, Clone)]
pub enum RegisterAddress {
    SystemConfigurationParameter = 0x0004,

    ModelOne = 0x0000,
    ModelTwo = 0x0001,
    VoltageRange = 0x0002,
    CurrentRange = 0x0003,

    FirstChannelVoltage = 0x0048,
    FirstChannelCurrent = 0x0049,
    FirstChannelActivePower = 0x004A,
    FirstChannelPositiveActiveEnergy = 0x004B,
    FirstChannelPowerFactor = 0x004C,
    FirstChannelNegativeActiveEnergy = 0x004D,

    PowerDirection = 0x004E,
    Frequency = 0x004F,

    SecondChannelVoltage = 0x0050,
    SecondChannelCurrent = 0x0051,
    SecondChannelActivePower = 0x0052,
    SecondChannelPositiveActiveEnergy = 0x0053,
    SecondChannelPowerFactor = 0x0054,
    SecondChannelNegativeActiveEnergy = 0x0055,

    ClearEnergyCommand = 0x000C,
}

impl From<RegisterAddress> for u16 {
    fn from(value: RegisterAddress) -> Self {
        value as u16
    }
}
impl TryFrom<u16> for RegisterAddress {
    type Error = JSYMk194Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0004 => Ok(RegisterAddress::SystemConfigurationParameter),
            0x0000 => Ok(RegisterAddress::ModelOne),
            0x0001 => Ok(RegisterAddress::ModelTwo),
            0x0002 => Ok(RegisterAddress::VoltageRange),
            0x0003 => Ok(RegisterAddress::CurrentRange),
            0x0048 => Ok(RegisterAddress::FirstChannelVoltage),
            0x0049 => Ok(RegisterAddress::FirstChannelCurrent),
            0x004A => Ok(RegisterAddress::FirstChannelActivePower),
            0x004B => Ok(RegisterAddress::FirstChannelPositiveActiveEnergy),
            0x004C => Ok(RegisterAddress::FirstChannelPowerFactor),
            0x004D => Ok(RegisterAddress::FirstChannelNegativeActiveEnergy),
            0x004E => Ok(RegisterAddress::PowerDirection),
            0x004F => Ok(RegisterAddress::Frequency),
            0x0050 => Ok(RegisterAddress::SecondChannelVoltage),
            0x0051 => Ok(RegisterAddress::SecondChannelCurrent),
            0x0052 => Ok(RegisterAddress::SecondChannelActivePower),
            0x0053 => Ok(RegisterAddress::SecondChannelPositiveActiveEnergy),
            0x0054 => Ok(RegisterAddress::SecondChannelPowerFactor),
            0x0055 => Ok(RegisterAddress::SecondChannelNegativeActiveEnergy),
            0x000C => Ok(RegisterAddress::ClearEnergyCommand),
            _ => Err(JSYMk194Error::ConversionError(
                ConversionError::UnknownRegister { address: value },
            )),
        }
    }
}
