use crate::registers::{
    channel_one_measuring_electrical_paramaters::{
        FirstChannelActivePowerRegister, FirstChannelCurrentRegister,
        FirstChannelNegativeActiveEnergyRegister, FirstChannelPositiveActiveEnergyRegister,
        FirstChannelPowerFactorRegister, FirstChannelVoltageRegister,
    },
    channel_two_measuring_electrical_paramaters::{
        SecondChannelActivePowerRegister, SecondChannelCurrentRegister,
        SecondChannelNegativeActiveEnergyRegister, SecondChannelPositiveActiveEnergyRegister,
        SecondChannelPowerFactorRegister, SecondChannelVoltageRegister,
    },
    misc_registers::{FrequencyRegister, PowerDirectionRegister},
    traits::Register,
};

// Modbus RTU header/data offsets
pub const MODBUS_DEVICE_ADDRESS_OFFSET: usize = 0;
pub const MODBUS_FUNCTION_CODE_OFFSET: usize = 1;
pub const MODBUS_ERROR_CODE_OFFSET: usize = 2;

// Number of bytes to be expected when doing multi register reads

pub const NUM_CHANNEL_ONE_READ_BYTES: usize = FirstChannelVoltageRegister::NUM_BYTES
    + FirstChannelCurrentRegister::NUM_BYTES
    + FirstChannelActivePowerRegister::NUM_BYTES
    + FirstChannelPositiveActiveEnergyRegister::NUM_BYTES
    + FirstChannelPowerFactorRegister::NUM_BYTES
    + FirstChannelNegativeActiveEnergyRegister::NUM_BYTES;

pub const NUM_CHANNEL_TWO_READ_BYTES: usize = SecondChannelVoltageRegister::NUM_BYTES
    + SecondChannelCurrentRegister::NUM_BYTES
    + SecondChannelActivePowerRegister::NUM_BYTES
    + SecondChannelPositiveActiveEnergyRegister::NUM_BYTES
    + SecondChannelPowerFactorRegister::NUM_BYTES
    + SecondChannelNegativeActiveEnergyRegister::NUM_BYTES;

pub const NUM_ALL_CHANNELS_READ_BYTES: usize = NUM_CHANNEL_ONE_READ_BYTES
    + NUM_CHANNEL_TWO_READ_BYTES
    + PowerDirectionRegister::NUM_BYTES
    + FrequencyRegister::NUM_BYTES;
