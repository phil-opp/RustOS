// An attempt at an rtl8139 ethernet card driver
use std::io::{Writer, IoResult, IoError};
use arch::cpu::Port;
use std::raw::Slice;
use std::mem::transmute;

pub struct Rtl8139 {
  command_register: Port, // TODO(ryan): better abstraction for registers (i.e., should take byte-width into consideration + also be for mmap)
  transmit_address: Port,
  transmit_status: Port,
  config_1: Port
}

impl Rtl8139 { // TODO(ryan): is there already a frame oriented interface in std libs to implement?

  pub fn new(io_offset: u16) -> Rtl8139 {
    
    Rtl8139 { config_1: Port::new(io_offset + 0x52),
	      command_register: Port::new(io_offset + 0x37),
	      transmit_address: Port::new(io_offset + 0x20),
	      transmit_status: Port::new(io_offset + 0x10)
	      }
  }

  pub fn init(&mut self) {
    self.config_1.out_b(0x00);

    self.command_register.out_b(0x10); // reset
    while (self.command_register.in_b() & 0x10) != 0 { } // wait till back

    
    self.command_register.out_b(0x0C); // enable transmit
    while (self.command_register.in_b() & 0x0c != 0x0c) {}
    
  }
  
  pub fn put_frame(&mut self, bytes: &[u8]) -> bool {
    let slice_bytes: Slice<u8> = unsafe { transmute(bytes) };

    debug!("sending {} bytes", slice_bytes.len)
    

    self.transmit_address.out_l(slice_bytes.data as u32);

    debug!("transmit_status before send is 0x{:x}", self.transmit_status.in_l())
    self.transmit_status.out_l(0xfff & (slice_bytes.len as u32));
    debug!("transmit_status after send is 0x{:x}", self.transmit_status.in_l())
    
    while (self.transmit_status.in_l() & 0x8000 == 0) { }
    
    true
  }

}
