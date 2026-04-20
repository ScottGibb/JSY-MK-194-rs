use crate::{
    define_register, define_scaled_register,
    registers::{
        scalars::{CURRENT_RANGE_SCALAR, VOLTAGE_RANGE_SCALAR},
        traits::ReadRegister,
    },
};

define_register!(
    ModelOneRegister,
    u16,
    crate::registers::RegisterAddress::ModelOne,
    0x0194
);

impl ReadRegister for ModelOneRegister {}

define_register!(
    ModelTwoRegister,
    u16,
    crate::registers::RegisterAddress::ModelTwo
);

impl ReadRegister for ModelTwoRegister {}

define_scaled_register!(
    VoltageRangeRegister,
    u16,
    crate::registers::RegisterAddress::VoltageRange,
    0x00FA, // Default value is 250V
    VOLTAGE_RANGE_SCALAR
);
impl ReadRegister for VoltageRangeRegister {}

define_scaled_register!(
    CurrentRangeRegister,
    u16,
    crate::registers::RegisterAddress::CurrentRange,
    0x0320, // Default value is 800 (80A)
    10.0
);

impl ReadRegister for CurrentRangeRegister {}
