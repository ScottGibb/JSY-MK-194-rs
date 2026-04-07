use crate::error::JSYMk194Error;
use crate::hal::*;
pub struct JsyMk194g<Serial: Read + Write> {
    pub device_address: u8,
    pub(crate) serial: Serial,
}

impl<Serial: Read + Write> JsyMk194g<Serial> {
    pub fn new(serial: Serial, device_address: u8) -> Self {
        Self {
            serial,
            device_address,
        }
    }
    #[maybe_async::maybe_async]
    pub async fn new_default(serial: Serial) -> Result<Self, JSYMk194Error> {
        Ok(Self {
            serial,
            device_address: 0,
        })
    }
}
