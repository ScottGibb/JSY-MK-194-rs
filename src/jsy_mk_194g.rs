use crate::error::JSYMk194Error;
use crate::hal::*;
pub struct JsyMk194g<Serial: Read + Write, D: DelayNs> {
    pub device_address: u8,
    pub(crate) serial: Serial,
    pub(crate) delay: D,
}

impl<Serial: Read + Write, D: DelayNs> JsyMk194g<Serial, D> {
    pub fn new(serial: Serial, device_address: u8, delay: D) -> Self {
        Self {
            serial,
            device_address,
            delay,
        }
    }
    #[maybe_async::maybe_async]
    pub async fn new_default(serial: Serial, delay: D) -> Result<Self, JSYMk194Error> {
        // Check if device is on the bus
        let device = Self {
            serial,
            device_address: 0,
            delay,
        };
        Ok(device)
    }
}
