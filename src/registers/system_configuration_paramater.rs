use crate::{
    error::{ConversionError, JSYMk194Error},
    registers::{
        RegisterAddress,
        traits::{ReadRegister, Register, WriteRegister},
    },
};

#[derive(Debug, PartialEq, Default)]
pub struct SystemConfigurationParamaterRegister {
    pub id: Id,
    pub baudrate: Baudrate,
}

#[derive(Debug, PartialEq, Clone, Default)]
#[repr(u8)]
pub enum Baudrate {
    _1200 = 3,
    _2400 = 4,
    #[default]
    _4800 = 5,
    _9600 = 6,
    _19200 = 7,
    _38400 = 8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Id {
    id: u8,
}

impl Id {
    pub fn new(id: u8) -> Result<Self, JSYMk194Error> {
        if id == 0 {
            return Err(JSYMk194Error::ConversionError(
                ConversionError::InvalidValue,
            ));
        }
        Ok(Self { id })
    }
}

impl From<Id> for u8 {
    fn from(value: Id) -> Self {
        value.id
    }
}

impl Default for Id {
    fn default() -> Self {
        Self { id: 1 }
    }
}

impl TryFrom<u8> for Baudrate {
    type Error = JSYMk194Error;
    fn try_from(value: u8) -> Result<Self, JSYMk194Error> {
        match value {
            3 => Ok(Baudrate::_1200),
            4 => Ok(Baudrate::_2400),
            5 => Ok(Baudrate::_4800),
            6 => Ok(Baudrate::_9600),
            7 => Ok(Baudrate::_19200),
            8 => Ok(Baudrate::_38400),
            _ => Err(JSYMk194Error::ConversionError(
                ConversionError::InvalidValue,
            )),
        }
    }
}

impl TryFrom<u32> for Baudrate {
    type Error = JSYMk194Error;
    fn try_from(baud_rate: u32) -> Result<Self, JSYMk194Error> {
        match baud_rate {
            1200 => Ok(Baudrate::_1200),
            2400 => Ok(Baudrate::_2400),
            4800 => Ok(Baudrate::_4800),
            9600 => Ok(Baudrate::_9600),
            19200 => Ok(Baudrate::_19200),
            38400 => Ok(Baudrate::_38400),
            _ => Err(JSYMk194Error::ConversionError(
                ConversionError::InvalidValue,
            )),
        }
    }
}

impl From<Baudrate> for u8 {
    fn from(baudrate: Baudrate) -> Self {
        baudrate as u8
    }
}

impl From<Baudrate> for u32 {
    fn from(baudrate: Baudrate) -> Self {
        match baudrate {
            Baudrate::_1200 => 1200,
            Baudrate::_2400 => 2400,
            Baudrate::_4800 => 4800,
            Baudrate::_9600 => 9600,
            Baudrate::_19200 => 19200,
            Baudrate::_38400 => 38400,
        }
    }
}

impl Register for SystemConfigurationParamaterRegister {
    const NUM_BYTES: usize = core::mem::size_of::<u8>() + core::mem::size_of::<Baudrate>();
    const ADDRESS: RegisterAddress = RegisterAddress::SystemConfigurationParameter;

    fn try_from_bytes(bytes: &[u8]) -> Result<Self, JSYMk194Error> {
        if bytes.len() != Self::NUM_BYTES {
            return Err(JSYMk194Error::ConversionError(
                ConversionError::InvalidRegisterDataLength {
                    length: bytes.len(),
                    address: Self::ADDRESS,
                },
            ));
        }
        let id = Id::new(bytes[0])
            .map_err(|_| JSYMk194Error::ConversionError(ConversionError::InvalidValue))?;
        let baudrate = Baudrate::try_from(bytes[1])
            .map_err(|_| JSYMk194Error::ConversionError(ConversionError::InvalidValue))?;
        Ok(Self { id, baudrate })
    }

    fn to_bytes(&self, bytes: &mut [u8]) -> Result<(), JSYMk194Error> {
        if bytes.len() < Self::NUM_BYTES {
            return Err(JSYMk194Error::ConversionError(
                ConversionError::InvalidRegisterDataLength {
                    length: bytes.len(),
                    address: Self::ADDRESS,
                },
            ));
        }
        bytes[0] = u8::from(self.id.clone());
        bytes[1] = u8::from(self.baudrate.clone());
        Ok(())
    }
}

impl ReadRegister for SystemConfigurationParamaterRegister {}
impl WriteRegister for SystemConfigurationParamaterRegister {}
