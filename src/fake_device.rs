use core::ops::Deref;
use fake_adapter::FakeBluetoothAdapter;
use fake_service::FakeBluetoothGATTService;
use rustc_serialize::hex::FromHex;
use std::error::Error;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct FakeBluetoothDevice {
    object_path: Arc<Mutex<String>>,
    adapter: Arc<FakeBluetoothAdapter>,
    address: Arc<Mutex<String>>,
    appearance: Arc<Mutex<u16>>,
    class: Arc<Mutex<u32>>,
    gatt_services: Arc<Mutex<Vec<Arc<FakeBluetoothGATTService>>>>,
    is_paired: Arc<Mutex<bool>>,
    is_connectable: Arc<Mutex<bool>>,
    is_connected: Arc<Mutex<bool>>,
    is_trusted: Arc<Mutex<bool>>,
    is_blocked: Arc<Mutex<bool>>,
    is_legacy_pairing: Arc<Mutex<bool>>,
    uuids: Arc<Mutex<Vec<String>>>,
    name: Arc<Mutex<String>>,
    icon: Arc<Mutex<String>>,
    alias: Arc<Mutex<String>>,
    product_version: Arc<Mutex<u32>>,
    rssi: Arc<Mutex<i16>>,
    tx_power: Arc<Mutex<i16>>,
    modalias: Arc<Mutex<String>>,
}

impl FakeBluetoothDevice {
    /*pub fn new_initialized(object_path: String,
               adapter: Arc<FakeBluetoothAdapter>,
               address: String,
               appearance: u16,
               class: u32,
               gatt_services: Vec<FakeBluetoothGATTService>,
               is_paired: bool,
               is_connectable: bool,
               is_connected: bool,
               is_trusted: bool,
               is_blocked: bool,
               is_legacy_pairing: bool,
               uuids: Vec<String>,
               name: String,
               icon: String,
               alias: String,
               product_version: u32,
               rssi: i16,
               tx_power: i16,
               modalias: String)
               -> FakeBluetoothDevice {
        FakeBluetoothDevice{
            object_path: Arc::new(Mutex::new(object_path)),
            adapter: Arc::new(Mutex::new(adapter)),
            address: Arc::new(Mutex::new(address)),
            appearance: Arc::new(Mutex::new(appearance)),
            class: Arc::new(Mutex::new(class)),
            gatt_services: Arc::new(Mutex::new(gatt_services)),
            is_paired: Arc::new(Mutex::new(is_paired)),
            is_connectable: Arc::new(Mutex::new(is_connectable)),
            is_connected: Arc::new(Mutex::new(is_connected)),
            is_trusted: Arc::new(Mutex::new(is_trusted)),
            is_blocked: Arc::new(Mutex::new(is_blocked)),
            is_legacy_pairing: Arc::new(Mutex::new(is_legacy_pairing)),
            uuids: Arc::new(Mutex::new(uuids)),
            name: Arc::new(Mutex::new(name)),
            icon: Arc::new(Mutex::new(icon)),
            alias: Arc::new(Mutex::new(alias)),
            product_version: Arc::new(Mutex::new(product_version)),
            rssi: Arc::new(Mutex::new(rssi)),
            tx_power: Arc::new(Mutex::new(tx_power)),
            modalias: Arc::new(Mutex::new(modalias)),
        }
    }*/

    pub fn new(adapter: Arc<FakeBluetoothAdapter>,
               name: String)
               -> Arc<FakeBluetoothDevice> {
        if let Ok(existing_device) = adapter.get_device(name.clone()) {
            return existing_device;
        }
        let device = Arc::new(FakeBluetoothDevice{
            object_path: Arc::new(Mutex::new(String::new())),
            adapter: adapter.clone(),
            address: Arc::new(Mutex::new(String::new())),
            appearance: Arc::new(Mutex::new(0)),
            class: Arc::new(Mutex::new(0)),
            gatt_services: Arc::new(Mutex::new(vec![])),
            is_paired: Arc::new(Mutex::new(false)),
            is_connectable: Arc::new(Mutex::new(false)),
            is_connected: Arc::new(Mutex::new(false)),
            is_trusted: Arc::new(Mutex::new(false)),
            is_blocked: Arc::new(Mutex::new(false)),
            is_legacy_pairing: Arc::new(Mutex::new(false)),
            uuids: Arc::new(Mutex::new(vec![])),
            name: Arc::new(Mutex::new(name)),
            icon: Arc::new(Mutex::new(String::new())),
            alias: Arc::new(Mutex::new(String::new())),
            product_version: Arc::new(Mutex::new(0)),
            rssi: Arc::new(Mutex::new(0)),
            tx_power: Arc::new(Mutex::new(0)),
            modalias: Arc::new(Mutex::new(String::new())),
        });
        let _ = adapter.add_device(device.clone());
        device
    }

    pub fn new_empty() -> FakeBluetoothDevice {
        FakeBluetoothDevice{
            object_path: Arc::new(Mutex::new(String::new())),
            adapter: Arc::new(FakeBluetoothAdapter::new_empty()),
            address: Arc::new(Mutex::new(String::new())),
            appearance: Arc::new(Mutex::new(0)),
            class: Arc::new(Mutex::new(0)),
            gatt_services: Arc::new(Mutex::new(vec![])),
            is_paired: Arc::new(Mutex::new(false)),
            is_connectable: Arc::new(Mutex::new(false)),
            is_connected: Arc::new(Mutex::new(false)),
            is_trusted: Arc::new(Mutex::new(false)),
            is_blocked: Arc::new(Mutex::new(false)),
            is_legacy_pairing: Arc::new(Mutex::new(false)),
            uuids: Arc::new(Mutex::new(vec![])),
            name: Arc::new(Mutex::new(String::new())),
            icon: Arc::new(Mutex::new(String::new())),
            alias: Arc::new(Mutex::new(String::new())),
            product_version: Arc::new(Mutex::new(0)),
            rssi: Arc::new(Mutex::new(0)),
            tx_power: Arc::new(Mutex::new(0)),
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

    pub fn set_id(&self, value: String) {
        let cloned = self.object_path.clone();
        //TODO remove unwrap, if possible
        let mut id = cloned.lock().unwrap();
        *id = value;
    }


    pub fn get_adapter(&self) -> Result<Arc<FakeBluetoothAdapter>, Box<Error>> {
        Ok(self.adapter.clone())
    }
/*
    pub fn set_adapter(&self, adapter: Arc<FakeBluetoothAdapter>) -> Result<(), Box<Error>> {
        Ok(self.adapter = adapter)
    }
*/

    pub fn get_address(&self) -> Result<String, Box<Error>> {
        let cloned = self.address.clone();
        let address = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(address)
    }

    pub fn set_address(&self, value: String) -> Result<(), Box<Error>> {
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

    pub fn set_name(&self, value: String) -> Result<(), Box<Error>> {
        let cloned = self.name.clone();
        //TODO remove unwrap, if possible
        let mut name = cloned.lock().unwrap();
        Ok(*name = value)
    }

    pub fn get_icon(&self) -> Result<String, Box<Error>> {
        let cloned = self.icon.clone();
        let icon = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(icon)
    }

    pub fn set_icon(&self, value: String) -> Result<(), Box<Error>> {
        let cloned = self.icon.clone();
        //TODO remove unwrap, if possible
        let mut icon = cloned.lock().unwrap();
        Ok(*icon = value)
    }

    pub fn get_class(&self) -> Result<u32, Box<Error>> {
        let cloned = self.class.clone();
        let class = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(class)
    }

    pub fn set_class(&self, value: u32) -> Result<(), Box<Error>> {
        let cloned = self.class.clone();
        //TODO remove unwrap, if possible
        let mut class = cloned.lock().unwrap();
        Ok(*class = value)
    }

    pub fn get_appearance(&self) -> Result<u16, Box<Error>> {
        let cloned = self.appearance.clone();
        let appearance = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(appearance)
    }

    pub fn set_appearance(&self, value: u16) -> Result<(), Box<Error>> {
        let cloned = self.appearance.clone();
        //TODO remove unwrap, if possible
        let mut appearance = cloned.lock().unwrap();
        Ok(*appearance = value)
    }

    pub fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        let cloned = self.uuids.clone();
        let uuids = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(uuids)
    }

    pub fn set_uuids(&self, value: Vec<String>) -> Result<(), Box<Error>> {
        let cloned = self.uuids.clone();
        //TODO remove unwrap, if possible
        let mut uuids = cloned.lock().unwrap();
        Ok(*uuids = value)
    }

    pub fn is_paired(&self) -> Result<bool, Box<Error>> {
        let cloned = self.is_paired.clone();
        let is_paired = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(is_paired)
    }

    pub fn pair(&self) -> Result<(), Box<Error>> {
        let cloned = self.is_paired.clone();
        //TODO remove unwrap, if possible
        let mut is_paired = cloned.lock().unwrap();
        Ok(*is_paired = true)
    }

    pub fn cancel_pairing(&self) -> Result<(), Box<Error>> {
        let cloned = self.is_paired.clone();
        //TODO remove unwrap, if possible
        let mut is_paired = cloned.lock().unwrap();
        Ok(*is_paired = false)
    }

    pub fn is_connected(&self) -> Result<bool, Box<Error>> {
        let cloned = self.is_connected.clone();
        let is_connected = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(is_connected)
    }

    pub fn set_connected(&self, value: bool) -> Result<(), Box<Error>> {
        let cloned = self.is_connected.clone();
        //TODO remove unwrap, if possible
        let mut is_connected = cloned.lock().unwrap();
        Ok(*is_connected = value)
    }

    pub fn is_connectable(&self) -> Result<bool, Box<Error>> {
        let cloned = self.is_connectable.clone();
        let is_connectable = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(is_connectable)
    }

    pub fn set_connectable(&self, value: bool) -> Result<(), Box<Error>> {
        let cloned = self.is_connectable.clone();
        //TODO remove unwrap, if possible
        let mut is_connectable = cloned.lock().unwrap();
        Ok(*is_connectable = value)
    }

    pub fn is_trusted(&self) -> Result<bool, Box<Error>> {
        let cloned = self.is_trusted.clone();
        let is_trusted = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(is_trusted)
    }

    pub fn set_trusted(&self, value: bool) -> Result<(), Box<Error>> {
        let cloned = self.is_trusted.clone();
        //TODO remove unwrap, if possible
        let mut is_trusted = cloned.lock().unwrap();
        Ok(*is_trusted = value)
    }

    pub fn is_blocked(&self) -> Result<bool, Box<Error>> {
        let cloned = self.is_blocked.clone();
        let is_blocked = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(is_blocked)
    }

    pub fn set_blocked(&self, value: bool) -> Result<(), Box<Error>> {
        let cloned = self.is_blocked.clone();
        //TODO remove unwrap, if possible
        let mut is_blocked = cloned.lock().unwrap();
        Ok(*is_blocked = value)
    }

    pub fn get_alias(&self) -> Result<String, Box<Error>> {
        let cloned = self.alias.clone();
        let alias = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(alias)
    }

    pub fn set_alias(&self, value: String) -> Result<(), Box<Error>> {
        let cloned = self.alias.clone();
        //TODO remove unwrap, if possible
        let mut alias = cloned.lock().unwrap();
        Ok(*alias = value)
    }

    pub fn is_legacy_pairing(&self) -> Result<bool, Box<Error>> {
        let cloned = self.is_legacy_pairing.clone();
        let is_legacy_pairing = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(is_legacy_pairing)
    }

    pub fn set_legacy_pairing(&self, value: bool) -> Result<(), Box<Error>> {
        let cloned = self.is_legacy_pairing.clone();
        //TODO remove unwrap, if possible
        let mut is_legacy_pairing = cloned.lock().unwrap();
        Ok(*is_legacy_pairing = value)
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

    pub fn set_modalias(&self, value: String) -> Result<(), Box<Error>> {
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

    pub fn get_rssi(&self) -> Result<i16, Box<Error>> {
        let cloned = self.rssi.clone();
        let rssi = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(rssi)
    }

    pub fn set_rssi(&self, value: i16) -> Result<(), Box<Error>> {
        let cloned = self.rssi.clone();
        //TODO remove unwrap, if possible
        let mut rssi = cloned.lock().unwrap();
        Ok(*rssi = value)
    }

    pub fn get_tx_power(&self) -> Result<i16, Box<Error>> {
        let cloned = self.tx_power.clone();
        let tx_power = match cloned.lock() {
            Ok(guard) => *guard.deref(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(tx_power)
    }

    pub fn set_tx_power(&self, value: i16) -> Result<(), Box<Error>> {
        let cloned = self.tx_power.clone();
        //TODO remove unwrap, if possible
        let mut tx_power = cloned.lock().unwrap();
        Ok(*tx_power = value)
    }

    pub fn get_gatt_services(&self) -> Result<Vec<Arc<FakeBluetoothGATTService>>, Box<Error>> {
        let cloned = self.gatt_services.clone();
        let gatt_services = match cloned.lock() {
            Ok(guard) => guard.deref().clone(),
            Err(_) => return Err(Box::from("Could not get the value.")),
        };
        Ok(gatt_services)
    }

    pub fn set_gatt_service(&self, services: Vec<Arc<FakeBluetoothGATTService>>) -> Result<(), Box<Error>> {
        let cloned = self.gatt_services.clone();
        //TODO remove unwrap, if possible
        let mut gatt_services = cloned.lock().unwrap();
        Ok(*gatt_services = services)
    }

    pub fn add_service(&self, service: Arc<FakeBluetoothGATTService>) -> Result<(), Box<Error>> {
        let cloned = self.gatt_services.clone();
        //TODO remove unwrap, if possible
        let mut gatt_services = cloned.lock().unwrap();
        Ok(gatt_services.push(service))
    }

    pub fn connect_profile(&self, _uuid: String) -> Result<(), Box<Error>> {
        unimplemented!();
    }

    pub fn disconnect_profile(&self, _uuid: String) -> Result<(), Box<Error>> {
        unimplemented!();
    }

    pub fn connect(&self) -> Result<(), Box<Error>> {
        //TODO remove unwrap
        if self.is_connectable().unwrap() && !self.is_connected().unwrap() {
            return self.set_connected(true);
        } else {
            return Err(Box::from("Could not connect to the device."));
        }
    }

    pub fn disconnect(&self) -> Result<(), Box<Error>>{
        //TODO remove unwrap
        if self.is_connected().unwrap() {
            return self.set_connected(false);
        } else {
            return Err(Box::from("The device is not connected."));
        }
    }
}
