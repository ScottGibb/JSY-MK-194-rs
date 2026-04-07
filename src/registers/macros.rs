#[macro_export]
macro_rules! define_register {
    // With explicit default
    ($name:ident, $data_type:ty, $address:expr, $default:expr) => {
        define_register!(@inner $name, $data_type, $address);

        impl Default for $name {
            fn default() -> Self {
                Self($default)
            }
        }
    };

    // Without default → fallback to underlying type default
    ($name:ident, $data_type:ty, $address:expr) => {
        define_register!(@inner $name, $data_type, $address);

        impl Default for $name {
            fn default() -> Self {
                Self(<$data_type as Default>::default())
            }
        }
    };

    // Shared implementation
    (@inner $name:ident, $data_type:ty, $address:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub $data_type);

        impl From<$data_type> for $name {
            fn from(value: $data_type) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $data_type {
            fn from(val: $name) -> $data_type {
                val.0
            }
        }

        impl $crate::registers::traits::Register for $name {
            const ADDRESS: $crate::registers::RegisterAddress = $address;
            const NUM_BYTES: usize = core::mem::size_of::<$data_type>();
            fn from_bytes(bytes: &[u8]) -> Self {
                let mut arr = [0u8; core::mem::size_of::<$data_type>()];
                arr.copy_from_slice(bytes);
                Self(<$data_type>::from_be_bytes(arr))
            }
            fn to_bytes(&self, bytes: &mut [u8]) -> Result<(), crate::error::JSYMk194Error> {
                let data_bytes = self.0.to_be_bytes();
                if bytes.len() < data_bytes.len() {
                    return Err(crate::error::JSYMk194Error::ConversionError);
                }
                bytes[..data_bytes.len()].copy_from_slice(&data_bytes);
                Ok(())
        }
    }

    };
}

#[macro_export]
macro_rules! define_scaled_register {
    // With explicit default
    ($name:ident, $data_type:ty, $address:expr, $default:expr, $scale:expr) => {
        define_scaled_register!(@inner $name, $data_type, $address, $scale);

        impl Default for $name {
            fn default() -> Self {
                Self($default)
            }
        }
    };

    // Without default
    ($name:ident, $data_type:ty, $address:expr, $scale:expr) => {
        define_scaled_register!(@inner $name, $data_type, $address, $scale);

        impl Default for $name {
            fn default() -> Self {
                Self(<$data_type as Default>::default())
            }
        }
    };

    // Shared implementation
    (@inner $name:ident, $data_type:ty, $address:expr, $scale:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub $data_type);

        impl From<$data_type> for $name {
            fn from(value: $data_type) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $data_type {
            fn from(val: $name) -> $data_type {
                val.0
            }
        }

        impl $crate::registers::traits::Register for $name {
            const ADDRESS: $crate::registers::RegisterAddress = $address;
            const NUM_BYTES: usize = core::mem::size_of::<$data_type>();
            fn from_bytes(bytes: &[u8]) -> Self {
                let mut arr = [0u8; core::mem::size_of::<$data_type>()];
                arr.copy_from_slice(bytes);
                Self(<$data_type>::from_be_bytes(arr))
            }
            fn to_bytes(&self, bytes: &mut [u8]) -> Result<(), crate::error::JSYMk194Error> {
                let data_bytes = self.0.to_be_bytes();
                if bytes.len() < data_bytes.len() {
                    return Err(crate::error::JSYMk194Error::ConversionError);
                }
                bytes[..data_bytes.len()].copy_from_slice(&data_bytes);
                Ok(())
        }
    }

        impl $name {
            pub fn get_scaled_value(&self) -> f32 {
                (self.0 as f32) * ($scale as f32)
            }
        }
    };
}
