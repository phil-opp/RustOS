use std::io::IoResult;
use pci::Pci;

pub trait Driver {

  fn init(&mut self);

}
/*
pub trait DriverManager {

  fn new() -> Box<DriverManager>;
  
  fn get_drivers(&mut self) -> Vec<DriverType>;

}

pub struct CentralManager {
  sub: Vec<DriverManager>
}

impl CentralManager {

  fn new() -> CentralManager {
    CentralManager { sub: vec!(box Pci::new() as DriverManager) }
  }
  
  fn get_drivers(&self) {
    self.sub.flat_map(|manager| manager.get_drivers())
  }

}






pub struct PciManifest {
  register_limit: uint,
  device_id: u16,
  vendor_id: u16,
  bus_master: bool
}



pub trait PciDeviceDriver {

  fn new() -> Box<PciDeviceDriver>;

  fn manifest(&self) -> PciManifest;

}

pub enum DriverType {
  Net(NetworkCard)
  
}

*/

pub trait NetworkCard {

  fn put_frame(&mut self, &[u8]) -> IoResult<()>;
  
  fn address(&mut self) -> [u8,..6];
  
  // TODO(ryan): more
  
}
