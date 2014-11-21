use std::io::IoResult;
use pci::Pci;

pub trait Driver {

  fn init(&mut self);

}

pub trait DriverManager {

  fn get_drivers(&mut self) -> Vec<Box<NetworkDriver>>;

}

pub trait NetworkDriver: Driver {

  fn put_frame(&mut self, &[u8]) -> IoResult<()>;
  
  fn address(&mut self) -> [u8,..6];
  
  // TODO(ryan): more
  
}
