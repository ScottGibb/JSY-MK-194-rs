use crate::{
    define_scaled_register,
    registers::{RegisterAddress, scalars},
};

#[derive(Debug, PartialEq)]
pub struct PowerDirectionRegister {
    pub first_channel: PowerDirection,
    pub second_channel: PowerDirection,
}

#[derive(Debug, PartialEq)]
#[repr(u16)]
pub enum PowerDirection {
    Positive = 0,
    Negative = 1,
}
impl TryFrom<u16> for PowerDirection {
    type Error = crate::error::JSYMk194Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PowerDirection::Positive),
            1 => Ok(PowerDirection::Negative),
            _ => Err(crate::error::JSYMk194Error::ConversionError),
        }
    }
}

define_scaled_register!(
    FrequencyRegister,
    u32,
    RegisterAddress::Frequency,
    scalars::FREQUENCY_SCALAR
);
