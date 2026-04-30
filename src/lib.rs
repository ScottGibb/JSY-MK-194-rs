#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]

mod modbus;
pub use modbus::REQUEST_RESPONSE_DELAY;
pub mod registers;

pub mod error;
pub mod jsy_mk_194g;

pub mod getters;
pub mod setters;

pub mod types;
pub mod units;

#[cfg(all(feature = "sync", feature = "async"))]
compile_error!("Choose only one of sync or async.");
#[cfg(all(feature = "std", any(feature = "sync", feature = "async")))]
compile_error!("Choose std OR embedded mode, not both.");

/// Sync Based HAL Imports
#[cfg(feature = "sync")]
mod hal {
    pub use embedded_hal::delay::DelayNs;
    pub use embedded_io::Error;
    pub use embedded_io::ErrorKind;
    pub use embedded_io::Read;
    pub use embedded_io::Write;
}

/// Async Based HAL Imports
#[cfg(feature = "async")]
mod hal {
    pub use embedded_hal_async::delay::DelayNs;
    pub use embedded_io_async::Error;
    pub use embedded_io_async::ErrorKind;
    pub use embedded_io_async::Read;
    pub use embedded_io_async::Write;
}

#[cfg(feature = "std")]
mod hal {
    pub use embedded_hal::delay::DelayNs;
    pub use std::io::{ErrorKind, Read, Write};
    pub trait Error {
        fn kind(&self) -> ErrorKind;
    }

    impl Error for std::io::Error {
        fn kind(&self) -> ErrorKind {
            std::io::Error::kind(self)
        }
    }
}
