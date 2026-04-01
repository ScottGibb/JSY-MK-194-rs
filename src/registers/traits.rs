use crate::registers::RegisterAddress;

pub trait WriteRegister<T: Sized> {
    fn raw_value(&self) -> T;
    fn get_command(&self) -> RegisterAddress;
}

pub trait ReadRegister<T: Sized> {
    fn new_with_raw_value(raw_value: T) -> Self;
    fn command() -> RegisterAddress;
}

pub trait ToBeBytes {
    type Bytes: AsRef<[u8]>;
    fn to_be_bytes(&self) -> Self::Bytes;
}

impl ToBeBytes for u16 {
    type Bytes = [u8; 2];
    fn to_be_bytes(&self) -> Self::Bytes {
        u16::to_be_bytes(*self)
    }
}

impl ToBeBytes for u32 {
    type Bytes = [u8; 4];
    fn to_be_bytes(&self) -> Self::Bytes {
        u32::to_be_bytes(*self)
    }
}
