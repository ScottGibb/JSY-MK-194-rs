#![cfg_attr(not(feature = "std-sync"), allow(dead_code))]

#[cfg(not(feature = "std-sync"))]
fn main() {
    println!("This example needs std-sync enabled");
}

#[cfg(feature = "std-sync")]
fn main() {
    use std::println;
    use std::time::Duration;

    use jsy_mk_194_rs::jsy_mk_194g::JsyMk194g;
    use jsy_mk_194_rs::types::Baudrate;
    use jsy_mk_194_rs::types::Channel;
    use jsy_mk_194_rs::units::*;

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

    let system_parameters = driver
        .get_system_parameters()
        .expect("Failed to read system parameters");
    println!("System parameters: {system_parameters}");

    let baudrate = driver.get_baudrate().expect("Failed to read baudrate");
    println!("Baudrate: {baudrate:?}");

    let frequency = driver.get_frequency().expect("Failed to read frequency");
    println!("Frequency: {} Hz", frequency.get::<hertz>());

    println!("------------------------");
    println!("Reading all channels...");
    let stats = driver
        .get_all_channels()
        .expect("Failed to read all channels");
    println!("Statistics: {stats}");

    println!("------------------------");
    println!("Reading channels individually...");
    let channel_one = driver
        .get_channel(Channel::One)
        .expect("Failed to read channel one");
    println!("Channel One: {channel_one}");

    let channel_two = driver
        .get_channel(Channel::Two)
        .expect("Failed to read channel two");
    println!("Channel Two: {channel_two}");

    println!("------------------------");
    let frequency = driver.get_frequency().expect("Failed to read frequency");
    println!("Frequency: {} Hz", frequency.get::<hertz>());

    println!("------------------------");
    let power_direction = driver
        .get_power_direction(Channel::One)
        .expect("Failed to read power direction");
    println!(" Channel One Power Direction: {power_direction:?}");

    let power_direction = driver
        .get_power_direction(Channel::Two)
        .expect("Failed to read power direction");
    println!(" Channel Two Power Direction: {power_direction:?}");
}
