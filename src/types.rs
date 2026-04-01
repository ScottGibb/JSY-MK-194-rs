use uom::si::f32::{ElectricCurrent, ElectricPotential, Energy, Frequency, Power};

use crate::registers::measuring_electrical_paramaters::PowerDirection;

pub struct Channel {
    pub voltage: ElectricPotential,
    pub current: ElectricCurrent,
    pub active_power: Power,
    pub positive_active_energy: Energy,
    pub negative_active_energy: Energy,

    pub power_factor: f32,
}

pub struct Statistics {
    pub channel_one: Channel,
    pub channel_two: Channel,

    pub power_direction: PowerDirection,
    pub frequency: Frequency,
}
