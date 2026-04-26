mod common;
use common::setup_device;
use jsy_mk_194_rs::types::{Baudrate, Id};
mod configuration_tests {

    use super::*;

    #[test]
    fn test_set_id() {
        let new_id = Id::new(0x02).expect("Failed to create new ID");
        {
            let mut device = setup_device(Id::default(), Baudrate::default());

            let original_id = device
            .read_register::<jsy_mk_194_rs::registers::system_configuration_paramater::SystemConfigurationParamaterRegister>()
            .expect("Failed to read System Configuration register")
            .id;
            println!("Original ID: {:?}", original_id);

            device.set_id(new_id.clone()).expect("Failed to set new ID");
            std::thread::sleep(jsy_mk_194_rs::REQUEST_RESPONSE_DELAY); // Give the device some time to process the change
            let updated_id = device.get_id().expect("Failed to read ID after update");
            println!("Updated ID: {:?}", updated_id.clone());
            assert_eq!(updated_id, new_id);

            // Reset the ID back to the default value so it doesn't affect other tests
            device.set_id(Id::default()).expect("Failed to reset ID");
            let reset_id = device.get_id().expect("Failed to read ID after reset");
            println!("Reset ID: {:?}", reset_id);
            assert_eq!(reset_id, Id::default());
        }
    }
    #[test]
    fn set_baudrate() {
        let new_baudrate = Baudrate::_38400;
        {
            let mut device = setup_device(Id::default(), Baudrate::default());

            let original_baudrate = device
                .read_register::<jsy_mk_194_rs::registers::system_configuration_paramater::SystemConfigurationParamaterRegister>()
                .expect("Failed to read System Configuration register")
                .baudrate;
            println!("Original Baudrate: {:?}", original_baudrate);

            device
                .set_baudrate(new_baudrate.clone())
                .expect("Failed to set new Baudrate");
            std::thread::sleep(jsy_mk_194_rs::REQUEST_RESPONSE_DELAY); // Give the device some time to process the change
        }

        {
            let mut device = setup_device(Id::default(), new_baudrate.clone());
            let updated_baudrate = device
                .get_baudrate()
                .expect("Failed to read Baudrate after update");
            println!("Updated Baudrate: {:?}", updated_baudrate.clone());
            assert_eq!(updated_baudrate, new_baudrate);

            // Reset the baudrate back to the default value so it doesn't affect other tests
            device
                .set_baudrate(Baudrate::default())
                .expect("Failed to reset Baudrate");
        }
        {
            let mut device = setup_device(Id::default(), Baudrate::default());
            let reset_baudrate = device
                .get_baudrate()
                .expect("Failed to read Baudrate after reset");
            println!("Reset Baudrate: {:?}", reset_baudrate);
            assert_eq!(reset_baudrate, Baudrate::default());
        }
    }
}
mod energy_tests {
    use super::*;
}
