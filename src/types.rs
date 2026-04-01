use crate::registers::measuring_electrical_paramaters::PowerDirection;
use crate::units::*;

pub struct ChannelStatistics {
    pub voltage: ElectricPotential,
    pub current: ElectricCurrent,
    pub active_power: Power,
    pub positive_active_energy: Energy,
    pub negative_active_energy: Energy,

    pub power_factor: f32,
}

pub struct Statistics {
    pub channel_one: ChannelStatistics,
    pub channel_two: ChannelStatistics,

    pub power_direction: PowerDirection,
    pub frequency: Frequency,
}

pub enum Channel {
    One,
    Two,
}
