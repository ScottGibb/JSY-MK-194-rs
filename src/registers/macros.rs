#[macro_export]
macro_rules! define_four_byte_register {
    // read-only
    ($register_address:expr, $name:ident, $method:ident, $scale:expr, r $(,)?) => {
        #[bitfield(u32)]
        pub struct $name {
            #[bits(0..=31, r)]
            pub reading: u32,
        }

        impl $name {
            pub fn $method(&self) -> f32 {
                self.reading() as f32 / ($scale as f32)
            }
        }

        $crate::impl_read_register!($name, $register_address, u32);
    };

    // write-only
    ($register_address:expr, $name:ident, $method:ident, $scale:expr, w $(,)?) => {
        #[bitfield(u32)]
        pub struct $name {
            #[bits(0..=31, w)]
            pub reading: u32,
        }

        impl $name {
            pub fn $method(&self) -> f32 {
                self.reading() as f32 / ($scale as f32)
            }
        }

        $crate::impl_write_register!($name, $register_address, u32);
    };

    // read-write
    ($register_address:expr, $name:ident, $method:ident, $scale:expr, rw $(,)?) => {
        #[bitfield(u32)]
        pub struct $name {
            #[bits(0..=31, rw)]
            pub reading: u32,
        }

        impl $name {
            pub fn $method(&self) -> f32 {
                self.reading() as f32 / ($scale as f32)
            }
        }

        $crate::impl_write_register!($name, $register_address, u32);
        $crate::impl_read_register!($name, $register_address, u32);
    };
}

#[macro_export]
macro_rules! impl_write_register {
    ($type:ty, $address:expr, $data_type:ty) => {
        impl $crate::registers::traits::WriteRegister<$data_type> for $type {
            fn raw_value(&self) -> $data_type {
                self.raw_value()
            }
            fn get_command(&self) -> $crate::registers::RegisterAddress {
                $address
            }
        }
    };
}

#[macro_export]
macro_rules! impl_read_register {
    ($type:ty, $address:expr, $data_type:ty) => {
        impl $crate::registers::traits::ReadRegister<$data_type> for $type {
            fn new_with_raw_value(raw_value: $data_type) -> Self {
                Self::new_with_raw_value(raw_value)
            }
            fn command() -> $crate::registers::RegisterAddress {
                $address
            }
        }
    };
}
