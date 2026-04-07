use crate::{
    define_scaled_register,
    registers::{
        RegisterAddress,
        scalars::{self, VOLTAGE_SCALAR},
    },
};

define_scaled_register!(
    FirstChannelVoltageRegister,
    u32,
    RegisterAddress::FirstChannelVoltage,
    VOLTAGE_SCALAR
);

define_scaled_register!(
    FirstChannelCurrentRegister,
    u32,
    RegisterAddress::FirstChannelCurrent,
    scalars::CURRENT_SCALAR
);

define_scaled_register!(
    FirstChannelActivePowerRegister,
    u32,
    RegisterAddress::FirstChannelActivePower,
    scalars::POWER_FACTOR_SCALAR
);

define_scaled_register!(
    FirstChannelPositiveActiveEnergyRegister,
    u32,
    RegisterAddress::FirstChannelPositiveActiveEnergy,
    scalars::ENERGY_SCALAR
);

define_scaled_register!(
    FirstChannelPowerFactorRegister,
    u32,
    RegisterAddress::FirstChannelPowerFactor,
    scalars::POWER_FACTOR_SCALAR
);

define_scaled_register!(
    FirstChannelNegativeActiveEnergyRegister,
    u32,
    RegisterAddress::FirstChannelNegativeActiveEnergy,
    scalars::ENERGY_SCALAR
);

#[derive(Debug, PartialEq)]
struct PowerDirectionRegister {
    pub first_channel: PowerDirection,
    pub second_channel: PowerDirection,
}

#[derive(Debug, PartialEq)]
#[repr(u16)]
enum PowerDirection {
    Positive = 0,
    Negative = 1,
}
impl TryFrom<u16> for PowerDirection {
    type Error = crate::error::JSYMk194Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PowerDirection::Positive),
            1 => Ok(PowerDirection::Negative),
            _ => Err(crate::error::JSYMk194Error::ConversionError),
        }
    }
}

define_scaled_register!(
    FrequencyRegister,
    u32,
    RegisterAddress::Frequency,
    scalars::FREQUENCY_SCALAR
);

define_scaled_register!(
    SecondChannelVoltageRegister,
    u32,
    RegisterAddress::SecondChannelVoltage,
    VOLTAGE_SCALAR
);
