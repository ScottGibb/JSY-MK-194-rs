use std::time::Duration;

use jsy_mk_194_rs::{
    jsy_mk_194g::JsyMk194g,
    registers::{
        channel_one_measuring_electrical_paramaters::{
            FirstChannelCurrentRegister, FirstChannelVoltageRegister,
        },
        channel_two_measuring_electrical_paramaters::{
            SecondChannelCurrentRegister, SecondChannelVoltageRegister,
        },
        system_configuration_paramater::Baudrate,
    },
};
use serialport::SerialPort;

const TEST_PORT: &str = "/dev/tty.usbserial-0001";
fn setup_device() -> JsyMk194g<Box<dyn SerialPort>, utils::StdDelay> {
    let port = serialport::new(TEST_PORT, u32::from(Baudrate::default()))
        .timeout(Duration::from_secs(1))
        .open()
        .expect("Failed to open port");
    let delay = utils::StdDelay;
    JsyMk194g::new_default(port, delay).expect("Device should initialise")
}

mod mains_tests {
    use super::*;
    #[test]
    fn test_get_first_channel_voltage_register_mains_uk() {
        let mut device = setup_device();
        let first_channel_voltage_register = device
            .read_register::<FirstChannelVoltageRegister>()
            .expect("Failed to read First Channel Voltage register");
        println!(
            "First Channel Voltage: {} V",
            first_channel_voltage_register.get_scaled_value()
        );
        // The voltage should be around 230V, but we can allow for some variation.
        const MIN_VOLTAGE: f32 = 200.0;
        const MAX_VOLTAGE: f32 = 260.0;
        assert!(
            first_channel_voltage_register.get_scaled_value() >= MIN_VOLTAGE
                && first_channel_voltage_register.get_scaled_value() <= MAX_VOLTAGE,
            "First Channel Voltage {} V is out of expected range ({} - {} V)",
            first_channel_voltage_register.get_scaled_value(),
            MIN_VOLTAGE,
            MAX_VOLTAGE
        );
    }

    #[test]
    fn test_get_second_channel_voltage_register_mains_uk() {
        let mut device = setup_device();
        let second_channel_voltage_register = device
            .read_register::<SecondChannelVoltageRegister>()
            .expect("Failed to read Second Channel Voltage register");
        println!(
            "Second Channel Voltage: {} V",
            second_channel_voltage_register.get_scaled_value()
        );
        // The voltage should be around 230V, but we can allow for some variation.
        const MIN_VOLTAGE: f32 = 200.0;
        const MAX_VOLTAGE: f32 = 260.0;
        assert!(
            second_channel_voltage_register.get_scaled_value() >= MIN_VOLTAGE
                && second_channel_voltage_register.get_scaled_value() <= MAX_VOLTAGE,
            "Second Channel Voltage {} V is out of expected range ({} - {} V)",
            second_channel_voltage_register.get_scaled_value(),
            MIN_VOLTAGE,
            MAX_VOLTAGE
        );
    }
}

mod no_load_tests {
    use jsy_mk_194_rs::registers::{
        channel_one_measuring_electrical_paramaters::{
            FirstChannelActivePowerRegister, FirstChannelPowerFactorRegister,
        },
        channel_two_measuring_electrical_paramaters::{
            SecondChannelActivePowerRegister, SecondChannelPowerFactorRegister,
        },
    };

    use super::*;
    #[test]
    fn test_get_first_channel_voltage_register_no_mains() {
        let mut device = setup_device();
        let first_channel_voltage_register = device
            .read_register::<FirstChannelVoltageRegister>()
            .expect("Failed to read First Channel Voltage register");
        println!(
            "First Channel Voltage: {} V",
            first_channel_voltage_register.get_scaled_value()
        );
        // If the device is not connected to mains power, the voltage should be 0V.
        assert_eq!(
            first_channel_voltage_register.get_scaled_value(),
            0.0,
            "First Channel Voltage should be 0 V when not connected to mains power"
        );
    }

    #[test]
    fn test_get_first_channel_current_register_no_load() {
        let mut device = setup_device();
        let first_channel_current_register = device
            .read_register::<FirstChannelCurrentRegister>()
            .expect("Failed to read First Channel Current register");
        println!(
            "First Channel Current: {} A",
            first_channel_current_register.get_scaled_value()
        );
        // If there is no load connected, the current should be 0A.
        assert_eq!(
            first_channel_current_register.get_scaled_value(),
            0.0,
            "First Channel Current should be 0 A when no load is connected"
        );
    }

    #[test]
    fn test_get_first_channel_active_power_no_load() {
        let mut device = setup_device();
        let first_channel_active_power_register = device
            .read_register::<FirstChannelActivePowerRegister>()
            .expect("Failed to read First Channel Active Power register");
        println!(
            "First Channel Active Power: {} W",
            first_channel_active_power_register.get_scaled_value()
        );
        // If there is no load connected, the active power should be 0W.
        assert_eq!(
            first_channel_active_power_register.get_scaled_value(),
            0.0,
            "First Channel Active Power should be 0 W when no load is connected"
        );
    }

    #[test]
    fn test_get_first_channel_power_factor_no_load() {
        let mut device = setup_device();
        let first_channel_power_factor_register = device
            .read_register::<FirstChannelPowerFactorRegister>()
            .expect("Failed to read First Channel Power Factor register");
        println!(
            "First Channel Power Factor: {}",
            first_channel_power_factor_register.get_scaled_value()
        );
        // If there is no load connected, the power factor should be 0.
        assert_eq!(
            first_channel_power_factor_register.get_scaled_value(),
            0.0,
            "First Channel Power Factor should be 0 when no load is connected"
        );
    }

    #[test]
    fn test_get_second_channel_voltage_register_no_mains() {
        let mut device = setup_device();
        let second_channel_voltage_register = device
            .read_register::<SecondChannelVoltageRegister>()
            .expect("Failed to read Second Channel Voltage register");
        println!(
            "Second Channel Voltage: {} V",
            second_channel_voltage_register.get_scaled_value()
        );
        // If the device is not connected to mains power, the voltage should be 0V.
        assert_eq!(
            second_channel_voltage_register.get_scaled_value(),
            0.0,
            "Second Channel Voltage should be 0 V when not connected to mains power"
        );
    }

    #[test]
    fn test_get_second_channel_current_register_no_load() {
        let mut device = setup_device();
        let second_channel_current_register = device
            .read_register::<SecondChannelCurrentRegister>()
            .expect("Failed to read Second Channel Current register");
        println!(
            "Second Channel Current: {} A",
            second_channel_current_register.get_scaled_value()
        );
        // If there is no load connected, the current should be 0A.
        assert_eq!(
            second_channel_current_register.get_scaled_value(),
            0.0,
            "Second Channel Current should be 0 A when no load is connected"
        );
    }

    #[test]
    fn test_get_second_channel_active_power_no_load() {
        let mut device = setup_device();
        let second_channel_active_power_register = device
            .read_register::<SecondChannelActivePowerRegister>()
            .expect("Failed to read Second Channel Active Power register");
        println!(
            "Second Channel Active Power: {} W",
            second_channel_active_power_register.get_scaled_value()
        );
        // If there is no load connected, the active power should be 0W.
        assert_eq!(
            second_channel_active_power_register.get_scaled_value(),
            0.0,
            "Second Channel Active Power should be 0 W when no load is connected"
        );
    }

    #[test]
    fn test_get_second_channel_power_factor_no_load() {
        let mut device = setup_device();
        let second_channel_power_factor_register = device
            .read_register::<SecondChannelPowerFactorRegister>()
            .expect("Failed to read Second Channel Power Factor register");
        println!(
            "Second Channel Power Factor: {}",
            second_channel_power_factor_register.get_scaled_value()
        );
        // If there is no load connected, the power factor should be 0.
        assert_eq!(
            second_channel_power_factor_register.get_scaled_value(),
            0.0,
            "Second Channel Power Factor should be 0 when no load is connected"
        );
    }
}
