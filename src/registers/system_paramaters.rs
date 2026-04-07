use crate::define_register;

define_register!(
    ModelOneRegister,
    u16,
    crate::registers::RegisterAddress::ModelOne,
    0x0194
);

define_register!(
    ModelTwoRegister,
    u16,
    crate::registers::RegisterAddress::ModelTwo
);

define_register!(
    VoltageRangeRegister,
    u16,
    crate::registers::RegisterAddress::VoltageRange,
    0x00FA // Default value is 250V
);

// define_scaled_register!(
//     CurrentRangeRegister,
//     u16,
//     crate::registers::RegisterAddress::CurrentRange,
//     0x0320, // Default value is 800 (80A)
//     CURRENT_RANGE_SCALAR
// );
