use bitbybit::bitfield;

#[bitfield(u16, default = 0x0194)]
pub struct ModelOneRegister {
    #[bits(0..=15, r)]
    pub model_one: u16,
}

#[bitfield(u16)]
pub struct ModelTwoRegister {
    #[bits(0..=15, r)]
    pub model_two: u16,
}

#[bitfield(u16, default = 0x00FA)]
pub struct VoltageRangeRegister {
    #[bits(0..=15, r,)]
    pub voltage_range: u16, // Default value is 250V
}

#[bitfield(u16, default = 0x0320)]
pub struct CurrentRangeRegister {
    #[bits(0..=15, r)]
    raw_current_range: u16, // Default value is 800
}

impl CurrentRangeRegister {
    pub fn current_range(&self) -> u16 {
        self.raw_current_range() / 10
    }
}
