use crate::{
    define_scaled_register,
    registers::{
        RegisterAddress,
        scalars::{self, VOLTAGE_SCALAR},
        traits::{ReadRegister, WriteRegister},
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
impl ReadRegister for FirstChannelActivePowerRegister {}

define_scaled_register!(
    FirstChannelPositiveActiveEnergyRegister,
    u32,
    RegisterAddress::FirstChannelPositiveActiveEnergy,
    0,
    scalars::ENERGY_SCALAR
);
impl WriteRegister for FirstChannelPositiveActiveEnergyRegister {}

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
    0,
    scalars::ENERGY_SCALAR
);
impl WriteRegister for FirstChannelNegativeActiveEnergyRegister {}
