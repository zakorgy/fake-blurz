use core::ops::Deref;
use fake_characteristic::FakeBluetoothGATTCharacteristic;
use std::error::Error;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct FakeBluetoothGATTDescriptor {
    object_path: Arc<Mutex<String>>,
    uuid: Arc<Mutex<String>>,
    characteristic: Arc<FakeBluetoothGATTCharacteristic>,
    value: Arc<Mutex<Vec<u8>>>,
    flags: Arc<Mutex<Vec<String>>>,
}

impl FakeBluetoothGATTDescriptor {
    /*pub fn new(object_path: String,
               uuid: String,
               characteristic: FakeBluetoothGATTCharacteristic,
               value: Vec<u8>,
               flags: Vec<String>)
               -> FakeBluetoothGATTDescriptor {
        FakeBluetoothGATTDescriptor {
            object_path: Arc::new(Mutex::new(object_path)),
            uuid: Arc::new(Mutex::new(uuid)),
            characteristic: Arc::new(Mutex::new(characteristic)),
            value: Arc::new(Mutex::new(value)),
            flags: Arc::new(Mutex::new(flags)),
        }
    }*/

    pub fn new(object_path: String,
               characteristic: Arc<FakeBluetoothGATTCharacteristic>)
               -> Arc<FakeBluetoothGATTDescriptor> {
        let descriptor = Arc::new(FakeBluetoothGATTDescriptor {
            object_path: Arc::new(Mutex::new(object_path)),
            uuid: Arc::new(Mutex::new(String::new())),
            characteristic: characteristic.clone(),
            value: Arc::new(Mutex::new(vec![])),
            flags: Arc::new(Mutex::new(vec![])),
        });
        let _ = characteristic.add_descriptor(descriptor.clone());
        descriptor
    }

    pub fn new_empty() -> FakeBluetoothGATTDescriptor {
        FakeBluetoothGATTDescriptor {
            object_path: Arc::new(Mutex::new(String::new())),
            uuid: Arc::new(Mutex::new(String::new())),
            characteristic: Arc::new(FakeBluetoothGATTCharacteristic::new_empty()),
            value: Arc::new(Mutex::new(vec![])),
            flags: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn get_id(&self) -> String {
        let cloned = self.object_path.clone();
        let id = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => String::new(),
        };
        id
    }

    pub fn set_id(&self, value: String) {
        let cloned = self.object_path.clone();
        //TODO remove unwrap, if possible
        let mut id = cloned.lock().unwrap();
        *id = value;
    }

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        let cloned = self.uuid.clone();
        let uuid = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(uuid)
    }

    pub fn set_uuid(&self, value: String) -> Result<(), Box<Error>> {
        let cloned = self.uuid.clone();
        //TODO remove unwrap, if possible
        let mut uuid = cloned.lock().unwrap();
        Ok(*uuid = value)
    }

    pub fn get_characteristic(&self) -> Result<Arc<FakeBluetoothGATTCharacteristic>, Box<Error>> {
        Ok(self.characteristic.clone())
    }

    /*pub fn set_characteristic(&self, characteristic: Arc<FakeBluetoothGATTCharacteristic>) -> Result<(), Box<Error>> {
        Ok(self.characteristic = characteristic)
    }*/

    pub fn get_flags(&self) -> Result<Vec<String>, Box<Error>> {
        let cloned = self.flags.clone();
        let flags = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(flags)
    }

    pub fn set_flags(&self, value: Vec<String>) -> Result<(), Box<Error>> {
        let cloned = self.flags.clone();
        //TODO remove unwrap, if possible
        let mut flags = cloned.lock().unwrap();
        Ok(*flags = value)
    }

    pub fn get_value(&self) -> Result<Vec<u8>, Box<Error>> {
        let cloned = self.value.clone();
        let value = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(value)
    }

    pub fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        self.get_value()
    }

    pub fn write_value(&self, new_value: Vec<u8>) -> Result<(), Box<Error>> {
        let cloned = self.value.clone();
        //TODO remove unwrap, if possible
        let mut value = cloned.lock().unwrap();
        Ok(*value = new_value)
    }
}
