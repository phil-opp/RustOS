// An attempt at an rtl8139 ethernet card driver
use std::io::IoResult;
use arch::cpu::Port;
use std::raw::Slice;
use std::mem::transmute;

pub struct Rtl8139 {
  command_register: Port, // TODO(ryan): better abstraction for registers (i.e., should take byte-width into consideration + also be for mmap)
  transmit_address: [Port,..4],
  transmit_status: [Port,..4],
  config_1: Port,
  descriptor: uint
}

impl Rtl8139 { // TODO(ryan): is there already a frame oriented interface in std libs to implement?

  pub fn new(io_offset: u16) -> Rtl8139 {
    
    let p = |off: u16| -> Port {
      Port::new(io_offset + off)
    };
    
    Rtl8139 { config_1: p(0x52),
	      command_register: p(0x37),
	      transmit_address: [p(0x20), p(0x24), p(0x28), p(0x2c)],
	      transmit_status: [p(0x10), p(0x14), p(0x18), p(0x1c)],
	      descriptor: 0
	      }
  }

  pub fn init(&mut self) {
    self.config_1.out_b(0x00);

    self.command_register.out_b(0x10); // reset
    while (self.command_register.in_b() & 0x10) != 0 { } // wait till back

    
    self.command_register.out_b(0x0C); // enable transmit
    while (self.command_register.in_b() & 0x0c != 0x0c) {}
    
  }
  
  pub fn put_frame(&mut self, bytes: &[u8]) -> IoResult<()> {
    let slice_bytes: Slice<u8> = unsafe { transmute(bytes) };

    debug!("sending {} bytes", slice_bytes.len)
    

    self.transmit_address[self.descriptor].out_l(slice_bytes.data as u32);

    self.transmit_status[self.descriptor].out_l(0xfff & (slice_bytes.len as u32));
    
    while (self.transmit_status[self.descriptor].in_l() & 0x8000 == 0) { } // TODO(ryan): this is fragile if error sending...
    self.descriptor = (self.descriptor + 1) % 4;
    Ok(())
  }

}
