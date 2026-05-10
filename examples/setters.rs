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

    let channel_stats = driver
        .get_channel(Channel::One)
        .expect("Failed to read channel one");
    println!("Channel One :\n  {channel_stats}");
    println!("------------------------");

    driver
        .set_positive_active_energy(Channel::One, Energy::new::<kilowatt_hour>(100.0))
        .expect("Failed to set positive active energy");

    driver
        .set_negative_active_energy(Channel::One, Energy::new::<kilowatt_hour>(150.0))
        .expect("Failed to set negative active energy");

    let new_stats = driver
        .get_channel(Channel::One)
        .expect("Failed to read channel one");
    println!("Channel One after setting energy : \n  {new_stats}");
}
