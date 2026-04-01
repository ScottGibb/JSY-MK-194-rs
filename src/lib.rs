#![no_std]
#![deny(unsafe_code)]

pub mod error;
pub mod getters;
pub mod jsy_mk_194g;
mod modubus;
pub mod registers;
pub mod setters;
pub mod types;
pub mod units;

#[cfg(all(feature = "sync", feature = "async"))]
compile_error!("You cannot use both sync and async features at the same time. Please choose one.");

#[cfg(all(not(feature = "async"), not(feature = "sync")))]
compile_error!("You must enable either the sync or async feature. Please choose one.");

/// Sync Based HAL Imports
#[cfg(feature = "sync")]
mod hal {
    pub use embedded_io::Error;
    pub use embedded_io::ErrorKind;
    pub use embedded_io::Read;
    pub use embedded_io::Write;
}

/// Async Based HAL Imports
#[cfg(feature = "async")]
mod hal {
    pub use embedded_io_async::Error;
    pub use embedded_io_async::ErrorKind;
    pub use embedded_io_async::Read;
    pub use embedded_io_async::Write;
}
