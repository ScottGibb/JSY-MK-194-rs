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

    let port_name = "/dev/tty.usbserial-0001";
    let port = serialport::new(port_name, u32::from(Baudrate::default()))
        .timeout(Duration::from_secs(1))
        .open()
        .expect("Failed to open port");

    let delay = StdDelay;
    let mut driver =
        JsyMk194g::new_default(port, delay).expect("The device should be on this port");

    println!("Default response delay: {:?}", driver.get_response_delay());

    driver.set_response_delay(Duration::from_millis(600), Duration::from_millis(700));
    println!("Updated response delay: {:?}", driver.get_response_delay());

    let id = driver.get_id().expect("Failed to read ID");
    println!("Device ID: {id:?}");
}
