use crate::{
    define_scaled_register,
    error::{ConversionError, JSYMk194Error},
    registers::{
        RegisterAddress, scalars,
        traits::{ReadRegister, Register},
    },
};

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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

    fn try_from_bytes(bytes: &[u8]) -> Result<Self, JSYMk194Error> {
        if bytes.len() != Self::NUM_BYTES {
            return Err(JSYMk194Error::ConversionError(
                ConversionError::InvalidRegisterDataLength {
                    given_length: bytes.len(),
                    address: Self::ADDRESS,
                },
            ));
        }
        // Datasheet layout for 0x004E stores direction flags in the first two bytes:
        // byte 0 -> channel one (0=positive, 1=negative)
        // byte 1 -> channel two (0=positive, 1=negative)
        let first_channel = PowerDirection::try_from(bytes[0])?;
        let second_channel = PowerDirection::try_from(bytes[1])?;
        Ok(Self {
            first_channel,
            second_channel,
        })
    }

    fn to_bytes(&self, bytes: &mut [u8]) -> Result<(), JSYMk194Error> {
        if bytes.len() < Self::NUM_BYTES {
            return Err(JSYMk194Error::ConversionError(
                ConversionError::InvalidRegisterDataLength {
                    given_length: bytes.len(),
                    address: Self::ADDRESS,
                },
            ));
        }

        bytes[0] = u8::from(self.first_channel.clone());
        bytes[1] = u8::from(self.second_channel.clone());
        // The remaining two bytes are reserved for this register in current protocol docs.
        bytes[2] = 0;
        bytes[3] = 0;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u16)]
/// Indicates whether active power is flowing in the positive or negative direction.
pub enum PowerDirection {
    /// Power is flowing in the positive direction.
    Positive = 0,
    /// Power is flowing in the negative direction.
    Negative = 1,
}

impl TryFrom<u8> for PowerDirection {
    type Error = JSYMk194Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PowerDirection::Positive),
            1 => Ok(PowerDirection::Negative),
            _ => Err(JSYMk194Error::ConversionError(
                ConversionError::InvalidValue,
            )),
        }
    }
}

impl From<PowerDirection> for u8 {
    fn from(direction: PowerDirection) -> Self {
        direction as u8
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
impl AsRef<str> for PowerDirection {
    fn as_ref(&self) -> &str {
        match self {
            PowerDirection::Positive => "Positive",
            PowerDirection::Negative => "Negative",
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
