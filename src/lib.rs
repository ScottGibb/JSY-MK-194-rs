#![deny(unsafe_code)]
#![cfg_attr(not(feature = "std-sync"), no_std)]
//! Rust driver for the JSY MK-194 power monitor IC.
//!
//! # Feature flags
//!
//! Exactly one runtime mode must be enabled:
//!
//! - `std-sync` (default): Synchronous desktop/`std` mode using `std::io`.
//! - `sync`: Synchronous `no_std` embedded mode using `embedded-io` and `embedded-hal`.
//! - `async`: Asynchronous `no_std` embedded mode using `embedded-io-async` and
//!   `embedded-hal-async`.
//!
//! You can select a runtime mode in your `Cargo.toml`:
//!
//! ```toml
//! # std-sync (default)
//! jsy-mk-194-rs = { version = "x.y.z", features = ["std-sync"] }
//!
//! # sync
//! jsy-mk-194-rs = { version = "x.y.z", default-features = false, features = ["sync"] }
//!
//! # async
//! jsy-mk-194-rs = { version = "x.y.z", default-features = false, features = ["async"] }
//! ```
//!
//! ## `advanced` feature
//!
//! The optional `advanced` feature exposes low-level register APIs (including the public
//! `registers` module and direct register read/write methods).
//!
//! This is intended for advanced usage where you need register-level control beyond the
//! high-level getter/setter API. Prefer the high-level API when possible, and only enable
//! `advanced` when you need direct register access.

mod modbus;
pub use modbus::REQUEST_RESPONSE_DELAY;
#[cfg(feature = "advanced")]
pub mod registers;
#[cfg(not(feature = "advanced"))]
mod registers;

pub mod error;
pub mod jsy_mk_194g;

mod getters;
mod setters;

pub mod types;
pub mod units;

// Ensure exactly one mode is enabled
#[cfg(all(feature = "sync", feature = "async"))]
compile_error!("Choose only one of sync, async, or std-sync.");
#[cfg(all(feature = "sync", feature = "std-sync"))]
compile_error!("Choose only one of sync, async, or std-sync.");
#[cfg(all(feature = "async", feature = "std-sync"))]
compile_error!("Choose only one of sync, async, or std-sync.");
#[cfg(not(any(feature = "sync", feature = "async", feature = "std-sync")))]
compile_error!("Choose one of sync, async, or std-sync.");

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

#[cfg(feature = "std-sync")]
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
