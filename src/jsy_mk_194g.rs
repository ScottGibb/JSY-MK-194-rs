use crate::error::JSYMk194Error;
use crate::hal::*;
pub struct JsyMk194g<Serial: Read + Write> {
    pub(crate) serial: Serial,
}

impl<Serial: Read + Write> JsyMk194g<Serial> {
    pub fn new(serial: Serial) -> Self {
        Self { serial }
    }
    #[maybe_async::maybe_async]
    pub async fn new_default(serial: Serial) -> Result<Self, JSYMk194Error> {
        Ok(Self { serial })
    }
}
