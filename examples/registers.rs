#![cfg_attr(not(all(feature = "std-sync", feature = "advanced")), allow(dead_code))]

#[cfg(not(all(feature = "std-sync", feature = "advanced")))]
fn main() {
    println!("This example needs advanced and std-sync enabled");
}

#[cfg(all(feature = "std-sync", feature = "advanced"))]
fn main() {
    use std::println;
    use std::time::Duration;

    use jsy_mk_194_rs::jsy_mk_194g::JsyMk194g;
    use jsy_mk_194_rs::registers::system_configuration_parameter::{
        Baudrate, SystemConfigurationParameterRegister,
    };

    let ports = serialport::available_ports().expect("No ports found!");
    println!("Available ports:");
    for p in ports {
        println!("  {}", p.port_name);
    }
    println!("------------------------");
    // Open the first available port
    let port_name = "/dev/tty.usbserial-0001";
    let port = serialport::new(port_name, u32::from(Baudrate::default()))
        .timeout(Duration::from_secs(1))
        .open()
        .expect("Failed to open port");

    let delay = utils::StdDelay;

    let mut driver =
        JsyMk194g::new_default(port, delay).expect(" The Device should be on this port");

    let system_configuration_parameters_register = driver
        .read_register::<SystemConfigurationParameterRegister>()
        .expect("Failed to read System Configuration Parameter register");

    println!(
        "System Configuration Parameter Register: {system_configuration_parameters_register:?}"
    );
}
