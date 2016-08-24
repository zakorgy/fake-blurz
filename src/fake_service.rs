use core::ops::Deref;
use fake_characteristic::FakeBluetoothGATTCharacteristic;
use fake_device::FakeBluetoothDevice;
use std::error::Error;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct FakeBluetoothGATTService {
    id: Arc<Mutex<String>>,
    device: Arc<FakeBluetoothDevice>,
    gatt_characteristics: Arc<Mutex<Vec<Arc<FakeBluetoothGATTCharacteristic>>>>,
    is_primary: Arc<Mutex<bool>>,
    included_services: Arc<Mutex<Vec<Arc<FakeBluetoothGATTService>>>>,
    uuid: Arc<Mutex<String>>,
}

impl FakeBluetoothGATTService {
    /*pub fn new(id: String,
               device: FakeBluetoothDevice,
               gatt_characteristics: Vec<Arc<FakeBluetoothGATTCharacteristic>>,
               is_primary: bool,
               included_services: Vec<Arc<FakeBluetoothGATTService>>,
               uuid: String)
               -> FakeBluetoothGATTService {
        FakeBluetoothGATTService {
            id: Arc::new(Mutex::new(id)),
            device: Arc::new(device),
            gatt_characteristics: Arc::new(Mutex::new(gatt_characteristics)),
            is_primary: Arc::new(Mutex::new(is_primary)),
            included_services: Arc::new(Mutex::new(included_services)),
            uuid: Arc::new(Mutex::new(uuid)),
        }
    }*/

    pub fn new(id: String,
               device: Arc<FakeBluetoothDevice>)
               -> Arc<FakeBluetoothGATTService> {
        let service = Arc::new(FakeBluetoothGATTService {
            id: Arc::new(Mutex::new(id)),
            device: device.clone(),
            gatt_characteristics: Arc::new(Mutex::new(vec![])),
            is_primary: Arc::new(Mutex::new(false)),
            included_services: Arc::new(Mutex::new(vec![])),
            uuid: Arc::new(Mutex::new(String::new())),
        });
        let _ = device.add_service(service.clone());
        service
    }

    pub fn new_empty() -> FakeBluetoothGATTService {
        FakeBluetoothGATTService {
            id: Arc::new(Mutex::new(String::new())),
            device: Arc::new(FakeBluetoothDevice::new_empty()),
            gatt_characteristics: Arc::new(Mutex::new(vec![])),
            is_primary: Arc::new(Mutex::new(false)),
            included_services: Arc::new(Mutex::new(vec![])),
            uuid: Arc::new(Mutex::new(String::new())),
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

    pub fn get_device(&self) -> Result<Arc<FakeBluetoothDevice>, Box<Error>> {
        Ok(self.device.clone())
    }

    /*pub fn set_device(&self, device: Arc<FakeBluetoothDevice>) -> Result<(), Box<Error>> {
        Ok(self.device = device)
    }*/

    pub fn get_gatt_characteristics(&self) -> Result<Vec<Arc<FakeBluetoothGATTCharacteristic>>, Box<Error>> {
        let cloned = self.gatt_characteristics.clone();
        let gatt_characteristics = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(gatt_characteristics)
    }

    pub fn set_gatt_characteristics(&self, characteristics: Vec<Arc<FakeBluetoothGATTCharacteristic>>) -> Result<(), Box<Error>> {
        let cloned = self.gatt_characteristics.clone();
        //TODO remove unwrap, if possible
        let mut gatt_characteristics = cloned.lock().unwrap();
        Ok(*gatt_characteristics = characteristics)
    }

    pub fn add_characteristic(&self, characteristic: Arc<FakeBluetoothGATTCharacteristic>) -> Result<(), Box<Error>> {
        let cloned = self.gatt_characteristics.clone();
        //TODO remove unwrap, if possible
        let mut gatt_characteristics = cloned.lock().unwrap();
        Ok(gatt_characteristics.push(characteristic))
    }

    pub fn is_primary(&self) -> Result<bool, Box<Error>> {
        let cloned = self.is_primary.clone();
        let is_primary = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(is_primary)
    }

    pub fn set_is_primary(&self, value: bool) -> Result<(), Box<Error>> {
        let cloned = self.is_primary.clone();
        //TODO remove unwrap, if possible
        let mut is_primary = cloned.lock().unwrap();
        Ok(*is_primary = value)
    }

    pub fn get_includes(&self) -> Result<Vec<Arc<FakeBluetoothGATTService>>, Box<Error>> {
        let cloned = self.included_services.clone();
        let included_services = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(included_services)
    }

    pub fn set_includes(&self, value: Vec<Arc<FakeBluetoothGATTService>>) -> Result<(), Box<Error>> {
        let cloned = self.included_services.clone();
        //TODO remove unwrap, if possible
        let mut included_services = cloned.lock().unwrap();
        Ok(*included_services = value)
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
}
