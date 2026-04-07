use crate::{
    error::JSYMk194Error,
    registers::{RegisterAddress, traits::Register},
};

#[derive(Debug, PartialEq)]
pub struct SystemConfigurationParamaterRegister {
    pub id: u8,
    pub baudrate: Baudrate,
}

#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum Baudrate {
    _1200 = 3,
    _2400 = 4,
    _4800 = 5,
    _9600 = 6,
    _19200 = 7,
    _38400 = 8,
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

impl Register for SystemConfigurationParamaterRegister {
    const NUM_BYTES: usize = core::mem::size_of::<u8>() + core::mem::size_of::<Baudrate>();
    const ADDRESS: RegisterAddress = RegisterAddress::SystemConfigurationParameter;

    fn from_bytes(bytes: &[u8]) -> Self {
        if bytes.len() != Self::NUM_BYTES {
            panic!("Invalid byte length for SystemConfigurationParamaterRegister");
        }
        let id = bytes[0];
        let baudrate = Baudrate::try_from(bytes[1]).expect("Invalid baudrate value");
        Self { id, baudrate }
    }

    fn to_bytes(&self, bytes: &mut [u8]) -> Result<(), JSYMk194Error> {
        if bytes.len() < Self::NUM_BYTES {
            return Err(JSYMk194Error::ConversionError);
        }
        bytes[0] = self.id;
        bytes[1] = self.baudrate.clone() as u8;
        Ok(())
    }
}
