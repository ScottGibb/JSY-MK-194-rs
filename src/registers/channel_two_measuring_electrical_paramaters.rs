
use crate::{
    define_scaled_register,
    registers::{
        RegisterAddress,
        scalars::{self, VOLTAGE_SCALAR},
        traits::WriteRegister,
    },
};

define_scaled_register!(
    SecondChannelVoltageRegister,
    u32,
    RegisterAddress::SecondChannelVoltage,
    VOLTAGE_SCALAR
);

define_scaled_register!(
    SecondChannelCurrentRegister,
    u32,
    RegisterAddress::SecondChannelCurrent,
    scalars::CURRENT_SCALAR
);

define_scaled_register!(
    SecondChannelActivePowerRegister,
    u32,
    RegisterAddress::SecondChannelActivePower,
    scalars::POWER_FACTOR_SCALAR
);

define_scaled_register!(
    SecondChannelPositiveActiveEnergyRegister,
    u32,
    RegisterAddress::SecondChannelPositiveActiveEnergy,
    0,
    scalars::ENERGY_SCALAR
);
impl WriteRegister for SecondChannelPositiveActiveEnergyRegister {}

define_scaled_register!(
    SecondChannelPowerFactorRegister,
    u32,
    RegisterAddress::SecondChannelPowerFactor,
    scalars::POWER_FACTOR_SCALAR
);

define_scaled_register!(
    SecondChannelNegativeActiveEnergyRegister,
    u32,
    RegisterAddress::SecondChannelNegativeActiveEnergy,
    0,
    scalars::ENERGY_SCALAR
);
impl WriteRegister for SecondChannelNegativeActiveEnergyRegister {}
