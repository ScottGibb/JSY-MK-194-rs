use crate::error::JSYMk194Error;
use crate::registers::RegisterAddress;
pub trait Register {
    const NUM_BYTES: usize;
    const ADDRESS: RegisterAddress;
    fn try_from_bytes(bytes: &[u8]) -> Result<Self, JSYMk194Error>
    where
        Self: Sized;
    fn to_bytes(&self, bytes: &mut [u8]) -> Result<(), JSYMk194Error>;
    fn address(&self) -> RegisterAddress {
        Self::ADDRESS
    }
    fn num_bytes(&self) -> usize {
        Self::NUM_BYTES
    }
}

pub trait ReadRegister {}

pub trait WriteRegister {}
