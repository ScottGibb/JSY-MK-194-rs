use super::protocol::{calculate_crc_bytes, create_request_modbus_header};
use crate::error::JSYMk194Error;
use crate::hal::*;
use crate::jsy_mk_194g::JsyMk194g;
use crate::modbus::offsets::{
    MODBUS_DATA_START_OFFSET, SINGLE_READ_RESPONSE_HEADER_SIZE,
    SINGLE_WRITE_REQUEST_HEADER_SIZE, SINGLE_WRITE_RESPONSE_HEADER_SIZE,
};
use crate::modbus::protocol::{
    REQUEST_RESPONSE_DELAY, construct_single_read_request,
};
use crate::modbus::types::FunctionCode;
use crate::registers::traits::{self, Register};

impl<Serial: Read + Write, D: DelayNs> JsyMk194g<Serial, D> {
    #[maybe_async::maybe_async]
    pub async fn read_register<Register>(&mut self) -> Result<Register, JSYMk194Error>
    where
        Register: traits::Register + traits::ReadRegister,
    {
        let buff = construct_single_read_request(
            self.device_address.clone(),
            Register::ADDRESS,
            Register::NUM_BYTES,
        )?;
        println!("[Modbus] Sending read request: {:02X?}", &buff);
        self.write_buffer(&buff).await?;
        self.delay
            .delay_ms(
                u32::try_from(REQUEST_RESPONSE_DELAY.as_millis())
                    .expect("This should not fail to convert"),
            )
            .await;
        let mut response_buff = [0u8; SINGLE_READ_RESPONSE_HEADER_SIZE];
        self.read_buffer(&mut response_buff).await?;
        println!("[Modbus] Received read response: {:02X?}", &response_buff);

        let register_buff = response_buff
            .get(MODBUS_DATA_START_OFFSET..(MODBUS_DATA_START_OFFSET + Register::NUM_BYTES))
            .ok_or(JSYMk194Error::InvalidResponse)?;
        println!("[Modbus] Register bytes: {register_buff:02X?}");
        Ok(Register::from_bytes(register_buff))
    }

    #[maybe_async::maybe_async]
    pub async fn write_register(
        &mut self,
        register: impl Register + traits::WriteRegister,
    ) -> Result<(), JSYMk194Error> {
        let address_header = create_request_modbus_header(
            self.device_address.clone(),
            FunctionCode::WriteOneOrMoreRegisters,
            register.address(),
        );
        let num_bytes = u16::try_from(register.num_bytes())
            .map_err(|_| JSYMk194Error::ConversionError("Invalid register size".into()))?; // Fix `This`
        if num_bytes % 2 != 0 {
            return Err(JSYMk194Error::ConversionError(
                "Invalid register size: must be a multiple of 2 bytes".into(),
            ));
        }
        let num_registers = num_bytes / 2;
        let [num_bytes_high, num_bytes_low] = num_bytes.to_be_bytes();
        let [num_registers_high, num_registers_low] = num_registers.to_be_bytes();
        match num_bytes {
            2 => {
                let mut buff = [0u8; SINGLE_WRITE_REQUEST_HEADER_SIZE + 1];
                buff[0..4].copy_from_slice(&address_header);
                buff[4] = num_registers_high;
                buff[5] = num_registers_low;
                buff[6] = num_bytes_low;
                buff[7] = num_bytes_high;
                register.to_bytes(&mut buff[7..9])?;
                let crc = calculate_crc_bytes(&buff[0..9]);
                buff[9..11].copy_from_slice(&crc);
                println!("[Modbus] Sending write request: {:02X?}", &buff);
                self.write_buffer(&buff).await?;
            }
            4 => {
                let mut buff = [0u8; SINGLE_WRITE_REQUEST_HEADER_SIZE + 3];
                buff[0..4].copy_from_slice(&address_header);
                buff[4] = num_registers_high;
                buff[5] = num_registers_low;
                buff[6] = num_bytes_low;
                register.to_bytes(&mut buff[7..11])?;
                let crc = calculate_crc_bytes(&buff[0..11]);
                buff[11..13].copy_from_slice(&crc);
                println!("[Modbus] Sending write request: {:02X?}", &buff);
                self.write_buffer(&buff).await?;
            }
            _ => {
                return Err(JSYMk194Error::ConversionError(
                    format!(
                        "Invalid register size: {} for register Address: {:02X}",
                        num_bytes,
                        u16::from(register.address())
                    ),
                ));
            }
        };
        self.delay
            .delay_ms(
                u32::try_from(REQUEST_RESPONSE_DELAY.as_millis())
                    .expect("This should not fail to convert"),
            )
            .await;
        let mut response_buff = [0u8; SINGLE_WRITE_RESPONSE_HEADER_SIZE]; // Error response is smaller than normal response, so this will work for both
        self.read_buffer(&mut response_buff).await?;
        Ok(())
    }
}
