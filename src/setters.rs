use crate::error::JSYMk194Error;
use crate::hal::*;
use crate::jsy_mk_194g::JsyMk194g;
use crate::registers::system_configuration_paramater::{
    Baudrate, SystemConfigurationParamaterRegister,
};
impl<Serial: Read + Write> JsyMk194g<Serial> {
    #[maybe_async::maybe_async]
    pub async fn set_baudrate(&mut self, baudrate: Baudrate) -> Result<(), JSYMk194Error> {
        let previous_register = self
            .read_register::<SystemConfigurationParamaterRegister>()
            .await?;
        let register = SystemConfigurationParamaterRegister {
            baudrate,
            id: previous_register.id, // Keep the same ID as the previous register
        };
        self.write_register(register).await?;
        Ok(())
    }
}
