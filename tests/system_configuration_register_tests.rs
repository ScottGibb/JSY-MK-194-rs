use core::time::Duration;

use jsy_mk_194_rs::{
    jsy_mk_194g::JsyMk194g,
    registers::system_configuration_paramater::{
        Baudrate, Id, SystemConfigurationParamaterRegister,
    },
};
use serialport::SerialPort;

const TEST_PORT: &str = "/dev/tty.usbserial-0001";
fn setup_device(device_id: Id, baud: Baudrate) -> JsyMk194g<Box<dyn SerialPort>, utils::StdDelay> {
    let port = serialport::new(TEST_PORT, u32::from(baud))
        .timeout(Duration::from_secs(1))
        .open()
        .expect("Failed to open port");
    let delay = utils::StdDelay;
    JsyMk194g::new(port, device_id, delay)
}

#[test]
fn test_get_system_configuration_register() {
    let mut device = setup_device(Id::default(), Baudrate::default());
    let system_configuration = device
        .read_register::<SystemConfigurationParamaterRegister>()
        .expect("Failed to read System Configuration register");
    println!("System Configuration: {:?}", system_configuration);
    assert_eq!(
        system_configuration,
        SystemConfigurationParamaterRegister::default()
    );
}

#[test]
fn test_set_system_configuration_register() {
    let new_id = Id::new(0x02).expect("Failed to create new ID");
    let new_baudrate = Baudrate::_19200;
    {
        let mut device = setup_device(Id::default(), Baudrate::default());

        let original_system_configuration = device
            .read_register::<SystemConfigurationParamaterRegister>()
            .expect("Failed to read System Configuration register");
        println!(
            "Original System Configuration: {:?}",
            original_system_configuration
        );

        let new_system_configuration = SystemConfigurationParamaterRegister {
            id: new_id.clone(),
            baudrate: new_baudrate.clone(),
        };
        device
            .write_register(new_system_configuration)
            .expect("Failed to write new System Configuration register");
    }
    let mut device = setup_device(new_id.clone(), new_baudrate.clone());
    let system_configuration = device
        .read_register::<SystemConfigurationParamaterRegister>()
        .expect("Failed to read System Configuration register after update");

    println!("Updated System Configuration: {:?}", system_configuration);
    assert_eq!(system_configuration.baudrate, new_baudrate);
    assert_eq!(system_configuration.id, new_id);
}
