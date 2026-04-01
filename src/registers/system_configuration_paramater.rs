use bitbybit::{bitenum, bitfield};

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

// Supports both Read and Write operations
