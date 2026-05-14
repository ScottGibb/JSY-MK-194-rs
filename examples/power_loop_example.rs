#![cfg_attr(not(feature = "std-sync"), allow(dead_code))]

#[cfg(not(feature = "std-sync"))]
fn main() {
    println!("This example needs std-sync enabled");
}

#[cfg(feature = "std-sync")]
fn main() {
    use std::println;
    use std::time::Duration;

    use jsy_mk_194_rs::delay::StdDelay;
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

    let delay = StdDelay;

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
    println!("Reading Voltages...");

    loop {
        let channel_one = driver
            .get_channel(Channel::One)
            .expect("Failed to read channel one");
        println!(
            "Channel One Power: {} W",
            channel_one.active_power.get::<watt>()
        );
        println!("Power direction: {}", channel_one.power_direction);

        let channel_two = driver
            .get_channel(Channel::Two)
            .expect("Failed to read channel two");
        println!(
            "Channel Two Power: {} W",
            channel_two.active_power.get::<watt>()
        );
        println!("Power direction: {}", channel_two.power_direction);
        println!("------------------------");
        std::thread::sleep(Duration::from_secs(1));
    }
}
