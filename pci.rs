#[macro_escape]

use std::io::{Stream, IoResult, IoError};
use std::mem::transmute;
use panic::{println, put_int};

pub struct Pci<'a> {
  address_port: Box<Stream + 'a>,
  data_port: Box<Stream + 'a>
}

impl<'a> Pci<'a> {

  pub fn new<'a>(address_port: Box<Stream>, data_port: Box<Stream>) -> Pci<'a> {
    Pci { address_port: address_port, data_port: data_port }
  }
  
  pub fn init(&mut self) {}
  
  pub fn read(&mut self, bus: u8, device: u8, function: u8, offset: u8) -> IoResult<u32> {
    if (function & 0x03 != 0x00) || (device >= 0x1 << 5) || (function >= 0x1 << 3)  {
      Ok(0 as u32)
    } else {
      let address: u32 = (0x1 as u32 << 31) | (bus as u32 << 16) | (device as u32 << 11) | (function as u32 << 8) | offset as u32;
      self.address_port.write_le_u32(address);
      self.address_port.read_le_u32()
    }
  }
  
  pub fn check_devices(&mut self) -> (u32, u32) {
    let mut no_device_count = 0;
    for bus in range(0, 256u) {
      for device in range(0, 32u) {
	let (vendor, device_id): (u16, u16) = unsafe { transmute(self.read(bus as u8, device as u8, 0, 0).unwrap()) };
	if vendor == 0xffff {
	  no_device_count += 1;
	}
	put_int(device_id as u32); println(" <-dev");
      }
    }
    debug!("debugging :)");
    (no_device_count, 256*32 - no_device_count)
  }

}
