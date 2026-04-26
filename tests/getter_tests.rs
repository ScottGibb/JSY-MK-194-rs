mod common;
use common::setup_device;
use jsy_mk_194_rs::types::{Baudrate, Id};
mod fresh_device_tests {
    use super::*;

    #[test]
    fn test_get_id() {
        let mut device = setup_device(Id::default(), Baudrate::default());
        let id = device.get_id().expect("This should not fail");
        println!("device Id {:?}", id);
        assert_eq!(id, Id::default())
    }

    #[test]
    fn test_baudrate() {
        let mut device = setup_device(Id::default(), Baudrate::default());
        let baudrate = device.get_baudrate().expect("This should not fail");
        println!("Baudrate: {:?}", baudrate);
        assert_eq!(baudrate, Baudrate::default());
    }
}
