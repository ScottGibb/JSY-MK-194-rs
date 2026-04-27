use super::protocol::{calculate_crc_bytes, create_request_modbus_header};
use crate::error::JSYMk194Error;
use crate::hal::*;
use crate::jsy_mk_194g::JsyMk194g;
use crate::modbus::protocol::{
    CHANNEL_READ_RESPONSE_HEADER_SIZE, FULL_READ_RESPONSE_HEADER_SIZE, ModbusErrorResponse,
    REQUEST_RESPONSE_DELAY, SINGLE_READ_RESPONSE_HEADER_SIZE, SINGLE_WRITE_REQUEST_HEADER_SIZE,
    SINGLE_WRITE_RESPONSE_HEADER_SIZE, construct_channel_read_request, construct_full_read_request,
    construct_single_read_request,
};
use crate::modbus::types::FunctionCode;
use crate::registers::channel_one_measuring_electrical_paramaters::{
    FirstChannelActivePowerRegister, FirstChannelCurrentRegister, FirstChannelVoltageRegister,
};
use crate::registers::channel_two_measuring_electrical_paramaters::{
    SecondChannelActivePowerRegister, SecondChannelCurrentRegister,
    SecondChannelNegativeActiveEnergyRegister, SecondChannelPositiveActiveEnergyRegister,
    SecondChannelPowerFactorRegister, SecondChannelVoltageRegister,
};
use crate::registers::misc_registers::{FrequencyRegister, PowerDirectionRegister};
use crate::registers::traits::{self, Register};
use crate::types::{Channel, ChannelStatistics, Id, Statistics};
use crate::units::{
    ElectricCurrent, ElectricPotential, Energy, Frequency, Power, ampere, hertz, kilowatt_hour,
    volt, watt,
};

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
        self.write_buffer(&buff).await?;
        self.delay
            .delay_ms(
                u32::try_from(REQUEST_RESPONSE_DELAY.as_millis())
                    .expect("This should not fail to convert"),
            )
            .await;
        let mut response_buff = [0u8; SINGLE_READ_RESPONSE_HEADER_SIZE];
        self.read_buffer(&mut response_buff).await?;

        let register_buff = response_buff
            .get(3..(3 + Register::NUM_BYTES))
            .ok_or(JSYMk194Error::InvalidResponse)?;
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
                self.write_buffer(&buff).await?;
            }
            _ => {
                return Err(JSYMk194Error::ConversionError(
                    "Invalid register size".into(),
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

    #[maybe_async::maybe_async]
    async fn write_buffer(&mut self, buffer: &[u8]) -> Result<(), JSYMk194Error> {
        let bytes_written = self.serial.write(buffer).await?;
        if bytes_written < buffer.len() {
            return Err(JSYMk194Error::FailedToWrite {
                written: bytes_written,
                expected: buffer.len(),
            });
        }
        Ok(())
    }
    #[maybe_async::maybe_async]
    async fn read_buffer(&mut self, buffer: &mut [u8]) -> Result<(), JSYMk194Error> {
        let bytes_read = self.serial.read(buffer).await?;
        if bytes_read == ModbusErrorResponse::ERROR_RESPONSE_HEADER_SIZE {
            let error_code = ModbusErrorResponse::from_bytes(
                &buffer[..ModbusErrorResponse::ERROR_RESPONSE_HEADER_SIZE],
            )?
            .error_code;
            return Err(JSYMk194Error::DeviceError(error_code));
        }
        if bytes_read < buffer.len() {
            return Err(JSYMk194Error::FailedToRead {
                read: bytes_read,
                expected: buffer.len(),
            });
        }
        Ok(())
    }
}

impl<Serial: Read + Write, D: DelayNs> JsyMk194g<Serial, D> {
    #[maybe_async::maybe_async]
    pub async fn read_statistics(&mut self) -> Result<Statistics, JSYMk194Error> {
        let read_request = construct_full_read_request(self.device_address.clone())?;
        self.write_buffer(&read_request).await?;
        self.delay
            .delay_ms(
                u32::try_from(REQUEST_RESPONSE_DELAY.as_millis())
                    .expect("This should not fail to convert"),
            )
            .await;

        let mut response_buff = [0u8; FULL_READ_RESPONSE_HEADER_SIZE];
        self.read_buffer(&mut response_buff).await?;

        Ok(self.extract_statistics(&response_buff).await?)
    }
    #[maybe_async::maybe_async]
    async fn extract_statistics(&mut self, buffer: &[u8]) -> Result<Statistics, JSYMk194Error> {
        if buffer.len() < FULL_READ_RESPONSE_HEADER_SIZE {
            return Err(JSYMk194Error::InvalidResponse);
        }
        let _device_id = Id::new(buffer[0])?;
        let _function_code = FunctionCode::try_from(buffer[1])?;
        let _byte_count = buffer[2] as usize;

        let first_channel_voltage_register =
            FirstChannelVoltageRegister::from_bytes(&buffer[3..7]).get_scaled_value();
        let first_channel_current =
            FirstChannelCurrentRegister::from_bytes(&buffer[7..11]).get_scaled_value();
        let first_channel_active_power =
            FirstChannelActivePowerRegister::from_bytes(&buffer[11..15]).get_scaled_value();
        let first_channel_positive_active_energy =
            SecondChannelPositiveActiveEnergyRegister::from_bytes(&buffer[15..19])
                .get_scaled_value();
        let first_channel_power_factor =
            SecondChannelPowerFactorRegister::from_bytes(&buffer[19..23]).get_scaled_value();
        let first_channel_negative_active_energy =
            SecondChannelNegativeActiveEnergyRegister::from_bytes(&buffer[23..27])
                .get_scaled_value();

        let power_direction = PowerDirectionRegister::from_bytes(&buffer[27..31]);

        let frequency = FrequencyRegister::from_bytes(&buffer[31..35]).get_scaled_value();

        let second_channel_voltage =
            SecondChannelVoltageRegister::from_bytes(&buffer[35..39]).get_scaled_value();
        let second_channel_current =
            SecondChannelCurrentRegister::from_bytes(&buffer[39..43]).get_scaled_value();
        let second_channel_active_power =
            SecondChannelActivePowerRegister::from_bytes(&buffer[43..47]).get_scaled_value();

        let second_channel_positive_active_energy =
            SecondChannelPositiveActiveEnergyRegister::from_bytes(&buffer[47..51])
                .get_scaled_value();
        let second_channel_power_factor =
            SecondChannelPowerFactorRegister::from_bytes(&buffer[51..55]).get_scaled_value();
        let second_channel_negative_active_energy =
            SecondChannelNegativeActiveEnergyRegister::from_bytes(&buffer[55..59])
                .get_scaled_value();

        let _crc = u16::from_be_bytes(
            buffer[59..61]
                .try_into()
                .map_err(|_| JSYMk194Error::CrcError)?,
        );

        Ok(Statistics {
            channel_one: ChannelStatistics {
                voltage: ElectricPotential::new::<volt>(first_channel_voltage_register),
                current: ElectricCurrent::new::<ampere>(first_channel_current),
                active_power: Power::new::<watt>(first_channel_active_power),
                positive_active_energy: Energy::new::<kilowatt_hour>(
                    first_channel_positive_active_energy,
                ),
                power_factor: first_channel_power_factor,
                negative_active_energy: Energy::new::<kilowatt_hour>(
                    first_channel_negative_active_energy,
                ),
                power_direction: power_direction.first_channel,
            },
            channel_two: ChannelStatistics {
                voltage: ElectricPotential::new::<volt>(second_channel_voltage),
                current: ElectricCurrent::new::<ampere>(second_channel_current),
                active_power: Power::new::<watt>(second_channel_active_power),
                positive_active_energy: Energy::new::<kilowatt_hour>(
                    second_channel_positive_active_energy,
                ),
                power_factor: second_channel_power_factor,
                negative_active_energy: Energy::new::<kilowatt_hour>(
                    second_channel_negative_active_energy,
                ),
                power_direction: power_direction.second_channel,
            },
            frequency: Frequency::new::<hertz>(frequency),
        })
    }
}

impl<Serial: Read + Write, D: DelayNs> JsyMk194g<Serial, D> {
    #[maybe_async::maybe_async]
    pub async fn read_channel_statistics(
        &mut self,
        channel: Channel,
    ) -> Result<ChannelStatistics, JSYMk194Error> {
        let read_request =
            construct_channel_read_request(self.device_address.clone(), channel.clone())?;
        self.write_buffer(&read_request).await?;
        self.delay
            .delay_ms(
                u32::try_from(REQUEST_RESPONSE_DELAY.as_millis())
                    .expect("This should not fail to convert"),
            )
            .await;

        let mut response_buff = [0u8; CHANNEL_READ_RESPONSE_HEADER_SIZE];
        self.read_buffer(&mut response_buff).await?;

        let statistics: ChannelStatistics = self
            .extract_channel_statistics(&response_buff, &channel)
            .await?;
        Ok(statistics)
    }

    #[maybe_async::maybe_async]
    async fn extract_channel_statistics(
        &mut self,
        buffer: &[u8],
        channel: &Channel,
    ) -> Result<ChannelStatistics, JSYMk194Error> {
        if buffer.len() < CHANNEL_READ_RESPONSE_HEADER_SIZE {
            return Err(JSYMk194Error::InvalidResponse);
        }
        let _device_id = Id::new(buffer[0])?;
        let _function_code = FunctionCode::try_from(buffer[1])?;
        let _byte_count = buffer[2] as usize;

        let voltage = if *channel == Channel::One {
            FirstChannelVoltageRegister::from_bytes(&buffer[3..7]).get_scaled_value()
        } else {
            SecondChannelVoltageRegister::from_bytes(&buffer[3..7]).get_scaled_value()
        };
        let current = if *channel == Channel::One {
            FirstChannelCurrentRegister::from_bytes(&buffer[7..11]).get_scaled_value()
        } else {
            SecondChannelCurrentRegister::from_bytes(&buffer[7..11]).get_scaled_value()
        };
        let active_power = if *channel == Channel::One {
            FirstChannelActivePowerRegister::from_bytes(&buffer[11..15]).get_scaled_value()
        } else {
            SecondChannelActivePowerRegister::from_bytes(&buffer[11..15]).get_scaled_value()
        };
        let positive_active_energy = if *channel == Channel::One {
            SecondChannelPositiveActiveEnergyRegister::from_bytes(&buffer[15..19])
                .get_scaled_value()
        } else {
            SecondChannelPositiveActiveEnergyRegister::from_bytes(&buffer[15..19])
                .get_scaled_value()
        };
        let power_factor = if *channel == Channel::One {
            SecondChannelPowerFactorRegister::from_bytes(&buffer[19..23]).get_scaled_value()
        } else {
            SecondChannelPowerFactorRegister::from_bytes(&buffer[19..23]).get_scaled_value()
        };
        let negative_active_energy = if *channel == Channel::One {
            SecondChannelNegativeActiveEnergyRegister::from_bytes(&buffer[23..27])
                .get_scaled_value()
        } else {
            SecondChannelNegativeActiveEnergyRegister::from_bytes(&buffer[23..27])
                .get_scaled_value()
        };

        let power_direction = PowerDirectionRegister::from_bytes(&buffer[27..31]);

        Ok(ChannelStatistics {
            voltage: ElectricPotential::new::<volt>(voltage),
            current: ElectricCurrent::new::<ampere>(current),
            active_power: Power::new::<watt>(active_power),
            positive_active_energy: Energy::new::<kilowatt_hour>(positive_active_energy),
            negative_active_energy: Energy::new::<kilowatt_hour>(negative_active_energy),
            power_direction: if *channel == Channel::One {
                power_direction.first_channel
            } else {
                power_direction.second_channel
            },
            power_factor,
        })
    }
}
