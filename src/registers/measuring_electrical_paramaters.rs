use super::scalars::*;
use crate::define_scaled_register;
use bitbybit::{bitenum, bitfield};

// First Channel Registers
define_scaled_register!(FirstChannelVoltageRegister, voltage, VOLTAGE_SCALAR, r);
define_scaled_register!(FirstChannelCurrentRegister, current, CURRENT_SCALAR, r);
define_scaled_register!(FirstChannelActivePower, active_power, ENERGY_SCALAR, r);
define_scaled_register!(
    FirstChannelPositiveActiveEnergy,
    active_positive_energy,
    ENERGY_SCALAR,
    rw
);
define_scaled_register!(
    FirstChannelPowerFactor,
    power_factor,
    POWER_FACTOR_SCALAR,
    r
);
define_scaled_register!(
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

define_scaled_register!(FrequencyRegister, frequency, FREQUENCY_SCALAR, r);

// Second Channel Registers
define_scaled_register!(SecondChannelVoltageRegister, voltage, VOLTAGE_SCALAR, r);
define_scaled_register!(SecondChannelCurrentRegister, current, CURRENT_SCALAR, r);
define_scaled_register!(SecondChannelActivePower, active_power, ENERGY_SCALAR, r);
define_scaled_register!(
    SecondChannelPositiveActiveEnergy,
    active_positive_energy,
    ENERGY_SCALAR,
    rw
);
define_scaled_register!(
    SecondChannelPowerFactor,
    power_factor,
    POWER_FACTOR_SCALAR,
    r
);
define_scaled_register!(
    SecondChannelNegativeActiveEnergy,
    active_negative_energy,
    ENERGY_SCALAR,
    rw
);
