use std::time::Duration;

use jsy_mk_194_rs::{
    jsy_mk_194g::JsyMk194g,
    types::{Baudrate, Id},
};
use serialport::SerialPort;

const TEST_PORT: &str = "/dev/tty.usbserial-0001";
pub fn setup_device(
    device_id: Id,
    baud: Baudrate,
) -> JsyMk194g<Box<dyn SerialPort>, utils::StdDelay> {
    let port = serialport::new(TEST_PORT, u32::from(baud))
        .timeout(Duration::from_secs(1))
        .open()
        .expect("Failed to open port");
    let delay = utils::StdDelay;
    JsyMk194g::new(port, device_id, delay)
}
