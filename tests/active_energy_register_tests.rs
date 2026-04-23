use std::time::Duration;

use jsy_mk_194_rs::{
    REQUEST_RESPONSE_DELAY,
    jsy_mk_194g::JsyMk194g,
    registers::{
        channel_one_measuring_electrical_paramaters::{
            FirstChannelNegativeActiveEnergyRegister, FirstChannelPositiveActiveEnergyRegister,
        },
        channel_two_measuring_electrical_paramaters::{
            SecondChannelNegativeActiveEnergyRegister, SecondChannelPositiveActiveEnergyRegister,
        },
        system_configuration_paramater::Baudrate,
    },
    units::{Energy, watt_hour},
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

#[ignore = "  This test is only used to reset the device in order to do other tests"]
#[test]
fn reset_active_energy_registers() {
    let mut device = setup_device();
    let zero_energy = Energy::new::<watt_hour>(0.0);
    let zero_first_channel_positive_active_energy_register =
        FirstChannelPositiveActiveEnergyRegister::from_scaled_value(zero_energy.get::<watt_hour>());
    let zero_first_channel_negative_active_energy_register =
        FirstChannelNegativeActiveEnergyRegister::from_scaled_value(zero_energy.get::<watt_hour>());

    let zero_second_channel_positive_active_energy_register =
        SecondChannelPositiveActiveEnergyRegister::from_scaled_value(
            zero_energy.get::<watt_hour>(),
        );
    let zero_second_channel_negative_active_energy_register =
        SecondChannelNegativeActiveEnergyRegister::from_scaled_value(
            zero_energy.get::<watt_hour>(),
        );

    // Write zero to all the active energy registers to reset them to a known state
    device
        .write_register(zero_first_channel_positive_active_energy_register)
        .expect("Failed to reset Channel One Positive Active Energy register");
    std::thread::sleep(REQUEST_RESPONSE_DELAY * 2); // Ensure we don't write too quickly after
    device
        .write_register(zero_first_channel_negative_active_energy_register)
        .expect("Failed to reset Channel One Negative Active Energy register");
    std::thread::sleep(REQUEST_RESPONSE_DELAY * 2); // Ensure we don't write too quickly after
    device
        .write_register(zero_second_channel_positive_active_energy_register)
        .expect("Failed to reset Channel Two Positive Active Energy register");
    std::thread::sleep(REQUEST_RESPONSE_DELAY * 2); // Ensure we don't write too quickly after
    device
        .write_register(zero_second_channel_negative_active_energy_register)
        .expect("Failed to reset Channel Two Negative Active Energy register");
    std::thread::sleep(REQUEST_RESPONSE_DELAY * 2); // Ensure we don't write too quickly after

    println!("Active energy registers reset to zero");
}

mod fresh_device_tests {
    use super::*;
    #[test]
    fn test_get_channel_one_positive_active_energy_register_fresh_device() {
        let mut device = setup_device();
        // device.write_register(FirstChannelPositiveActiveEnergyRegister::from_scaled_value(0.0)).expect(
        //     "Failed to reset Channel One Positive Active Energy register to default value before test",
        // );
        let first_channel_positive_active_energy_register = device
            .read_register::<FirstChannelPositiveActiveEnergyRegister>()
            .expect("Failed to read Channel One Positive Active Energy register");
        println!(
            "Channel One Positive Active Energy: {:?}",
            first_channel_positive_active_energy_register
        );

        // This register can change and has no default value assume, fresh device here
        assert_eq!(
            first_channel_positive_active_energy_register.get_scaled_value(),
            0.0
        );
    }

    #[test]
    fn test_get_channel_one_negative_active_energy_register_fresh_device() {
        let mut device = setup_device();
        let first_channel_negative_active_energy_register = device
            .read_register::<FirstChannelNegativeActiveEnergyRegister>()
            .expect("Failed to read Channel One Negative Active Energy register");
        println!(
            "Channel One Negative Active Energy: {:?}",
            first_channel_negative_active_energy_register
        );

        // This register can change and has no default value assume, fresh device here
        assert_eq!(
            first_channel_negative_active_energy_register.get_scaled_value(),
            0.0
        );
    }
    #[test]
    fn test_get_channel_two_positive_active_energy_register_fresh_device() {
        let mut device = setup_device();
        let second_channel_positive_active_energy_register = device
            .read_register::<SecondChannelPositiveActiveEnergyRegister>()
            .expect("Failed to read Channel Two Positive Active Energy register");
        println!(
            "Channel Two Positive Active Energy: {:?}",
            second_channel_positive_active_energy_register
        );

        // This register can change and has no default value assume, fresh device here
        assert_eq!(
            second_channel_positive_active_energy_register.get_scaled_value(),
            0.0
        );
    }

    /// TODO: This always fails
    #[test]
    fn test_get_channel_two_negative_active_energy_register_fresh_device() {
        let mut device = setup_device();
        let second_channel_negative_active_energy_register = device
            .read_register::<SecondChannelNegativeActiveEnergyRegister>()
            .expect("Failed to read Channel Two Negative Active Energy register");
        println!(
            "Channel Two Negative Active Energy: {:?}",
            second_channel_negative_active_energy_register
        );

        // This register can change and has no default value assume, fresh device here
        assert_eq!(
            second_channel_negative_active_energy_register.get_scaled_value(),
            0.0
        );
    }
}

mod set_register_tests {
    use super::*;
    #[test]
    fn test_set_channel_one_positive_active_energy_register() {
        let mut device = setup_device();
        let old_energy_register = device
            .read_register::<FirstChannelPositiveActiveEnergyRegister>()
            .expect("Failed to read Channel One Positive Active Energy register");
        println!(
            "Old Channel One Positive Active Energy Register: {:?}",
            old_energy_register
        );

        let new_energy = Energy::new::<watt_hour>(123.45);

        let new_energy_register = FirstChannelPositiveActiveEnergyRegister::from_scaled_value(
            new_energy.get::<watt_hour>(),
        );
        std::thread::sleep(REQUEST_RESPONSE_DELAY); // Ensure we don't write too quickly after reading
        device
            .write_register(new_energy_register)
            .expect("Failed to write new Channel One Positive Active Energy register");

        let updated_energy_register = device
            .read_register::<FirstChannelPositiveActiveEnergyRegister>()
            .expect("Failed to read updated Channel One Positive Active Energy register");
        println!(
            "Updated Channel One Positive Active Energy Register: {:?}",
            updated_energy_register
        );

        assert_eq!(
            updated_energy_register.get_scaled_value(),
            new_energy_register.get_scaled_value()
        );

        // Reset the register back to the original value so it doesn't affect other tests
        std::thread::sleep(REQUEST_RESPONSE_DELAY); // Ensure we don't write too quickly after reading
        device.write_register(old_energy_register).expect(
            "Failed to reset Channel One Positive Active Energy
    register",
        );
        std::thread::sleep(REQUEST_RESPONSE_DELAY); // Ensure we don't read too quickly after writing
        let reset_energy_register = device
            .read_register::<FirstChannelPositiveActiveEnergyRegister>()
            .expect("Failed to read reset Channel One Positive Active Energy register");
        println!(
            "Reset Channel One Positive Active Energy Register: {:?}",
            reset_energy_register
        );

        assert_eq!(
            reset_energy_register.get_scaled_value(),
            old_energy_register.get_scaled_value()
        );
    }

    #[test]
    fn test_set_channel_two_positive_active_energy_register() {
        let mut device = setup_device();
        let old_energy_register = device
            .read_register::<SecondChannelPositiveActiveEnergyRegister>()
            .expect("Failed to read Channel Two Positive Active Energy register");
        println!(
            "Old Channel Two Positive Active Energy Register: {:?}",
            old_energy_register
        );

        let new_energy = Energy::new::<watt_hour>(123.45);

        let new_energy_register = SecondChannelPositiveActiveEnergyRegister::from_scaled_value(
            new_energy.get::<watt_hour>(),
        );
        std::thread::sleep(REQUEST_RESPONSE_DELAY); // Ensure we don't write too quickly after reading
        device
            .write_register(new_energy_register)
            .expect("Failed to write new Channel Two Positive Active Energy register");

        let updated_energy_register = device
            .read_register::<SecondChannelPositiveActiveEnergyRegister>()
            .expect("Failed to read updated Channel Two Positive Active Energy register");
        println!(
            "Updated Channel Two Positive Active Energy Register: {:?}",
            updated_energy_register
        );

        assert_eq!(
            updated_energy_register.get_scaled_value(),
            new_energy_register.get_scaled_value()
        );

        // Reset the register back to the original value so it doesn't affect other tests
        std::thread::sleep(REQUEST_RESPONSE_DELAY); // Ensure we don't write too quickly after reading
        device.write_register(old_energy_register).expect(
            "Failed to reset Channel Two Positive Active Energy
    register",
        );
        std::thread::sleep(REQUEST_RESPONSE_DELAY); // Ensure we don't read too quickly after writing
        let reset_energy_register = device
            .read_register::<SecondChannelPositiveActiveEnergyRegister>()
            .expect("Failed to read reset Channel Two Positive Active Energy register");
        println!(
            "Reset Channel Two Positive Active Energy Register: {:?}",
            reset_energy_register
        );

        assert_eq!(
            reset_energy_register.get_scaled_value(),
            old_energy_register.get_scaled_value()
        );
    }

    #[test]
    fn test_set_channel_one_negative_active_energy_register() {
        let mut device = setup_device();
        let old_energy_register = device
            .read_register::<FirstChannelNegativeActiveEnergyRegister>()
            .expect("Failed to read Channel One Negative Active Energy register");
        println!(
            "Old Channel One Negative Active Energy Register: {:?}",
            old_energy_register
        );

        let new_energy = Energy::new::<watt_hour>(123.45);

        let new_energy_register = FirstChannelNegativeActiveEnergyRegister::from_scaled_value(
            new_energy.get::<watt_hour>(),
        );
        std::thread::sleep(REQUEST_RESPONSE_DELAY); // Ensure we don't write too quickly after reading
        device
            .write_register(new_energy_register)
            .expect("Failed to write new Channel One Negative Active Energy register");

        let updated_energy_register = device
            .read_register::<FirstChannelNegativeActiveEnergyRegister>()
            .expect("Failed to read updated Channel One Negative Active Energy register");
        println!(
            "Updated Channel One Negative Active Energy Register: {:?}",
            updated_energy_register
        );

        assert_eq!(
            updated_energy_register.get_scaled_value(),
            new_energy_register.get_scaled_value()
        );

        // Reset the register back to the original value so it doesn't affect other tests
        std::thread::sleep(REQUEST_RESPONSE_DELAY); // Ensure we don't write too quickly after reading
        device.write_register(old_energy_register).expect(
            "Failed to reset Channel One Negative Active Energy
    register",
        );
        std::thread::sleep(REQUEST_RESPONSE_DELAY); // Ensure we don't read too quickly after writing
        let reset_energy_register = device
            .read_register::<FirstChannelNegativeActiveEnergyRegister>()
            .expect("Failed to read reset Channel One Negative Active Energy register");
        println!(
            "Reset Channel One Negative Active Energy Register: {:?}",
            reset_energy_register
        );

        assert_eq!(
            reset_energy_register.get_scaled_value(),
            old_energy_register.get_scaled_value()
        );
    }

    /// TODO: This always fails
    #[test]
    fn test_set_channel_two_negative_active_energy_register() {
        let mut device = setup_device();
        let old_energy_register = device
            .read_register::<SecondChannelNegativeActiveEnergyRegister>()
            .expect("Failed to read Channel Two Negative Active Energy register");
        println!(
            "Old Channel Two Negative Active Energy Register: {:?}",
            old_energy_register
        );

        let new_energy = Energy::new::<watt_hour>(123.45);

        let new_energy_register = SecondChannelNegativeActiveEnergyRegister::from_scaled_value(
            new_energy.get::<watt_hour>(),
        );
        std::thread::sleep(REQUEST_RESPONSE_DELAY); // Ensure we don't write too quickly after reading
        device
            .write_register(new_energy_register)
            .expect("Failed to write new Channel Two Negative Active Energy register");

        let updated_energy_register = device
            .read_register::<SecondChannelNegativeActiveEnergyRegister>()
            .expect("Failed to read updated Channel Two Negative Active Energy register");
        println!(
            "Updated Channel Two Negative Active Energy Register: {:?}",
            updated_energy_register
        );

        assert_eq!(
            updated_energy_register.get_scaled_value(),
            new_energy_register.get_scaled_value()
        );

        // Reset the register back to the original value so it doesn't affect other tests
        std::thread::sleep(REQUEST_RESPONSE_DELAY); // Ensure we don't write too quickly after reading
        device.write_register(old_energy_register).expect(
            "Failed to reset Channel Two Negative Active Energy
    register",
        );
        std::thread::sleep(REQUEST_RESPONSE_DELAY); // Ensure we don't read too quickly after writing
        let reset_energy_register = device
            .read_register::<SecondChannelNegativeActiveEnergyRegister>()
            .expect("Failed to read reset Channel Two Negative Active Energy register");
        println!(
            "Reset Channel Two Negative Active Energy Register: {:?}",
            reset_energy_register
        );

        assert_eq!(
            reset_energy_register.get_scaled_value(),
            old_energy_register.get_scaled_value()
        );
    }
}
