#![cfg_attr(not(feature = "tokio-async"), allow(dead_code))]

#[cfg(not(feature = "tokio-async"))]
fn main() {
    println!("This example needs async enabled");
}

use jsy_mk_194_rs::{jsy_mk_194g::JsyMk194g, types::Baudrate};

use tokio_serial::SerialPortBuilderExt;
#[cfg(feature = "tokio-async")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use utils::StdDelay;

    let port = tokio_serial::new("/dev/tty.usbserial-0001", Baudrate::default().into())
        .open_native_async()?;

    let mut device = JsyMk194g::new_default(port, StdDelay)
        .await
        .expect("this should not fail");

    // Example: Read the device ID
    let id = device.get_id().await.unwrap();
    println!("Device ID: {id:?}");

    // Example: Read the baud rate
    let baudrate = device.get_baudrate().await.unwrap();
    println!("Baudrate: {baudrate:? }");
    loop {
        let stats = device.read_statistics().await.unwrap();
        println!("Statistics: {stats:?}");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
