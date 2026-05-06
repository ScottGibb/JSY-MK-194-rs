use crate::{
    REQUEST_RESPONSE_DELAY,
    error::JSYMk194Error,
    hal::*,
    jsy_mk_194g::JsyMk194g,
    modbus::{
        offsets::{
            CHANNEL_READ_RESPONSE_HEADER_SIZE, ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES,
            FULL_READ_RESPONSE_HEADER_SIZE, MODBUS_DATA_START_OFFSET,
        },
        protocol::{
            construct_channel_read_request, construct_full_read_request,
            extract_modbus_response_header,
        },
    },
    registers::{
        channel_one_measuring_electrical_paramaters::{
            FirstChannelActivePowerRegister, FirstChannelCurrentRegister,
            FirstChannelVoltageRegister,
        },
        channel_two_measuring_electrical_paramaters::{
            SecondChannelActivePowerRegister, SecondChannelCurrentRegister,
            SecondChannelNegativeActiveEnergyRegister, SecondChannelPositiveActiveEnergyRegister,
            SecondChannelPowerFactorRegister, SecondChannelVoltageRegister,
        },
        misc_registers::{FrequencyRegister, PowerDirectionRegister},
        traits::Register,
    },
    types::{Channel, ChannelStatistics, Statistics},
    units::*,
};

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

        self.extract_statistics(&response_buff)
    }
    #[maybe_async::maybe_async]
    fn extract_statistics(&mut self, buffer: &[u8]) -> Result<Statistics, JSYMk194Error> {
        if buffer.len() < FULL_READ_RESPONSE_HEADER_SIZE {
            return Err(JSYMk194Error::InvalidResponse);
        }
        let _ = extract_modbus_response_header(buffer)?;

        let first_channel_voltage_register = FirstChannelVoltageRegister::from_bytes(
            &buffer[MODBUS_DATA_START_OFFSET
                ..MODBUS_DATA_START_OFFSET + ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES],
        )
        .get_scaled_value();
        let first_channel_current = FirstChannelCurrentRegister::from_bytes(
            &buffer[MODBUS_DATA_START_OFFSET + ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES
                ..MODBUS_DATA_START_OFFSET + (2 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
        )
        .get_scaled_value();
        let first_channel_active_power = FirstChannelActivePowerRegister::from_bytes(
            &buffer[MODBUS_DATA_START_OFFSET + (2 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                ..MODBUS_DATA_START_OFFSET + (3 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
        )
        .get_scaled_value();
        let first_channel_positive_active_energy =
            SecondChannelPositiveActiveEnergyRegister::from_bytes(
                &buffer[MODBUS_DATA_START_OFFSET + (3 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                    ..MODBUS_DATA_START_OFFSET + (4 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
            )
            .get_scaled_value();
        let first_channel_power_factor = SecondChannelPowerFactorRegister::from_bytes(
            &buffer[MODBUS_DATA_START_OFFSET + (4 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                ..MODBUS_DATA_START_OFFSET + (5 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
        )
        .get_scaled_value();
        let first_channel_negative_active_energy =
            SecondChannelNegativeActiveEnergyRegister::from_bytes(
                &buffer[MODBUS_DATA_START_OFFSET + (5 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                    ..MODBUS_DATA_START_OFFSET + (6 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
            )
            .get_scaled_value();

        let power_direction = PowerDirectionRegister::from_bytes(
            &buffer[MODBUS_DATA_START_OFFSET + (6 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                ..MODBUS_DATA_START_OFFSET + (7 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
        );

        let frequency = FrequencyRegister::from_bytes(
            &buffer[MODBUS_DATA_START_OFFSET + (7 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                ..MODBUS_DATA_START_OFFSET + (8 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
        )
        .get_scaled_value();

        let second_channel_voltage = SecondChannelVoltageRegister::from_bytes(
            &buffer[MODBUS_DATA_START_OFFSET + (8 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                ..MODBUS_DATA_START_OFFSET + (9 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
        )
        .get_scaled_value();
        let second_channel_current = SecondChannelCurrentRegister::from_bytes(
            &buffer[MODBUS_DATA_START_OFFSET + (9 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                ..MODBUS_DATA_START_OFFSET + (10 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
        )
        .get_scaled_value();
        let second_channel_active_power = SecondChannelActivePowerRegister::from_bytes(
            &buffer[MODBUS_DATA_START_OFFSET + (10 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                ..MODBUS_DATA_START_OFFSET + (11 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
        )
        .get_scaled_value();

        let second_channel_positive_active_energy =
            SecondChannelPositiveActiveEnergyRegister::from_bytes(
                &buffer[MODBUS_DATA_START_OFFSET + (11 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                    ..MODBUS_DATA_START_OFFSET + (12 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
            )
            .get_scaled_value();
        let second_channel_power_factor = SecondChannelPowerFactorRegister::from_bytes(
            &buffer[MODBUS_DATA_START_OFFSET + (12 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                ..MODBUS_DATA_START_OFFSET + (13 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
        )
        .get_scaled_value();
        let second_channel_negative_active_energy =
            SecondChannelNegativeActiveEnergyRegister::from_bytes(
                &buffer[MODBUS_DATA_START_OFFSET + (13 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                    ..MODBUS_DATA_START_OFFSET + (14 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
            )
            .get_scaled_value();
        //make compile for now
        let _crc =
            u16::from_be_bytes(
                buffer[59..61]
                    .try_into()
                    .map_err(|_| JSYMk194Error::CrcError {
                        expected: 2,
                        actual: u16::from_be_bytes(buffer[59..61].try_into().unwrap()),
                    })?,
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

    // TODO: this function has a lot of repeated code that could be cleaned up, but I want to get it working first before I refactor it
    // Generics might be a good way to fix it
    #[maybe_async::maybe_async]
    async fn extract_channel_statistics(
        &mut self,
        buffer: &[u8],
        channel: &Channel,
    ) -> Result<ChannelStatistics, JSYMk194Error> {
        if buffer.len() < CHANNEL_READ_RESPONSE_HEADER_SIZE {
            return Err(JSYMk194Error::InvalidResponse);
        }
        let _ = extract_modbus_response_header(buffer)?;
        let (
            voltage,
            current,
            active_power,
            positive_active_energy,
            power_factor,
            negative_active_energy,
            power_direction,
        ) = match channel {
            Channel::One => (
                FirstChannelVoltageRegister::from_bytes(
                    &buffer[MODBUS_DATA_START_OFFSET
                        ..MODBUS_DATA_START_OFFSET + ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES],
                )
                .get_scaled_value(),
                FirstChannelCurrentRegister::from_bytes(
                    &buffer[MODBUS_DATA_START_OFFSET + ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES
                        ..MODBUS_DATA_START_OFFSET + (2 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
                )
                .get_scaled_value(),
                FirstChannelActivePowerRegister::from_bytes(
                    &buffer[MODBUS_DATA_START_OFFSET + (2 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                        ..MODBUS_DATA_START_OFFSET + (3 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
                )
                .get_scaled_value(),
                SecondChannelPositiveActiveEnergyRegister::from_bytes(
                    &buffer[MODBUS_DATA_START_OFFSET + (3 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                        ..MODBUS_DATA_START_OFFSET + (4 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
                )
                .get_scaled_value(),
                SecondChannelPowerFactorRegister::from_bytes(
                    &buffer[MODBUS_DATA_START_OFFSET + (4 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                        ..MODBUS_DATA_START_OFFSET + (5 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
                )
                .get_scaled_value(),
                SecondChannelNegativeActiveEnergyRegister::from_bytes(
                    &buffer[MODBUS_DATA_START_OFFSET + (5 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                        ..MODBUS_DATA_START_OFFSET + (6 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
                )
                .get_scaled_value(),
                PowerDirectionRegister::from_bytes(
                    &buffer[MODBUS_DATA_START_OFFSET + (6 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                        ..MODBUS_DATA_START_OFFSET + (7 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
                )
                .first_channel,
            ),
            Channel::Two => (
                SecondChannelVoltageRegister::from_bytes(
                    &buffer[MODBUS_DATA_START_OFFSET
                        ..MODBUS_DATA_START_OFFSET + ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES],
                )
                .get_scaled_value(),
                SecondChannelCurrentRegister::from_bytes(
                    &buffer[MODBUS_DATA_START_OFFSET + ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES
                        ..MODBUS_DATA_START_OFFSET + (2 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
                )
                .get_scaled_value(),
                SecondChannelActivePowerRegister::from_bytes(
                    &buffer[MODBUS_DATA_START_OFFSET + (2 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                        ..MODBUS_DATA_START_OFFSET + (3 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
                )
                .get_scaled_value(),
                SecondChannelPositiveActiveEnergyRegister::from_bytes(
                    &buffer[MODBUS_DATA_START_OFFSET + (3 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                        ..MODBUS_DATA_START_OFFSET + (4 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
                )
                .get_scaled_value(),
                SecondChannelPowerFactorRegister::from_bytes(
                    &buffer[MODBUS_DATA_START_OFFSET + (4 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                        ..MODBUS_DATA_START_OFFSET + (5 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
                )
                .get_scaled_value(),
                SecondChannelNegativeActiveEnergyRegister::from_bytes(
                    &buffer[MODBUS_DATA_START_OFFSET + (5 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                        ..MODBUS_DATA_START_OFFSET + (6 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
                )
                .get_scaled_value(),
                PowerDirectionRegister::from_bytes(
                    &buffer[MODBUS_DATA_START_OFFSET + (6 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)
                        ..MODBUS_DATA_START_OFFSET + (7 * ELECTRICAL_PARAMATER_REGISTER_NUM_BYTES)],
                )
                .second_channel,
            ),
        };

        Ok(ChannelStatistics {
            voltage: ElectricPotential::new::<volt>(voltage),
            current: ElectricCurrent::new::<ampere>(current),
            active_power: Power::new::<watt>(active_power),
            positive_active_energy: Energy::new::<kilowatt_hour>(positive_active_energy),
            negative_active_energy: Energy::new::<kilowatt_hour>(negative_active_energy),
            power_direction,
            power_factor,
        })
    }
}
