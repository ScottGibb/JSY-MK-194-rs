use crate::registers::{
    RegisterAddress,
    scalars::{self, VOLTAGE_SCALAR},
    traits::{ReadRegister, WriteRegister},
};

define_scaled_register!(
    FirstChannelVoltageRegister,
    u32,
    RegisterAddress::FirstChannelVoltage,
    VOLTAGE_SCALAR
);
impl ReadRegister for FirstChannelVoltageRegister {}

define_scaled_register!(
    FirstChannelCurrentRegister,
    u32,
    RegisterAddress::FirstChannelCurrent,
    scalars::CURRENT_SCALAR
);
impl ReadRegister for FirstChannelCurrentRegister {}

define_scaled_register!(
    FirstChannelActivePowerRegister,
    u32,
    RegisterAddress::FirstChannelActivePower,
    scalars::POWER_SCALAR
);
impl ReadRegister for FirstChannelActivePowerRegister {}

define_scaled_register!(
    FirstChannelPositiveActiveEnergyRegister,
    u32,
    RegisterAddress::FirstChannelPositiveActiveEnergy,
    scalars::ENERGY_SCALAR
);
impl WriteRegister for FirstChannelPositiveActiveEnergyRegister {}
impl ReadRegister for FirstChannelPositiveActiveEnergyRegister {}

define_scaled_register!(
    FirstChannelPowerFactorRegister,
    u32,
    RegisterAddress::FirstChannelPowerFactor,
    scalars::POWER_FACTOR_SCALAR
);
impl ReadRegister for FirstChannelPowerFactorRegister {}

define_scaled_register!(
    FirstChannelNegativeActiveEnergyRegister,
    u32,
    RegisterAddress::FirstChannelNegativeActiveEnergy,
    scalars::ENERGY_SCALAR
);

impl ReadRegister for FirstChannelNegativeActiveEnergyRegister {}
impl WriteRegister for FirstChannelNegativeActiveEnergyRegister {}
