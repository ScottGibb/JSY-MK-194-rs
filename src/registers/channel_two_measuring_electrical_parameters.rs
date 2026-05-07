use crate::{
    define_scaled_register,
    registers::{
        RegisterAddress,
        scalars::{self, VOLTAGE_SCALAR},
        traits::{ReadRegister, WriteRegister},
    },
};

define_scaled_register!(
    SecondChannelVoltageRegister,
    u32,
    RegisterAddress::SecondChannelVoltage,
    VOLTAGE_SCALAR
);
impl ReadRegister for SecondChannelVoltageRegister {}

define_scaled_register!(
    SecondChannelCurrentRegister,
    u32,
    RegisterAddress::SecondChannelCurrent,
    scalars::CURRENT_SCALAR
);
impl ReadRegister for SecondChannelCurrentRegister {}

define_scaled_register!(
    SecondChannelActivePowerRegister,
    u32,
    RegisterAddress::SecondChannelActivePower,
    scalars::POWER_SCALAR
);
impl ReadRegister for SecondChannelActivePowerRegister {}

define_scaled_register!(
    SecondChannelPositiveActiveEnergyRegister,
    u32,
    RegisterAddress::SecondChannelPositiveActiveEnergy,
    scalars::ENERGY_SCALAR
);
impl ReadRegister for SecondChannelPositiveActiveEnergyRegister {}
impl WriteRegister for SecondChannelPositiveActiveEnergyRegister {}

define_scaled_register!(
    SecondChannelPowerFactorRegister,
    u32,
    RegisterAddress::SecondChannelPowerFactor,
    scalars::POWER_FACTOR_SCALAR
);
impl ReadRegister for SecondChannelPowerFactorRegister {}

define_scaled_register!(
    SecondChannelNegativeActiveEnergyRegister,
    u32,
    RegisterAddress::SecondChannelNegativeActiveEnergy,
    scalars::ENERGY_SCALAR
);

impl ReadRegister for SecondChannelNegativeActiveEnergyRegister {}
impl WriteRegister for SecondChannelNegativeActiveEnergyRegister {}
