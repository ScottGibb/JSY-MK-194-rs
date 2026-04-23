use std::time::Duration;

use jsy_mk_194_rs::{
    jsy_mk_194g::JsyMk194g,
    registers::{
        system_configuration_paramater::Baudrate,
        system_paramaters::{CurrentRangeRegister, ModelOneRegister, VoltageRangeRegister},
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

#[test]
fn test_model_one_register() {
    let mut device = setup_device();
    let model_one = device
        .read_register::<ModelOneRegister>()
        .expect("Failed to read Model One register");
    println!("Model One: {}", model_one.0);
    assert_eq!(model_one.0, ModelOneRegister::default().0); //TODO: Tidy up this .0 maybe make it a getter or something
}

#[test]
fn test_voltage_range_register() {
    let mut device = setup_device();
    let voltage_range = device
        .read_register::<VoltageRangeRegister>()
        .expect("Failed to read Voltage Range register");
    println!("Voltage Range: {}", voltage_range.get_scaled_value());
    assert_eq!(
        voltage_range.get_scaled_value(),
        VoltageRangeRegister::default().get_scaled_value()
    );
}

#[test]
fn test_current_range_register() {
    let mut device = setup_device();
    let current_range = device
        .read_register::<CurrentRangeRegister>()
        .expect("Failed to read Current Range register");
    println!("Current Range: {}", current_range.get_scaled_value());
    assert_eq!(
        current_range.get_scaled_value(),
        CurrentRangeRegister::default().get_scaled_value()
    );
}
