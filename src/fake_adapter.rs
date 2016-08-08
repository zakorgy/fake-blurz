//use fake_device::String;
//use fake_discovery_session::FakeBluetoothDiscoverySession;
use std::error::Error;
use std::sync::{Arc, Mutex};
use rustc_serialize::hex::FromHex;
use core::ops::Deref;

#[derive(Clone, Debug)]
pub struct FakeBluetoothAdapter {
    object_path: Arc<Mutex<String>>,
    is_present: Arc<Mutex<bool>>,
    is_powered: bool,
    can_start_discovery: bool,
    can_stop_discovery: bool,
    devices: Vec<Arc<String>>,
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
    modalias: String,
}

impl FakeBluetoothAdapter {
    pub fn new(object_path: String,
               is_present: bool,
               is_powered: bool,
               can_start_discovery: bool,
               can_stop_discovery: bool,
               devices: Vec<Arc<String>>,
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
            is_powered: is_powered,
            can_start_discovery: can_start_discovery,
            can_stop_discovery: can_stop_discovery,
            devices: devices,
            addatas: addatas,
            address: address,
            name: name,
            alias: alias,
            class: class,
            is_discoverable: is_discoverable,
            is_pairable: is_pairable,
            pairable_timeout: pairable_timeout,
            discoverable_timeout: discoverable_timeout,
            is_discovering: is_discovering,
            uuids: uuids,
            modalias: modalias,
        }
    }

    pub fn new_empty() -> FakeBluetoothAdapter {
        FakeBluetoothAdapter {
            object_path: Arc::new(Mutex::new(String::new())),
            is_present: Arc::new(Mutex::new(false)),
            is_powered: false,
            can_start_discovery: false,
            can_stop_discovery: false,
            devices: vec![],
            addatas: vec![],
            address: String::new(),
            name: String::new(),
            alias: String::new(),
            class: 0,
            is_discoverable: false,
            is_pairable: false,
            pairable_timeout: 0,
            discoverable_timeout: 0,
            is_discovering: false,
            uuids: vec![],
            modalias: String::new(),
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
        ///TODO remove unwrap, if possible
        let mut is_present = cloned.lock().unwrap();
        Ok(*is_present = value)
    }

    pub fn is_powered(&self) -> Result<bool, Box<Error>> {
        Ok(self.is_powered)
    }

    pub fn set_powered(&mut self, value: bool) -> Result<(), Box<Error>> {
        Ok(self.is_powered = value)
    }

    pub fn get_can_start_discovery(&self) -> Result<bool, Box<Error>> {
        Ok(self.can_start_discovery)
    }

    pub fn set_can_start_discovery(&mut self, value: bool) -> Result<(), Box<Error>> {
        Ok(self.can_start_discovery = value)
    }

    pub fn get_can_stop_siscovery(&self) -> Result<bool, Box<Error>> {
        Ok(self.can_stop_discovery)
    }

    pub fn set_can_stop_discovery(&mut self, value: bool) -> Result<(), Box<Error>> {
        Ok(self.can_stop_discovery = value)
    }

    /*pub fn get_device_list(&self) -> Result<Vec<String>, Box<Error>> {
        let mut names = vec![];
        for device in &self.devices {
            names.push(device.get_name().unwrap());
        }
        Ok(names)
    }*/

    pub fn get_devices(&self) -> Result<Vec<Arc<String>>, Box<Error>> {
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
    }

    pub fn get_addatas(&self) -> Result<Vec<String>, Box<Error>> {
        Ok(self.addatas.clone())
    }

    pub fn set_addatas(&mut self, addatas: Vec<String>) -> Result<(), Box<Error>> {
        Ok(self.addatas = addatas)
    }

    pub fn get_first_addata(&self) -> Result<String, Box<Error>> {
        if self.addatas.is_empty() {
            return Err(Box::from("No addata found."))
        }
        Ok(self.addatas[0].clone())
    }

    pub fn get_address(&self) -> Result<String, Box<Error>> {
        Ok(self.address.clone())
    }

    pub fn set_address(&mut self, address: String) -> Result<(), Box<Error>> {
        Ok(self.address = address)
    }

    pub fn get_name(&self) -> Result<String, Box<Error>> {
        Ok(self.name.clone())
    }

    pub fn set_name(&mut self, name: String) -> Result<(), Box<Error>> {
        Ok(self.name = name)
    }

    /*pub fn create_discovery_session(&self) -> Result<FakeBluetoothDiscoverySession, Box<Error>> {
        FakeBluetoothDiscoverySession::create_session(Arc::new(self.clone()))
    }*/

    pub fn get_alias(&self) -> Result<String, Box<Error>> {
        Ok(self.alias.clone())
    }

    pub fn set_alias(&mut self, value: String) -> Result<(), Box<Error>> {
        self.alias = value;
        Ok(())
    }

    pub fn get_class(&self) -> Result<u32, Box<Error>> {
        Ok(self.class)
    }

    pub fn set_class(&mut self, value: u32) -> Result<(), Box<Error>> {
        Ok(self.class = value)
    }

    pub fn is_discoverable(&self) -> Result<bool, Box<Error>> {
        Ok(self.is_discoverable)
    }

    pub fn set_discoverable(&mut self, value: bool) -> Result<(), Box<Error>> {
        Ok(self.is_discoverable = value)
    }

    pub fn is_pairable(&self) -> Result<bool, Box<Error>> {
        Ok(self.is_pairable)
    }

    pub fn set_pairable(&mut self, value: bool) -> Result<(), Box<Error>> {
        Ok(self.is_pairable = value)
    }

    pub fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
        Ok(self.pairable_timeout)
    }

    pub fn set_pairable_timeout(&mut self, value: u32) -> Result<(), Box<Error>> {
        Ok(self.pairable_timeout = value)
    }

    pub fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
        Ok(self.discoverable_timeout)
    }

    pub fn set_discoverable_timeout(&mut self, value: u32) -> Result<(), Box<Error>> {
        Ok(self.discoverable_timeout = value)
    }

    pub fn is_discovering(&self) -> Result<bool, Box<Error>> {
        Ok(self.is_discovering)
    }

    pub fn set_discovering(&mut self, value: bool) -> Result<(), Box<Error>> {
        Ok(self.is_discovering = value)
    }

    pub fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        Ok(self.uuids.clone())
    }

    pub fn set_uuids(&mut self, value: Vec<String>) -> Result<(), Box<Error>> {
        Ok(self.uuids = value)
    }

    pub fn get_modalias(&self) ->  Result<(String, u32, u32, u32), Box<Error>> {
        let ids: Vec<&str> = self.modalias.split(":").collect();

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
        Ok(self.modalias = value)
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
