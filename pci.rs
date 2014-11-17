#[macro_escape]

use std::io::{Stream, IoResult, IoError};
use std::mem::transmute;
use arch::cpu::Port;

pub struct Pci {
  address_port: Port,
  data_port: Port
}

impl Pci {

  pub fn new<'a>(address_port: Port, data_port: Port) -> Pci {
    Pci { address_port: address_port, data_port: data_port }
  }
  
  pub fn init(&mut self) {}
  
  pub fn read(&mut self, bus: u8, device: u8, function: u8, offset: u8) -> IoResult<u32> {
    if (function & 0x03 != 0x00) || (device >= 0x1 << 5) || (function >= 0x1 << 3)  {
      Ok(1 as u32)
    } else {
      let address: u32 = (0x1 as u32 << 31) | (bus as u32 << 16) | (device as u32 << 11) | (function as u32 << 8) | offset as u32;
      self.address_port.out_l(address);
      Ok(self.address_port.in_l())
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
	debug!("vendor id: 0x{:x}", vendor);
      }
    }
    debug!("debugging :)");
    (no_device_count, 256*32 - no_device_count)
  }

}
