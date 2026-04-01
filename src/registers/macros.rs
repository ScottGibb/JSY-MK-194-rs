#[macro_export]
macro_rules! define_scaled_register {
    ($name:ident, $scale:expr) => {
        define_scaled_register!($name, value, $scale, r);
    };
    ($name:ident, $method:ident, $scale:expr) => {
        define_scaled_register!($name, $method, $scale, r);
    };
    ($name:ident, $method:ident, $scale:expr, $access:ident) => {
        #[bitfield(u32)]
        pub struct $name {
            #[bits(0..=31, $access)]
            pub reading: u32,
        }

        impl $name {
            pub fn $method(&self) -> f32 {
                self.reading() as f32 / ($scale as f32)
            }
        }
    };
}
