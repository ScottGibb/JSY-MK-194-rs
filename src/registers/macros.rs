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
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
            fn try_from_bytes(bytes: &[u8]) -> Result<Self, $crate::error::JSYMk194Error> {
                if bytes.len() != core::mem::size_of::<$data_type>() {
                    return Err($crate::error::JSYMk194Error::ConversionError($crate::error::ConversionError::InvalidRegisterDataLength {
                        given_length: bytes.len(),
                        address: Self::ADDRESS,
                    }));
                }
                let mut arr = [0u8; core::mem::size_of::<$data_type>()];
                arr.copy_from_slice(bytes);
                Ok(Self(<$data_type>::from_be_bytes(arr)))
            }
            fn to_bytes(&self, bytes: &mut [u8]) -> Result<(), $crate::error::JSYMk194Error> {
                let data_bytes = self.0.to_be_bytes();
                if bytes.len() < data_bytes.len() {
                    return Err($crate::error::JSYMk194Error::ConversionError(
                        $crate::error::ConversionError::InvalidRegisterDataLength {
                            given_length: bytes.len(),
                            address: Self::ADDRESS,
                        },
                    ));
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
        // No default implementation, so default() cannot be called

    };

    // Shared implementation
    (@inner $name:ident, $data_type:ty, $address:expr, $scale:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
            fn try_from_bytes(bytes: &[u8]) -> Result<Self, $crate::error::JSYMk194Error> {
                if bytes.len() != core::mem::size_of::<$data_type>() {
                    return Err($crate::error::JSYMk194Error::ConversionError($crate::error::ConversionError::InvalidRegisterDataLength {
                        given_length: bytes.len(),
                        address: Self::ADDRESS,
                    }));
                }
                let mut arr = [0u8; core::mem::size_of::<$data_type>()];
                arr.copy_from_slice(bytes);
                Ok(Self(<$data_type>::from_be_bytes(arr)))
            }
            fn to_bytes(&self, bytes: &mut [u8]) -> Result<(), $crate::error::JSYMk194Error> {
                if bytes.len() != core::mem::size_of::<$data_type>() {
                    return Err($crate::error::JSYMk194Error::ConversionError($crate::error::ConversionError::InvalidRegisterDataLength {
                        given_length: bytes.len(),
                        address: Self::ADDRESS,
                    }));
                }
                let data_bytes = self.0.to_be_bytes();
                bytes[..data_bytes.len()].copy_from_slice(&data_bytes);
                Ok(())
        }
    }

        impl $name {
            pub fn get_scaled_value(&self) -> f32 {
                (self.0 as f32) / ($scale as f32)
            }

            pub fn from_scaled_value(scaled_value: f32) -> Self {
                let raw_value = (scaled_value * ($scale as f32)) as $data_type;
                Self(raw_value)
            }
        }
    };
}
