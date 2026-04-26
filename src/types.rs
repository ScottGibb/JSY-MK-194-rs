pub use crate::registers::system_configuration_paramater::{Baudrate, Id};
use crate::registers::system_paramaters::{CurrentRangeRegister, VoltageRangeRegister};
use crate::registers::{
    misc_registers::PowerDirection,
    system_configuration_paramater::SystemConfigurationParamaterRegister,
    system_paramaters::ModelOneRegister,
};
use crate::units::*;
#[derive(Debug, PartialEq)]
pub struct ChannelStatistics {
    pub voltage: ElectricPotential,
    pub current: ElectricCurrent,
    pub active_power: Power,
    pub positive_active_energy: Energy,
    pub negative_active_energy: Energy,

    pub power_direction: PowerDirection,
    pub power_factor: f32,
}

impl core::fmt::Display for ChannelStatistics {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Voltage: {} V, Current: {} A, Active Power: {} W, Positive Active Energy: {} kWh, Negative Active Energy: {} kWh, Power Direction: {:?}, Power Factor: {}",
            self.voltage.get::<volt>(),
            self.current.get::<ampere>(),
            self.active_power.get::<watt>(),
            self.positive_active_energy.get::<kilowatt_hour>(),
            self.negative_active_energy.get::<kilowatt_hour>(),
            self.power_direction,
            self.power_factor
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Statistics {
    pub channel_one: ChannelStatistics,
    pub channel_two: ChannelStatistics,

    pub frequency: Frequency,
}

impl core::fmt::Display for Statistics {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Frequency: {} Hz\nChannel One:\n{}\nChannel Two:\n{}",
            self.frequency.get::<hertz>(),
            self.channel_one,
            self.channel_two
        )
    }
}

#[derive(Debug)]
pub enum Channel {
    One,
    Two,
}

#[derive(Debug, PartialEq)]
pub struct SystemParameters {
    pub model_one: u16,
    // pub model_two: u16, // Left out due to Datasheet saying its reserved
    pub voltage_range: ElectricPotential,
    pub current_range: ElectricCurrent,
}

impl core::fmt::Display for SystemParameters {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Model: {}, Voltage Range: {} V, Current Range: {} A",
            self.model_one,
            self.voltage_range.get::<volt>(),
            self.current_range.get::<ampere>()
        )
    }
}

impl Default for SystemParameters {
    fn default() -> Self {
        Self {
            model_one: ModelOneRegister::default().0,
            voltage_range: ElectricPotential::new::<volt>(
                VoltageRangeRegister::default().get_scaled_value(),
            ),
            current_range: ElectricCurrent::new::<ampere>(
                CurrentRangeRegister::default().get_scaled_value(),
            ),
        }
    }
}
