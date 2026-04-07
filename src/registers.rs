pub mod channel_one_measuring_electrical_paramaters;
pub mod channel_two_measuring_electrical_paramaters;
mod macros;
pub mod misc_registers;
mod scalars;
pub mod system_configuration_paramater;
pub mod system_paramaters;
pub mod traits;

#[derive(Debug, Clone)]
#[repr(u16)]
pub enum RegisterAddress {
    SystemConfigurationParameter = 0x0004,

    ModelOne = 0x0000,
    ModelTwo = 0x0001,
    VoltageRange = 0x0002,
    CurrentRange = 0x0003,

    FirstChannelVoltage = 0x0048,
    FirstChannelCurrent = 0x0049,
    FirstChannelActivePower = 0x004A,
    FirstChannelPositiveActiveEnergy = 0x004B,
    FirstChannelPowerFactor = 0x004C,
    FirstChannelNegativeActiveEnergy = 0x004D,

    PowerDirection = 0x004E,
    Frequency = 0x004F,

    SecondChannelVoltage = 0x0050,
    SecondChannelCurrent = 0x0051,
    SecondChannelActivePower = 0x0052,
    SecondChannelPositiveActiveEnergy = 0x0053,
    SecondChannelPowerFactor = 0x0054,
    SecondChannelNegativeActiveEnergy = 0x0055,
}

impl From<RegisterAddress> for u16 {
    fn from(value: RegisterAddress) -> Self {
        value as u16
    }
}
