use crate::{
    error::JSYMk194Error,
    hal::*,
    jsy_mk_194g::JsyMk194g,
    modbus::{
        protocol::CHANNEL_REQUEST_RESPONSE_DELAY, requests::ReadRequest, responses::ReadResponse,
    },
    registers::{
        channel_one_measuring_electrical_paramaters::{
            FirstChannelActivePowerRegister, FirstChannelCurrentRegister,
            FirstChannelPowerFactorRegister, FirstChannelVoltageRegister,
        },
        channel_two_measuring_electrical_paramaters::{
            SecondChannelActivePowerRegister, SecondChannelCurrentRegister,
            SecondChannelNegativeActiveEnergyRegister, SecondChannelPositiveActiveEnergyRegister,
            SecondChannelPowerFactorRegister, SecondChannelVoltageRegister,
        },
        misc_registers::{FrequencyRegister, PowerDirection, PowerDirectionRegister},
        traits::Register,
    },
    types::{Channel, ChannelStatistics, Statistics},
    units::*,
};

impl<Serial: Read + Write, D: DelayNs> JsyMk194g<Serial, D> {
    #[maybe_async::maybe_async]
    pub async fn read_channel_statistics(
        &mut self,
        channel: Channel,
    ) -> Result<ChannelStatistics, JSYMk194Error> {
        let read_request = match channel {
            Channel::One => ReadRequest::new(
                self.device_address.clone(),
                FirstChannelVoltageRegister::ADDRESS,
                6,
            ),
            Channel::Two => ReadRequest::new(
                self.device_address.clone(),
                SecondChannelVoltageRegister::ADDRESS,
                6,
            ),
        };
        self.write_buffer(&read_request.to_bytes()).await?;
        self.delay
            .delay_ms(
                u32::try_from(CHANNEL_REQUEST_RESPONSE_DELAY.as_millis())
                    .expect("This should not fail to convert"),
            )
            .await;

        let mut response_buff = [0u8; 256];
        let bytes_read = self.read_buffer(&mut response_buff).await?;
        let read_response = ReadResponse::from_bytes(&response_buff[..bytes_read])?;
        // Check if bytes read are enough to contain the registers we expect to read
        if bytes_read < ReadResponse::RESPONSE_HEADER_SIZE + 12 {
            return Err(JSYMk194Error::FailedToRead {
                read: bytes_read,
                expected: ReadResponse::RESPONSE_HEADER_SIZE + 12, // 14 bytes for the 6 registers we expect to read (2 bytes each)
            });
        }

        // Perform an extra read for the power direction register
        let power_direction = self.get_power_direction(channel).await?;
        self.extract_channel_statistics_from_response(read_response, power_direction)
    }

    fn extract_channel_statistics_from_response(
        &self,
        read_response: ReadResponse,
        power_direction: PowerDirection,
    ) -> Result<ChannelStatistics, JSYMk194Error> {
        let voltage = FirstChannelVoltageRegister::from_bytes(&read_response.register_data[0..4])
            .get_scaled_value();
        let current = FirstChannelCurrentRegister::from_bytes(&read_response.register_data[4..8])
            .get_scaled_value();
        let active_power =
            FirstChannelActivePowerRegister::from_bytes(&read_response.register_data[8..12])
                .get_scaled_value();
        let positive_activer_energy =
            FirstChannelActivePowerRegister::from_bytes(&read_response.register_data[12..16])
                .get_scaled_value();
        let power_factor =
            FirstChannelPowerFactorRegister::from_bytes(&read_response.register_data[16..20]);
        let negative_active_energy =
            FirstChannelActivePowerRegister::from_bytes(&read_response.register_data[20..24])
                .get_scaled_value();

        Ok(ChannelStatistics {
            voltage: ElectricPotential::new::<volt>(voltage),
            current: ElectricCurrent::new::<ampere>(current),
            active_power: Power::new::<watt>(active_power),
            positive_active_energy: Energy::new::<kilowatt_hour>(positive_activer_energy),
            negative_active_energy: Energy::new::<kilowatt_hour>(negative_active_energy),
            power_factor: power_factor.get_scaled_value(),
            power_direction,
        })
    }
}

impl<Serial: Read + Write, D: DelayNs> JsyMk194g<Serial, D> {
    #[maybe_async::maybe_async]
    pub async fn read_statistics(&mut self) -> Result<Statistics, JSYMk194Error> {
        let write_request = ReadRequest::new(
            self.device_address.clone(),
            FirstChannelVoltageRegister::ADDRESS,
            14,
        );
        self.write_buffer(&write_request.to_bytes()).await?;
        self.delay
            .delay_ms(
                u32::try_from(CHANNEL_REQUEST_RESPONSE_DELAY.as_millis())
                    .expect("This should not fail to convert"),
            )
            .await;
        let mut response_buff = [0u8; 256];
        let bytes_read = self.read_buffer(&mut response_buff).await?;
        let read_response = ReadResponse::from_bytes(&response_buff[..bytes_read])?;

        // Check if bytes read are enough to contain the registers we expect to read
        if bytes_read < ReadResponse::RESPONSE_HEADER_SIZE + 28 {
            return Err(JSYMk194Error::FailedToRead {
                read: bytes_read,
                expected: ReadResponse::RESPONSE_HEADER_SIZE + 28, // 28 bytes for the 14 registers we expect to read (2 bytes each)
            });
        }
        self.extract_statistics_from_response(read_response)
    }

    fn extract_statistics_from_response(
        &mut self,
        read_response: ReadResponse,
    ) -> Result<Statistics, JSYMk194Error> {
        let first_voltage =
            FirstChannelVoltageRegister::from_bytes(&read_response.register_data[0..4])
                .get_scaled_value();
        let first_current =
            FirstChannelCurrentRegister::from_bytes(&read_response.register_data[4..8])
                .get_scaled_value();
        let first_active_power =
            FirstChannelActivePowerRegister::from_bytes(&read_response.register_data[8..12])
                .get_scaled_value();
        let first_positive_active_energy =
            FirstChannelActivePowerRegister::from_bytes(&read_response.register_data[12..16])
                .get_scaled_value();
        let first_power_factor =
            FirstChannelPowerFactorRegister::from_bytes(&read_response.register_data[16..20]);
        let first_negative_active_energy =
            FirstChannelActivePowerRegister::from_bytes(&read_response.register_data[20..24])
                .get_scaled_value();

        let power_direction_register =
            PowerDirectionRegister::from_bytes(&read_response.register_data[24..28]);
        let frequency =
            FrequencyRegister::from_bytes(&read_response.register_data[28..32]).get_scaled_value();

        let second_voltage =
            SecondChannelVoltageRegister::from_bytes(&read_response.register_data[32..36])
                .get_scaled_value();
        let second_current =
            SecondChannelCurrentRegister::from_bytes(&read_response.register_data[36..40])
                .get_scaled_value();
        let second_active_power =
            SecondChannelActivePowerRegister::from_bytes(&read_response.register_data[40..44])
                .get_scaled_value();
        let second_positive_active_energy = SecondChannelPositiveActiveEnergyRegister::from_bytes(
            &read_response.register_data[44..48],
        )
        .get_scaled_value();
        let second_power_factor =
            SecondChannelPowerFactorRegister::from_bytes(&read_response.register_data[48..52]);
        let second_negative_active_energy = SecondChannelNegativeActiveEnergyRegister::from_bytes(
            &read_response.register_data[52..56],
        )
        .get_scaled_value();
        Ok(Statistics {
            channel_one: ChannelStatistics {
                voltage: ElectricPotential::new::<volt>(first_voltage),
                current: ElectricCurrent::new::<ampere>(first_current),
                active_power: Power::new::<watt>(first_active_power),
                positive_active_energy: Energy::new::<kilowatt_hour>(first_positive_active_energy),
                negative_active_energy: Energy::new::<kilowatt_hour>(first_negative_active_energy),
                power_factor: first_power_factor.get_scaled_value(),
                power_direction: power_direction_register.first_channel,
            },
            channel_two: ChannelStatistics {
                voltage: ElectricPotential::new::<volt>(second_voltage),
                current: ElectricCurrent::new::<ampere>(second_current),
                active_power: Power::new::<watt>(second_active_power),
                positive_active_energy: Energy::new::<kilowatt_hour>(second_positive_active_energy),
                negative_active_energy: Energy::new::<kilowatt_hour>(second_negative_active_energy),
                power_factor: second_power_factor.get_scaled_value(),
                power_direction: power_direction_register.second_channel,
            },
            frequency: Frequency::new::<hertz>(frequency),
        })
    }
}
