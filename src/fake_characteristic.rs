use core::ops::Deref;
use fake_descriptor::FakeBluetoothGATTDescriptor;
use fake_service::FakeBluetoothGATTService;
use std::error::Error;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct FakeBluetoothGATTCharacteristic {
    id: Arc<Mutex<String>>,
    uuid: Arc<Mutex<String>>,
    service: Arc<FakeBluetoothGATTService>,
    value: Arc<Mutex<Vec<u8>>>,
    is_notifying: Arc<Mutex<bool>>,
    flags: Arc<Mutex<Vec<String>>>,
    gatt_descriptors: Arc<Mutex<Vec<Arc<FakeBluetoothGATTDescriptor>>>>,
}

impl FakeBluetoothGATTCharacteristic {
    /*pub fn new(id: String,
               uuid: String,
               service: FakeBluetoothGATTService,
               value: Vec<u8>,
               is_notifying: bool,
               flags: Vec<String>,
               gatt_descriptors: Vec<Arc<FakeBluetoothGATTDescriptor>>
               )
               -> FakeBluetoothGATTCharacteristic {
        FakeBluetoothGATTCharacteristic {
            id: Arc::new(Mutex::new(id)),
            uuid: Arc::new(Mutex::new(uuid)),
            service: Arc::new(Mutex::new(service)),
            value: Arc::new(Mutex::new(value)),
            is_notifying: Arc::new(Mutex::new(is_notifying)),
            flags: Arc::new(Mutex::new(flags)),
            gatt_descriptors: gatt_descriptors,
        }
    }*/

    pub fn new(id: String,
               service: Arc<FakeBluetoothGATTService>)
               -> Arc<FakeBluetoothGATTCharacteristic> {
        let characteristic = Arc::new(FakeBluetoothGATTCharacteristic {
            id: Arc::new(Mutex::new(id)),
            uuid: Arc::new(Mutex::new(String::new())),
            service: service.clone(),
            value: Arc::new(Mutex::new(vec![])),
            is_notifying: Arc::new(Mutex::new(false)),
            flags: Arc::new(Mutex::new(vec![])),
            gatt_descriptors: Arc::new(Mutex::new(vec![])),
        });
        let _ = service.add_characteristic(characteristic.clone());
        characteristic
    }

    pub fn new_empty() -> FakeBluetoothGATTCharacteristic {
        FakeBluetoothGATTCharacteristic {
            id: Arc::new(Mutex::new(String::new())),
            uuid: Arc::new(Mutex::new(String::new())),
            service: Arc::new(FakeBluetoothGATTService::new_empty()),
            value: Arc::new(Mutex::new(vec![])),
            is_notifying: Arc::new(Mutex::new(false)),
            flags: Arc::new(Mutex::new(vec![])),
            gatt_descriptors: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn get_id(&self) -> String {
        let cloned = self.id.clone();
        let id = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => String::new(),
        };
        id
    }

    pub fn set_id(&self, value: String) {
        let cloned = self.id.clone();
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

    pub fn get_service(&self) -> Result<Arc<FakeBluetoothGATTService>, Box<Error>> {
        Ok(self.service.clone())
    }

    /*pub fn set_service(&self, service: Arc<FakeBluetoothGATTService>) -> Result<(), Box<Error>> {
        Ok(self.service = service)
    }*/


    pub fn is_notifying(&self) -> Result<bool, Box<Error>> {
        let cloned = self.is_notifying.clone();
        let is_notifying = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(is_notifying)
    }

    pub fn set_notifying(&self, value: bool) -> Result<(), Box<Error>> {
        let cloned = self.is_notifying.clone();
        //TODO remove unwrap, if possible
        let mut is_notifying = cloned.lock().unwrap();
        Ok(*is_notifying = value)
    }

    pub fn start_notify(&self) -> Result<(), Box<Error>> {
        self.set_notifying(true)
    }

    pub fn stop_notify(&self) -> Result<(), Box<Error>> {
        self.set_notifying(false)
    }

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

    pub fn get_gatt_descriptors(&self) -> Result<Vec<Arc<FakeBluetoothGATTDescriptor>>, Box<Error>> {
        let cloned = self.gatt_descriptors.clone();
        let gatt_descriptors = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(gatt_descriptors)
    }

    pub fn set_gatt_descriptors(&self, descriptors: Vec<Arc<FakeBluetoothGATTDescriptor>>) -> Result<(), Box<Error>> {
        let cloned = self.gatt_descriptors.clone();
        //TODO remove unwrap, if possible
        let mut gatt_descriptors = cloned.lock().unwrap();
        Ok(*gatt_descriptors = descriptors)
    }

    pub fn add_descriptor(&self, descriptor: Arc<FakeBluetoothGATTDescriptor>) -> Result<(), Box<Error>> {
        let cloned = self.gatt_descriptors.clone();
        //TODO remove unwrap, if possible
        let mut gatt_descriptors = cloned.lock().unwrap();
        Ok(gatt_descriptors.push(descriptor))
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
