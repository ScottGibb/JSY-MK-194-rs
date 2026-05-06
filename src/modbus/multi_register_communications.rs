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
        channel_two_measuring_electrical_paramaters::SecondChannelVoltageRegister,
        misc_registers::PowerDirection,
        traits::Register,
    },
    types::{Channel, ChannelStatistics},
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
        self.extract_statistics_from_response(read_response, power_direction)
    }

    fn extract_statistics_from_response(
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
