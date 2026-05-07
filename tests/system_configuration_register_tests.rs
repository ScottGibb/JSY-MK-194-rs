#![cfg(all(feature = "std-sync", feature = "advanced"))]

use jsy_mk_194_rs::registers::system_configuration_paramater::{
    Baudrate, Id, SystemConfigurationParameterRegister,
};
mod common;
use common::setup_device;

#[test]
fn test_get_system_configuration_register() {
    let mut device = setup_device(Id::default(), Baudrate::default());
    let system_configuration = device
        .read_register::<SystemConfigurationParameterRegister>()
        .expect("Failed to read System Configuration register");
    println!("System Configuration: {system_configuration:?}");
    assert_eq!(
        system_configuration,
        SystemConfigurationParameterRegister::default()
    );
}

#[test]
fn test_set_system_configuration_register() {
    let new_id = Id::new(0x02).expect("Failed to create new ID");
    let new_baudrate = Baudrate::_19200;
    {
        let mut device = setup_device(Id::default(), Baudrate::default());

        let original_system_configuration = device
            .read_register::<SystemConfigurationParameterRegister>()
            .expect("Failed to read System Configuration register");
        println!("Original System Configuration: {original_system_configuration:?}");

        let new_system_configuration = SystemConfigurationParameterRegister {
            id: new_id.clone(),
            baudrate: new_baudrate.clone(),
        };
        device
            .write_register(new_system_configuration)
            .expect("Failed to write new System Configuration register");
    }
    {
        let mut device = setup_device(new_id.clone(), new_baudrate.clone());
        let system_configuration = device
            .read_register::<SystemConfigurationParameterRegister>()
            .expect("Failed to read System Configuration register after update");

        println!("Updated System Configuration: {system_configuration:?}");
        assert_eq!(system_configuration.baudrate, new_baudrate);
        assert_eq!(system_configuration.id, new_id);

        // Reset the register back to the default values so it doesn't affect other tests
        device
            .write_register(SystemConfigurationParameterRegister::default())
            .expect("Failed to reset System Configuration register");
    }
    let mut device = setup_device(Id::default(), Baudrate::default());
    let system_configuration = device
        .read_register::<SystemConfigurationParameterRegister>()
        .expect("Failed to read System Configuration register after reset");

    println!("Reset System Configuration: {system_configuration:?}");
    assert_eq!(
        system_configuration,
        SystemConfigurationParameterRegister::default()
    );
}
