use crate::error::JSYMk194Error;
use crate::jsy_mk_194g::JsyMk194g;
use crate::modubus::types::ReadDataFrame;
use crate::registers::traits::{ReadRegister, ToBeBytes};
use crate::{hal::*, modubus::types::FunctionCode};

impl<Serial: Read + Write> JsyMk194g<Serial> {
    // #[maybe_async::maybe_async]
    // pub async fn write_register<S>(
    //     &mut self,
    //     register: impl WriteRegister<S>,
    // ) -> Result<(), JSYMk194Error>
    // where
    //     S: Sized + ToBeBytes,
    //     <S as ToBeBytes>::Bytes: AsRef<[u8]>,
    // {
    //     let command = register.get_address();
    //     let bytes = register.raw_value().to_be_bytes();
    //     let data_frame = WriteDataFrame {
    //         device_address: command,
    //         function_code: FunctionCode::WriteOneOrMoreRegisters,
    //         data: bytes.as_ref(),
    //         crc: CylicRedundanyCheck::calculate(register.raw_value().to_be_bytes().as_ref()),
    //     };

    //     let result = self.serial.write(data_frame.data).await;
    //     match result {
    //         Ok(bytes_written) => {
    //             if bytes_written == data_frame.data.len() {
    //                 Ok(())
    //             } else {
    //                 // Something has went wrong when writing to the buffer
    //                 Err(JSYMk194Error::FailedToWrite(
    //                     data_frame.data.len() - bytes_written,
    //                 ))
    //             }
    //         }
    //         Err(e) => Err(JSYMk194Error::from(e)),
    //     }
    // }

    #[maybe_async::maybe_async]
    pub async fn read_register<Register, S>(&mut self) -> Result<Register, JSYMk194Error>
    where
        Register: ReadRegister<S>,
        S: Sized + ToBeBytes,
    {
        let command = u16::from(Register::address());
        let mut data = command.to_be_bytes();

        todo!()
        // Now we need to read the response from the device
    }
}
