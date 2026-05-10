//! This Delay module is only required in std usage as it requires the `DelayNs` trait from
//! `embedded-hal`/`embedded-hal-async`.
//!
//! In no_std environments, the underlying HAL implementation will be provided by the user and the Delay type
//! provided. In Std enviornments this is not usually provided, thus we provide example implementations here.

/// A simple implementation of the `DelayNs` trait using `std::thread::sleep` for synchronous contexts and
///  `tokio::time::sleep` for asynchronous contexts. This is intended for use in desktop applications
pub struct StdDelay;

#[cfg(feature = "std-sync")]
impl embedded_hal::delay::DelayNs for StdDelay {
    fn delay_ns(&mut self, ns: u32) {
        std::thread::sleep(std::time::Duration::from_nanos(u64::from(ns)));
    }
}

#[cfg(feature = "tokio-async")]
impl embedded_hal_async::delay::DelayNs for StdDelay {
    async fn delay_ns(&mut self, ns: u32) {
        tokio::time::sleep(std::time::Duration::from_nanos(u64::from(ns))).await;
    }
}
