use core::prelude::*;

use alloc::boxed::Box;

use collections::Vec;

pub trait Driver {

  fn init(&mut self);

}

pub trait DriverManager {

  fn get_drivers(&mut self) -> Vec<Box<NetworkDriver>>;

}

pub trait NetworkDriver: Driver {

  fn put_frame(&mut self, &[u8]) -> Result<(), ()>;
  
  fn address(&mut self) -> [u8,..6];
  
  // TODO(ryan): more
  
}
