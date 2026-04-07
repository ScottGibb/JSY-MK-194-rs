use crate::registers::misc_registers::PowerDirection;
use crate::units::*;

pub struct ChannelStatistics {
    pub voltage: ElectricPotential,
    pub current: ElectricCurrent,
    pub active_power: Power,
    pub positive_active_energy: Energy,
    pub negative_active_energy: Energy,

    pub power_direction: PowerDirection,
    pub power_factor: f32,
}

pub struct Statistics {
    pub channel_one: ChannelStatistics,
    pub channel_two: ChannelStatistics,

    pub frequency: Frequency,
}

pub enum Channel {
    One,
    Two,
}

pub struct SystemParameters {
    pub ModelOne: u16,
    pub voltage_range: ElectricPotential,
    pub current_range: ElectricCurrent,
}
