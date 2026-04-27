use jsy_mk_194_rs::{
    registers::misc_registers::{FrequencyRegister, PowerDirection, PowerDirectionRegister},
    types::{Baudrate, Id},
};
mod common;
use common::setup_device;
#[test]
fn test_power_direction_register() {
    let mut device = setup_device(Id::default(), Baudrate::default());
    let power_direction = device
        .read_register::<PowerDirectionRegister>()
        .expect("Failed to read Power Direction register");
    println!("Power Direction: {power_direction:?}");
    // There is no default value for this register, so we can't assert against it.
    // Just check that it can be read without error.

    // When running this, it shows positive, so an unoffical positive can be used
    assert_eq!(
        power_direction,
        PowerDirectionRegister {
            first_channel: PowerDirection::Positive,
            second_channel: PowerDirection::Positive,
        }
    );
}

#[test]
fn test_frequency_register_mains_uk() {
    let mut device = setup_device(Id::default(), Baudrate::default());
    let frequency_register = device
        .read_register::<FrequencyRegister>()
        .expect("Failed to read Frequency register");
    let frequency = frequency_register.get_scaled_value();
    println!("Frequency: {frequency} Hz");

    //The frequency should be around 50 or 60 Hz, depending on the region.
    //We can allow for some variation, but it shouldn't be wildly off.
    const MIN_FREQUENCY: f32 = 45.0;
    const MAX_FREQUENCY: f32 = 65.0;
    assert!(
        (MIN_FREQUENCY..=MAX_FREQUENCY).contains(&frequency),
        "Frequency {frequency} Hz is out of expected range ({MIN_FREQUENCY} - {MAX_FREQUENCY} Hz)"
    );
}

#[test]
fn test_frequency_register_no_mains() {
    let mut device = setup_device(Id::default(), Baudrate::default());
    let frequency_register = device
        .read_register::<FrequencyRegister>()
        .expect("Failed to read Frequency register");
    let frequency = frequency_register.get_scaled_value();
    println!("Frequency: {frequency} Hz");
    // If there is no mains power, the frequency should be 0 Hz.
    assert_eq!(
        frequency, 0.0,
        "Frequency should be 0 Hz when there is no mains power"
    );
}
