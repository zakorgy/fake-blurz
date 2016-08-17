//use fake_device::String;
use fake_discovery_session::FakeBluetoothDiscoverySession;
use std::error::Error;
use std::sync::{Arc, Mutex};
use rustc_serialize::hex::FromHex;
use core::ops::Deref;

#[derive(Clone, Debug)]
pub struct FakeBluetoothAdapter {
    object_path: Arc<Mutex<String>>,
    is_present: Arc<Mutex<bool>>,
    is_powered: Arc<Mutex<bool>>,
    can_start_discovery: Arc<Mutex<bool>>,
    can_stop_discovery: Arc<Mutex<bool>>,
    devices: Arc<Mutex<Vec<String>>>,
    addatas: Arc<Mutex<Vec<String>>>,
    address: Arc<Mutex<String>>,
    name: Arc<Mutex<String>>,
    alias: Arc<Mutex<String>>,
    class: Arc<Mutex<u32>>,
    is_discoverable: Arc<Mutex<bool>>,
    is_pairable: Arc<Mutex<bool>>,
    pairable_timeout: Arc<Mutex<u32>>,
    discoverable_timeout: Arc<Mutex<u32>>,
    is_discovering: Arc<Mutex<bool>>,
    uuids: Arc<Mutex<Vec<String>>>,
    modalias: Arc<Mutex<String>>,
}

impl FakeBluetoothAdapter {
    pub fn new(object_path: String,
               is_present: bool,
               is_powered: bool,
               can_start_discovery: bool,
               can_stop_discovery: bool,
               devices: Vec<String>,
               addatas: Vec<String>,
               address: String,
               name: String,
               alias: String,
               class: u32,
               is_discoverable: bool,
               is_pairable: bool,
               pairable_timeout: u32,
               discoverable_timeout: u32,
               is_discovering: bool,
               uuids: Vec<String>,
               modalias: String)
               ->FakeBluetoothAdapter {
        FakeBluetoothAdapter {
            object_path: Arc::new(Mutex::new(object_path)),
            is_present: Arc::new(Mutex::new(is_present)),
            is_powered: Arc::new(Mutex::new(is_powered)),
            can_start_discovery: Arc::new(Mutex::new(can_start_discovery)),
            can_stop_discovery: Arc::new(Mutex::new(can_stop_discovery)),
            devices: Arc::new(Mutex::new(devices)),
            addatas: Arc::new(Mutex::new(addatas)),
            address: Arc::new(Mutex::new(address)),
            name: Arc::new(Mutex::new(name)),
            alias: Arc::new(Mutex::new(alias)),
            class: Arc::new(Mutex::new(class)),
            is_discoverable: Arc::new(Mutex::new(is_discoverable)),
            is_pairable: Arc::new(Mutex::new(is_pairable)),
            pairable_timeout: Arc::new(Mutex::new(pairable_timeout)),
            discoverable_timeout: Arc::new(Mutex::new(discoverable_timeout)),
            is_discovering: Arc::new(Mutex::new(is_discovering)),
            uuids: Arc::new(Mutex::new(uuids)),
            modalias: Arc::new(Mutex::new(modalias)),
        }
    }

    pub fn new_empty() -> FakeBluetoothAdapter {
        FakeBluetoothAdapter {
            object_path: Arc::new(Mutex::new(String::new())),
            is_present: Arc::new(Mutex::new(false)),
            is_powered: Arc::new(Mutex::new(false)),
            can_start_discovery: Arc::new(Mutex::new(false)),
            can_stop_discovery: Arc::new(Mutex::new(false)),
            devices: Arc::new(Mutex::new(vec![])),
            addatas: Arc::new(Mutex::new(vec![])),
            address: Arc::new(Mutex::new(String::new())),
            name: Arc::new(Mutex::new(String::new())),
            alias: Arc::new(Mutex::new(String::new())),
            class: Arc::new(Mutex::new(0)),
            is_discoverable: Arc::new(Mutex::new(false)),
            is_pairable: Arc::new(Mutex::new(false)),
            pairable_timeout: Arc::new(Mutex::new(0)),
            discoverable_timeout: Arc::new(Mutex::new(0)),
            is_discovering: Arc::new(Mutex::new(false)),
            uuids: Arc::new(Mutex::new(vec![])),
            modalias: Arc::new(Mutex::new(String::new())),
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

    pub fn set_id(&mut self, value: String) {
        let cloned = self.object_path.clone();
        //TODO remove unwrap, if possible
        let mut id = cloned.lock().unwrap();
        *id = value;
    }

    pub fn is_present(&self) -> Result<bool, Box<Error>> {
        let cloned = self.is_present.clone();
        let is_present = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(is_present)
    }

    pub fn set_present(&mut self, value: bool) -> Result<(), Box<Error>> {
        let cloned = self.is_present.clone();
        //TODO remove unwrap, if possible
        let mut is_present = cloned.lock().unwrap();
        Ok(*is_present = value)
    }

    pub fn is_powered(&self) -> Result<bool, Box<Error>> {
        let cloned = self.is_powered.clone();
        let is_powered = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(is_powered)
    }

    pub fn set_powered(&mut self, value: bool) -> Result<(), Box<Error>> {
        let cloned = self.is_powered.clone();
        //TODO remove unwrap, if possible
        let mut is_powered = cloned.lock().unwrap();
        Ok(*is_powered = value)
    }

    pub fn get_can_start_discovery(&self) -> Result<bool, Box<Error>> {
        let cloned = self.can_start_discovery.clone();
        let can_start_discovery = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(can_start_discovery)
    }

    pub fn set_can_start_discovery(&mut self, value: bool) -> Result<(), Box<Error>> {
        let cloned = self.can_start_discovery.clone();
        //TODO remove unwrap, if possible
        let mut can_start_discovery = cloned.lock().unwrap();
        Ok(*can_start_discovery = value)
    }

    pub fn get_can_stop_siscovery(&self) -> Result<bool, Box<Error>> {
        let cloned = self.can_stop_discovery.clone();
        let can_stop_discovery = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(can_stop_discovery)
    }

    pub fn set_can_stop_discovery(&mut self, value: bool) -> Result<(), Box<Error>> {
        let cloned = self.can_stop_discovery.clone();
        //TODO remove unwrap, if possible
        let mut can_stop_discovery = cloned.lock().unwrap();
        Ok(*can_stop_discovery = value)
    }

    /*pub fn get_device_list(&self) -> Result<Vec<String>, Box<Error>> {
        let mut names = vec![];
        for device in &self.devices {
            names.push(device.get_name().unwrap());
        }
        Ok(names)
    }

    pub fn get_devices(&self) -> Result<Vec<String>, Box<Error>> {
        Ok(self.devices.clone())
    }

    pub fn set_devices(&mut self, devices: Vec<Arc<String>>) -> Result<(), Box<Error>> {
        Ok(self.devices = devices)
    }

    pub fn add_device(&mut self, device: Arc<String>) -> Result<(), Box<Error>> {
        Ok(self.devices.push(device))
    }

    pub fn get_first_device(&self) -> Result<Arc<String>, Box<Error>> {
        if self.devices.is_empty() {
            return Err(Box::from("No device found."))
        }
        Ok(self.devices[0].clone())
    }*/

    pub fn get_addatas(&self) -> Result<Vec<String>, Box<Error>> {
        let cloned = self.addatas.clone();
        let addatas = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(addatas)
    }

    pub fn set_addatas(&mut self, value: Vec<String>) -> Result<(), Box<Error>> {
        let cloned = self.addatas.clone();
        //TODO remove unwrap, if possible
        let mut addatas = cloned.lock().unwrap();
        Ok(*addatas = value)
    }

    pub fn get_first_addata(&self) -> Result<String, Box<Error>> {
        let cloned = self.addatas.clone();
        let addatas = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        if addatas.is_empty() {
            return Err(Box::from("No addata found."))
        }
        Ok(addatas[0].clone())
    }

    pub fn get_address(&self) -> Result<String, Box<Error>> {
        let cloned = self.address.clone();
        let address = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(address)
    }

    pub fn set_address(&mut self, value: String) -> Result<(), Box<Error>> {
        let cloned = self.address.clone();
        //TODO remove unwrap, if possible
        let mut address = cloned.lock().unwrap();
        Ok(*address = value)
    }

    pub fn get_name(&self) -> Result<String, Box<Error>> {
        let cloned = self.name.clone();
        let name = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(name)
    }

    pub fn set_name(&mut self, value: String) -> Result<(), Box<Error>> {
        let cloned = self.name.clone();
        //TODO remove unwrap, if possible
        let mut name = cloned.lock().unwrap();
        Ok(*name = value)
    }

    pub fn create_discovery_session(&self) -> Result<FakeBluetoothDiscoverySession, Box<Error>> {
        FakeBluetoothDiscoverySession::create_session(Arc::new(self.clone()))
    }

    pub fn get_alias(&self) -> Result<String, Box<Error>> {
        let cloned = self.alias.clone();
        let alias = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(alias)
    }

    pub fn set_alias(&mut self, value: String) -> Result<(), Box<Error>> {
        let cloned = self.alias.clone();
        //TODO remove unwrap, if possible
        let mut alias = cloned.lock().unwrap();
        Ok(*alias = value)
    }

    pub fn get_class(&self) -> Result<u32, Box<Error>> {
        let cloned = self.class.clone();
        let class = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(class)
    }

    pub fn set_class(&mut self, value: u32) -> Result<(), Box<Error>> {
        let cloned = self.class.clone();
        //TODO remove unwrap, if possible
        let mut class = cloned.lock().unwrap();
        Ok(*class = value)
    }

    pub fn is_discoverable(&self) -> Result<bool, Box<Error>> {
        let cloned = self.is_discoverable.clone();
        let is_discoverable = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(is_discoverable)
    }

    pub fn set_discoverable(&mut self, value: bool) -> Result<(), Box<Error>> {
        let cloned = self.is_discoverable.clone();
        //TODO remove unwrap, if possible
        let mut is_discoverable = cloned.lock().unwrap();
        Ok(*is_discoverable = value)
    }

    pub fn is_pairable(&self) -> Result<bool, Box<Error>> {
        let cloned = self.is_pairable.clone();
        let is_pairable = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(is_pairable)
    }

    pub fn set_pairable(&mut self, value: bool) -> Result<(), Box<Error>> {
        let cloned = self.is_pairable.clone();
        //TODO remove unwrap, if possible
        let mut is_pairable = cloned.lock().unwrap();
        Ok(*is_pairable = value)
    }

    pub fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
        let cloned = self.pairable_timeout.clone();
        let pairable_timeout = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(pairable_timeout)
    }

    pub fn set_pairable_timeout(&mut self, value: u32) -> Result<(), Box<Error>> {
        let cloned = self.pairable_timeout.clone();
        //TODO remove unwrap, if possible
        let mut pairable_timeout = cloned.lock().unwrap();
        Ok(*pairable_timeout = value)
    }

    pub fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
        let cloned = self.discoverable_timeout.clone();
        let discoverable_timeout = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(discoverable_timeout)
    }

    pub fn set_discoverable_timeout(&mut self, value: u32) -> Result<(), Box<Error>> {
        let cloned = self.discoverable_timeout.clone();
        //TODO remove unwrap, if possible
        let mut discoverable_timeout = cloned.lock().unwrap();
        Ok(*discoverable_timeout = value)
    }

    pub fn is_discovering(&self) -> Result<bool, Box<Error>> {
        let cloned = self.is_discovering.clone();
        let is_discovering = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(is_discovering)
    }

    pub fn set_discovering(&mut self, value: bool) -> Result<(), Box<Error>> {
        let cloned = self.is_discovering.clone();
        //TODO remove unwrap, if possible
        let mut is_discovering = cloned.lock().unwrap();
        Ok(*is_discovering = value)
    }

    pub fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        let cloned = self.uuids.clone();
        let uuids = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(uuids)
    }

    pub fn set_uuids(&mut self, value: Vec<String>) -> Result<(), Box<Error>> {
        let cloned = self.uuids.clone();
        //TODO remove unwrap, if possible
        let mut uuids = cloned.lock().unwrap();
        Ok(*uuids = value)
    }

    pub fn get_modalias(&self) ->  Result<(String, u32, u32, u32), Box<Error>> {
        let cloned = self.modalias.clone();
        let modalias = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        let ids: Vec<&str> = modalias.split(":").collect();

        let source = String::from(ids[0]);
        let vendor = ids[1][1..5].from_hex().unwrap();
        let product = ids[1][6..10].from_hex().unwrap();
        let device = ids[1][11..15].from_hex().unwrap();

        Ok((source,
        (vendor[0] as u32) * 16 * 16 + (vendor[1] as u32),
        (product[0] as u32) * 16 * 16 + (product[1] as u32),
        (device[0] as u32) * 16 * 16 + (device[1] as u32)))
    }

    pub fn set_modalias(&mut self, value: String) -> Result<(), Box<Error>> {
        let cloned = self.modalias.clone();
        //TODO remove unwrap, if possible
        let mut modalias = cloned.lock().unwrap();
        Ok(*modalias = value)
    }

    pub fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        let (vendor_id_source,_,_,_) = try!(self.get_modalias());
        Ok(vendor_id_source)
    }

    pub fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        let (_,vendor_id,_,_) = try!(self.get_modalias());
        Ok(vendor_id)
    }

    pub fn get_product_id(&self) -> Result<u32, Box<Error>> {
        let (_,_,product_id,_) = try!(self.get_modalias());
        Ok(product_id)
    }

    pub fn get_device_id(&self) -> Result<u32, Box<Error>> {
        let (_,_,_,device_id) = try!(self.get_modalias());
        Ok(device_id)
    }
}
