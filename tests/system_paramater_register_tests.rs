#![cfg(feature = "std")]

use jsy_mk_194_rs::{
    registers::system_paramaters::{CurrentRangeRegister, ModelOneRegister, VoltageRangeRegister},
    types::{Baudrate, Id},
};

mod common;
use common::setup_device;
#[test]
fn test_model_one_register() {
    let mut device = setup_device(Id::default(), Baudrate::default());
    let model_one = device
        .read_register::<ModelOneRegister>()
        .expect("Failed to read Model One register");
    println!("Model One: {}", model_one.0);
    assert_eq!(model_one.0, ModelOneRegister::default().0); //TODO: Tidy up this .0 maybe make it a getter or something
}

#[test]
fn test_voltage_range_register() {
    let mut device = setup_device(Id::default(), Baudrate::default());
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
    let mut device = setup_device(Id::default(), Baudrate::default());
    let current_range = device
        .read_register::<CurrentRangeRegister>()
        .expect("Failed to read Current Range register");
    println!("Current Range: {}", current_range.get_scaled_value());
    assert_eq!(
        current_range.get_scaled_value(),
        CurrentRangeRegister::default().get_scaled_value()
    );
}
