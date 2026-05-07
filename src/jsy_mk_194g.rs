use crate::error::JSYMk194Error;
use crate::hal::*;
use crate::registers::system_configuration_paramater::Id;
pub struct JsyMk194g<Serial: Read + Write, D: DelayNs> {
    pub device_address: Id,
    pub(crate) serial: Serial,
    pub(crate) delay: D,
}

impl<Serial: Read + Write, D: DelayNs> JsyMk194g<Serial, D> {
    pub fn new(serial: Serial, device_address: Id, delay: D) -> Self {
        Self {
            serial,
            device_address,
            delay,
        }
    }
    #[maybe_async::maybe_async]
    pub async fn new_default(serial: Serial, delay: D) -> Result<Self, JSYMk194Error> {
        // Check if device is on the bus
        let mut device = Self {
            serial,
            device_address: Id::default(),
            delay,
        };
        // Check if we can get the ID
        device.get_id().await?;
        Ok(device)
    }
}
