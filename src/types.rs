//! Types used across the driver, such as measurement structs and enums for configuration.
pub use crate::registers::misc_registers::PowerDirection;
pub use crate::registers::system_configuration_parameter::{Baudrate, Id};
use crate::registers::system_parameters::ModelOneRegister;
use crate::registers::system_parameters::{CurrentRangeRegister, VoltageRangeRegister};
use crate::units::*;

/// Measurements for a single channel.
///
/// Values are represented with strongly-typed units from [`crate::units`].
#[derive(Debug, PartialEq, Clone)]
pub struct ChannelStatistics {
    /// RMS voltage. Root mean square voltage is the effective voltage value that represents
    /// the equivalent DC voltage that would deliver the same power to a load.
    pub voltage: ElectricPotential,
    /// RMS current. Root mean square current is the effective current value that represents
    /// the equivalent DC current that would deliver the same power to a load.
    pub current: ElectricCurrent,
    /// Active power. Active power is the real power consumed by a load in an AC circuit.
    /// It represents the actual work done by the electrical energy.
    pub active_power: Power,
    /// Positive active energy counter. This counter accumulates the total active energy consumed
    /// by the load in the positive direction.
    pub positive_active_energy: Energy,
    /// Negative active energy counter. This counter accumulates the total active energy consumed
    /// by the load in the negative direction.
    pub negative_active_energy: Energy,
    /// Power factor in the range reported by the device.
    /// The device reports power factor as a value between 0 and 1, where 1 indicates that all the power
    /// is being effectively used (purely resistive load), and values less than 1 indicate the
    /// presence of reactive power (inductive or capacitive loads) which does not contribute to useful work.
    pub power_factor: f32,
    /// Current power flow direction.
    /// The device reports power direction as either positive or negative,
    /// indicating whether power is flowing into the load (positive) or back towards the source (negative).
    pub power_direction: PowerDirection,
}

impl core::fmt::Display for ChannelStatistics {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Voltage: {} V, Current: {} A, Active Power: {} W, Positive Active Energy: {} kWh, Negative Active Energy: {} kWh, Power Factor: {}",
            self.voltage.get::<volt>(),
            self.current.get::<ampere>(),
            self.active_power.get::<watt>(),
            self.positive_active_energy.get::<kilowatt_hour>(),
            self.negative_active_energy.get::<kilowatt_hour>(),
            self.power_factor
        )
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChannelStatistics {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "Voltage: {} V, Current: {} A, Active Power: {} W, Positive Active Energy: {} kWh, Negative Active Energy: {} kWh, Power Factor: {}",
            self.voltage.get::<volt>(),
            self.current.get::<ampere>(),
            self.active_power.get::<watt>(),
            self.positive_active_energy.get::<kilowatt_hour>(),
            self.negative_active_energy.get::<kilowatt_hour>(),
            self.power_factor
        );
    }
}

/// Combined measurements for both channels plus line frequency.
#[derive(Debug, PartialEq, Clone)]
pub struct Statistics {
    /// Statistics for channel one.
    pub channel_one: ChannelStatistics,
    /// Statistics for channel two.
    pub channel_two: ChannelStatistics,

    /// Measured line frequency.
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

#[cfg(feature = "defmt")]
impl defmt::Format for Statistics {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "Frequency: {} Hz\nChannel One:\n{}\nChannel Two:\n{}",
            self.frequency.get::<hertz>(),
            self.channel_one,
            self.channel_two
        );
    }
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Selects which channel an operation should target.
pub enum Channel {
    /// Channel one.
    One,
    /// Channel two.
    Two,
}

/// Static device parameters read from system registers.
#[derive(Debug, PartialEq, Clone)]
pub struct SystemParameters {
    /// Device model identifier (model one register).
    pub model_one: u16,
    // pub model_two: u16, // Left out due to Datasheet saying its reserved
    /// Nominal voltage range.
    pub voltage_range: ElectricPotential,
    /// Nominal current range.
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
#[cfg(feature = "defmt")]
impl defmt::Format for SystemParameters {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "Model: {}, Voltage Range: {} V, Current Range: {} A",
            self.model_one,
            self.voltage_range.get::<volt>(),
            self.current_range.get::<ampere>()
        );
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
