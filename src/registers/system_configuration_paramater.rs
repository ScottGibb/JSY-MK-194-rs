use bitbybit::{bitenum, bitfield};

use crate::{impl_read_register, impl_write_register, registers::RegisterAddress};

#[bitfield(u16, default = 0x0105)]
#[derive(Debug, PartialEq)]
struct SystemConfigurationParamaterRegister {
    #[bits(0..=7, rw)]
    pub id: u8,
    #[bits(8..=15, rw)]
    pub baudrate: Option<Baudrate>,
}

#[bitenum(u8, exhaustive = false)]
#[derive(Debug, PartialEq)]
pub enum Baudrate {
    _1200 = 3,
    _2400 = 4,
    _4800 = 5,
    _9600 = 6,
    _19200 = 7,
    _38400 = 8,
}

impl_read_register!(
    SystemConfigurationParamaterRegister,
    RegisterAddress::SystemConfigurationParameter,
    u16
);
impl_write_register!(
    SystemConfigurationParamaterRegister,
    RegisterAddress::SystemConfigurationParameter,
    u16
);

// Supports both Read and Write operations
