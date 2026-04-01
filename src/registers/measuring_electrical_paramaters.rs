use super::scalars::*;
use crate::{define_four_byte_register, impl_read_register, registers::RegisterAddress};
use bitbybit::{bitenum, bitfield};

// First Channel Registers
define_four_byte_register!(
    RegisterAddress::FirstChannelVoltage,
    FirstChannelVoltageRegister,
    voltage,
    VOLTAGE_SCALAR,
    r
);

define_four_byte_register!(
    RegisterAddress::FirstChannelCurrent,
    FirstChannelCurrentRegister,
    current,
    CURRENT_SCALAR,
    r
);

define_four_byte_register!(
    RegisterAddress::FirstChannelActivePower,
    FirstChannelActivePower,
    active_power,
    ENERGY_SCALAR,
    r
);
define_four_byte_register!(
    RegisterAddress::FirstChannelPositiveActiveEnergy,
    FirstChannelPositiveActiveEnergy,
    active_positive_energy,
    ENERGY_SCALAR,
    rw
);
define_four_byte_register!(
    RegisterAddress::FirstChannelPowerFactor,
    FirstChannelPowerFactor,
    power_factor,
    POWER_FACTOR_SCALAR,
    r
);

define_four_byte_register!(
    RegisterAddress::FirstChannelNegativeActiveEnergy,
    FirstChannelNegativeActiveEnergy,
    active_negative_energy,
    ENERGY_SCALAR,
    rw
);

#[bitfield(u32)]
pub struct ChannelsPowerDirection {
    #[bits(0..=7, r)]
    pub channel_one: Option<PowerDirection>,
    #[bits(8..=15, r)]
    pub channel_two: Option<PowerDirection>,
}

#[bitenum(u8, exhaustive = false)]
pub enum PowerDirection {
    Positive = 0x00,
    Neagative = 0x01,
}

define_four_byte_register!(
    RegisterAddress::Frequency,
    FrequencyRegister,
    frequency,
    FREQUENCY_SCALAR,
    r
);

// // Second Channel Registers
define_four_byte_register!(
    RegisterAddress::SecondChannelVoltage,
    SecondChannelVoltageRegister,
    voltage,
    VOLTAGE_SCALAR,
    r
);

define_four_byte_register!(
    RegisterAddress::SecondChannelCurrent,
    SecondChannelCurrentRegister,
    current,
    CURRENT_SCALAR,
    r
);

define_four_byte_register!(
    RegisterAddress::SecondChannelActivePower,
    SecondChannelActivePower,
    active_power,
    ENERGY_SCALAR,
    r
);

define_four_byte_register!(
    RegisterAddress::SecondChannelPositiveActiveEnergy,
    SecondChannelPositiveActiveEnergy,
    active_positive_energy,
    ENERGY_SCALAR,
    rw
);

define_four_byte_register!(
    RegisterAddress::SecondChannelPowerFactor,
    SecondChannelPowerFactor,
    power_factor,
    POWER_FACTOR_SCALAR,
    r
);

define_four_byte_register!(
    RegisterAddress::SecondChannelNegativeActiveEnergy,
    SecondChannelNegativeActiveEnergy,
    active_negative_energy,
    ENERGY_SCALAR,
    rw
);
