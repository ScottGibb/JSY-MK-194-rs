use std::time::Duration;

use jsy_mk_194_rs::jsy_mk_194g::JsyMk194g;
use jsy_mk_194_rs::registers::system_configuration_paramater::Baudrate;
use jsy_mk_194_rs::units::*;
fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    println!("Available ports:");
    for p in ports {
        println!("  {}", p.port_name);
    }

    // Open the first available port
    let port_name = "/dev/tty.usbserial-0001";
    let port = serialport::new(port_name, u32::from(Baudrate::default()))
        .timeout(Duration::from_secs(1))
        .open()
        .expect("Failed to open port");

    let delay = utils::StdDelay;

    let mut driver =
        JsyMk194g::new_default(port, delay).expect(" The Device shoudl be on the port");

    let system_parameters = driver
        .get_system_parameters()
        .expect("Failed to read system parameters");
    println!("System Paramaters: {system_parameters}");

    let baudrate = driver.get_baudrate().expect("Failed to read baudrate");
    println!("Baudrate: {baudrate:?}");

    let frequency = driver.get_frequency().expect("Failed to read frequency");
    println!("Frequency: {} Hz", frequency.get::<hertz>());
}
