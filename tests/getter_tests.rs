mod common;
use common::setup_device;
use jsy_mk_194_rs::registers::misc_registers::PowerDirection;
use jsy_mk_194_rs::types::Channel;
use jsy_mk_194_rs::types::SystemParameters;
use jsy_mk_194_rs::types::{Baudrate, Id};
use jsy_mk_194_rs::units::volt;
use jsy_mk_194_rs::units::{Energy, Power};
use jsy_mk_194_rs::units::{ampere, kilowatt_hour, watt};
use jsy_mk_194_rs::{
    types::ChannelStatistics,
    units::{ElectricCurrent, ElectricPotential},
};
use jsy_mk_194_rs::{
    types::Statistics,
    units::{Frequency, hertz},
};

fn fresh_channel_statistics() -> ChannelStatistics {
    ChannelStatistics {
        voltage: ElectricPotential::new::<volt>(0.0),
        current: ElectricCurrent::new::<ampere>(0.0),
        active_power: Power::new::<watt>(0.0),
        positive_active_energy: Energy::new::<kilowatt_hour>(0.0),
        negative_active_energy: Energy::new::<kilowatt_hour>(0.0),
        power_factor: 0.0,
        power_direction: PowerDirection::Positive,
    }
}

mod fresh_device_tests {

    use jsy_mk_194_rs::registers::misc_registers::PowerDirection;

    use super::*;

    #[test]
    fn test_get_id() {
        let mut device = setup_device(Id::default(), Baudrate::default());
        let id = device.get_id().expect("This should not fail");
        println!("device Id {id:?}");
        assert_eq!(id, Id::default())
    }

    #[test]
    fn test_baudrate() {
        let mut device = setup_device(Id::default(), Baudrate::default());
        let baudrate = device.get_baudrate().expect("This should not fail");
        println!("Baudrate: {baudrate:?}");
        assert_eq!(baudrate, Baudrate::default());
    }

    #[test]
    fn get_all_channels_test() {
        let mut device = setup_device(Id::default(), Baudrate::default());
        let fresh_device_all_channels: Statistics = Statistics {
            channel_one: fresh_channel_statistics(),
            channel_two: fresh_channel_statistics(),
            frequency: Frequency::new::<hertz>(0.0),
        };
        let all_channels = device
            .get_all_channels()
            .expect("Failed to get all channels");

        assert_eq!(all_channels, fresh_device_all_channels);
    }

    #[test]
    fn get_channel_one_test() {
        let mut device = setup_device(Id::default(), Baudrate::default());
        let channel_one = device
            .get_channel(Channel::One)
            .expect("This should not fail");

        assert_eq!(channel_one, fresh_channel_statistics());
    }

    #[test]
    fn get_channel_two_test() {
        let mut device = setup_device(Id::default(), Baudrate::default());
        let channel_two = device
            .get_channel(Channel::Two)
            .expect("This should not fail");

        assert_eq!(channel_two, fresh_channel_statistics());
    }

    #[test]
    fn get_frequency_test() {
        let mut device = setup_device(Id::default(), Baudrate::default());
        let frequency = device.get_frequency().expect("This should not fail");
        assert_eq!(frequency, Frequency::new::<hertz>(0.0));
    }

    #[test]
    fn get_system_paramaters_tests() {
        let mut device = setup_device(Id::default(), Baudrate::default());
        let system_paramaters = device
            .get_system_parameters()
            .expect("This should not fail");

        assert_eq!(system_paramaters, SystemParameters::default())
    }

    #[test]
    fn get_power_direction_test() {
        let mut device = setup_device(Id::default(), Baudrate::default());
        let power_direction_channel_one = device
            .get_power_direction(Channel::One)
            .expect("This should not fail");
        let power_direction_channel_two = device
            .get_power_direction(Channel::Two)
            .expect("This should not fail");
        println!("Power Direction Channel One: {power_direction_channel_one:?}");
        println!("Power Direction Channel Two: {power_direction_channel_two:?}");
        // There is no default value for this register, so we can't assert against it. Just check that it can be read without error.
        assert_eq!(power_direction_channel_one, PowerDirection::Positive);
        assert_eq!(power_direction_channel_two, PowerDirection::Positive);
    }
}

//TODO Add Mains Tests
