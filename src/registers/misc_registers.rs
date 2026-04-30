use crate::{
    define_scaled_register,
    registers::{
        RegisterAddress, scalars,
        traits::{ReadRegister, Register},
    },
};

#[derive(Debug, PartialEq)]
pub struct PowerDirectionRegister {
    pub first_channel: PowerDirection,
    pub second_channel: PowerDirection,
}

impl core::fmt::Display for PowerDirectionRegister {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "First Channel: {}, Second Channel: {}",
            self.first_channel, self.second_channel
        )
    }
}

impl ReadRegister for PowerDirectionRegister {}

impl Register for PowerDirectionRegister {
    const NUM_BYTES: usize =
        core::mem::size_of::<PowerDirection>() + core::mem::size_of::<PowerDirection>();
    const ADDRESS: RegisterAddress = RegisterAddress::PowerDirection;

    fn from_bytes(bytes: &[u8]) -> Self {
        if bytes.len() != Self::NUM_BYTES {
            panic!("Invalid byte length for PowerDirectionRegister");
        }
        let first_channel = PowerDirection::try_from(u16::from_le_bytes([bytes[0], bytes[1]]))
            .expect("Invalid first channel value");
        let second_channel = PowerDirection::try_from(u16::from_le_bytes([bytes[2], bytes[3]]))
            .expect("Invalid second channel value");
        Self {
            first_channel,
            second_channel,
        }
    }

    fn to_bytes(&self, bytes: &mut [u8]) -> Result<(), crate::error::JSYMk194Error> {
        if bytes.len() < Self::NUM_BYTES {
            return Err(crate::error::JSYMk194Error::ConversionError(
                "Invalid byte length for PowerDirectionRegister".into(),
            ));
        }
        let first_channel_bytes = (self.first_channel.clone() as u16).to_le_bytes();
        let second_channel_bytes = (self.second_channel.clone() as u16).to_le_bytes();
        bytes[0] = first_channel_bytes[0];
        bytes[1] = first_channel_bytes[1];
        bytes[2] = second_channel_bytes[0];
        bytes[3] = second_channel_bytes[1];
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
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
            _ => Err(crate::error::JSYMk194Error::ConversionError(format!(
                "Invalid power direction value: {value}"
            ))),
        }
    }
}

impl core::fmt::Display for PowerDirection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PowerDirection::Positive => write!(f, "Positive"),
            PowerDirection::Negative => write!(f, "Negative"),
        }
    }
}

define_scaled_register!(
    FrequencyRegister,
    u32,
    RegisterAddress::Frequency,
    scalars::FREQUENCY_SCALAR
);

impl ReadRegister for FrequencyRegister {}
