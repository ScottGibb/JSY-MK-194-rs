use std::{
    process::{ExitCode, exit},
    time::Duration,
};

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

#[test]
#[ignore = "This test is meant to be run manually to find the correct baudrate and id for the device. It will panic if it fails to find the device, so it should not be run as part of the normal test suite."]
pub fn find_device_test() {
    let baudrates = [
        Baudrate::_1200,
        Baudrate::_2400,
        Baudrate::_4800,
        Baudrate::_9600,
        Baudrate::_19200,
        Baudrate::_38400,
    ];

    for baud in baudrates {
        println!("________________________");
        println!("Trying baudrate {baud:?}");
        for id in 1..=255 {
            // Skipping 0 due to it not being valid
            println!("Trying baudrate {baud:?} and id {id}");
            let new_baud = baud.clone();
            let port = serialport::new(TEST_PORT, u32::from(baud.clone()))
                .timeout(Duration::from_secs(1))
                .open();
            match port {
                Ok(port) => {
                    let mut device: JsyMk194g<Box<dyn SerialPort>, utils::StdDelay> =
                        JsyMk194g::new(
                            port,
                            Id::new(id).expect("This should not fail"),
                            utils::StdDelay,
                        );
                    let device_id = Id::new(id).expect("Should not fail");
                    match device.get_id() {
                        Ok(id) => {
                            println!("Found device with baudrate {new_baud:?} and id {id:?}");
                            assert!(
                                true,
                                "Found device with baudrate {new_baud:?} and id {id:?}"
                            );
                            //Reset the device
                            println!("Resetting device id and baudrate to default");
                            device
                                .set_id(Id::default())
                                .expect("Failed to reset device id to default");
                            device
                                .set_baudrate(Baudrate::default())
                                .expect("Failed to reset device baudrate to default");
                            assert_eq!(id, device_id);
                            return;
                        }
                        Err(err) => {
                            println!("Failed to get id with baudrate {new_baud:?}: {err:?}")
                        }
                    }
                }
                Err(err) => println!("Failed to open port with baudrate {new_baud:?}: {err:?}"),
            }
        }
    }
    std::thread::sleep(Duration::from_millis(10));
    panic!("Failed to find device");
}
