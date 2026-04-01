mod macros;
pub mod measuring_electrical_paramaters;
mod scalars;
pub mod system_configuration_paramater;
pub mod system_paramaters;
pub enum RegisterMap {
    SystemConfigurationParameter = 0x0004,

    ModelOne = 0x0000,
    ModelTwo = 0x0001,
    VoltageRange = 0x0002,
    CurrentRange = 0x0003,

    FirstChannelVoltage = 0x0048,
    FirstChannelCurrent = 0x0049,
    FirstChannelActivePower = 0x004A,
    FirstChannelPositiveEnergy = 0x004B,
    FirstChannelPowerFactor = 0x004C,
    FirstChannelNegativeEnergy = 0x004D,

    PowerDirection = 0x004E,
    Frequency = 0x004F,

    SecondChannelVoltage = 0x0050,
    SecondChannelCurrent = 0x0051,
    SecondChannelActivePower = 0x0052,
    SecondChannelPositiveEnergy = 0x0053,
    SecondChannelPowerFactor = 0x0054,
    SecondChannelNegativeEnergy = 0x0055,
}
