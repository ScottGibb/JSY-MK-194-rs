use crate::{
    error::JSYMk194Error,
    registers::{
        RegisterAddress,
        traits::{ReadRegister, Register, WriteRegister},
    },
};

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Id {
    id: u8,
}

impl Id {
    pub fn new(id: u8) -> Result<Self, JSYMk194Error> {
        if id == 0 {
            return Err(JSYMk194Error::ConversionError);
        }
        Ok(Self { id })
    }

    pub fn value(&self) -> u8 {
        self.id
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
            _ => Err(JSYMk194Error::ConversionError),
        }
    }
}

impl TryFrom<u32> for Baudrate {
    type Error = JSYMk194Error;
    fn try_from(value: u32) -> Result<Self, JSYMk194Error> {
        match value {
            1200 => Ok(Baudrate::_1200),
            2400 => Ok(Baudrate::_2400),
            4800 => Ok(Baudrate::_4800),
            9600 => Ok(Baudrate::_9600),
            19200 => Ok(Baudrate::_19200),
            38400 => Ok(Baudrate::_38400),
            _ => Err(JSYMk194Error::ConversionError),
        }
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

    fn from_bytes(bytes: &[u8]) -> Self {
        //TODO: Handle errors properly instead of panicking
        if bytes.len() != Self::NUM_BYTES {
            panic!("Invalid byte length for SystemConfigurationParamaterRegister");
        }
        let id = Id::new(bytes[0]).expect("Invalid ID value");
        let baudrate = Baudrate::try_from(bytes[1]).expect("Invalid baudrate value");
        Self { id, baudrate }
    }

    fn to_bytes(&self, bytes: &mut [u8]) -> Result<(), JSYMk194Error> {
        if bytes.len() < Self::NUM_BYTES {
            return Err(JSYMk194Error::ConversionError);
        }
        bytes[0] = self.id.value();
        bytes[1] = self.baudrate.clone() as u8;
        Ok(())
    }
}

impl ReadRegister for SystemConfigurationParamaterRegister {}
impl WriteRegister for SystemConfigurationParamaterRegister {}
