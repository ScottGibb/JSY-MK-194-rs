use crate::error::JSYMk194Error;
use crate::hal::*;
use crate::jsy_mk_194g::JsyMk194g;
use crate::modubus::types::FunctionCode;
use crate::registers::traits::{self, Register};

const SINGLE_READ_REQUEST_HEADER_SIZE: usize = 10;
const SINGLE_READ_RESPONSE_HEADER_SIZE: usize = 8;
impl<Serial: Read + Write> JsyMk194g<Serial> {
    #[maybe_async::maybe_async]
    pub async fn read_register<Register>(&mut self) -> Result<Register, JSYMk194Error>
    where
        Register: traits::Register + traits::ReadRegister,
    {
        let num_bytes =
            u16::try_from(Register::NUM_BYTES).map_err(|_| JSYMk194Error::ConversionError)?; // Fix `This`

        let [num_bytes_high, num_bytes_low] = num_bytes.to_be_bytes();
        let [register_address_high, register_address_low] =
            u16::from(Register::ADDRESS).to_be_bytes();
        let mut buff = [0u8; SINGLE_READ_REQUEST_HEADER_SIZE];

        buff[0] = self.device_address;
        buff[1] = u8::from(FunctionCode::ReadOneOrMoreRegisters);
        buff[2] = register_address_high;
        buff[3] = register_address_low;

        buff[4] = num_bytes_high;
        buff[5] = num_bytes_low;

        let crc: u16 = 0; // TODO: Implement CRC calculation

        let [crc_low, crc_high] = crc.to_le_bytes();
        buff[6] = crc_low;
        buff[7] = crc_high;
        self.serial.write(&buff)?;
        let mut respone_buff = [0u8; SINGLE_READ_RESPONSE_HEADER_SIZE];
        self.serial.read(&mut respone_buff)?;
        Ok(Register::from_bytes(&respone_buff))
    }

    #[maybe_async::maybe_async]
    pub async fn write_register(
        &mut self,
        register: impl Register + traits::WriteRegister,
    ) -> Result<(), JSYMk194Error> {
        let register_address = u16::from(register.address());
        let num_bytes =
            u16::try_from(register.num_bytes()).map_err(|_| JSYMk194Error::ConversionError)?; // Fix `This`
        let [register_address_high, register_address_low] = register_address.to_be_bytes();
        let [num_bytes_high, num_bytes_low] = num_bytes.to_be_bytes();
        match num_bytes {
            2 => {
                let mut buff = [0u8; 10];
                buff[0] = self.device_address;
                buff[1] = u8::from(FunctionCode::WriteOneOrMoreRegisters);
                buff[2] = register_address_high;
                buff[3] = register_address_low;
                buff[4] = num_bytes_high;
                buff[5] = num_bytes_low;
                register.to_bytes(&mut buff[6..8])?;
                let crc: u16 = 0; // TODO: Implement CRC calculation
                let [crc_low, crc_high] = crc.to_le_bytes();
                buff[8] = crc_low;
                buff[9] = crc_high;
                self.serial.write(&buff)?;
            }
            4 => {
                let mut buff = [0u8; 12];
                buff[0] = self.device_address;
                buff[1] = u8::from(FunctionCode::WriteOneOrMoreRegisters);
                buff[2] = register_address_high;
                buff[3] = register_address_low;
                buff[4] = num_bytes_high;
                buff[5] = num_bytes_low;
                register.to_bytes(&mut buff[6..10])?;
                let crc: u16 = 0; // TODO: Implement CRC calculation
                let [crc_low, crc_high] = crc.to_le_bytes();
                buff[10] = crc_low;
                buff[11] = crc_high;
                self.serial.write(&buff)?;
            }
            _ => return Err(JSYMk194Error::ConversionError),
        };
        Ok(())
    }
}
