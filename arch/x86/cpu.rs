use arch::idt::IDT;
use arch::gdt::GDT;

extern "C" {
  
  fn test();
  
  fn interrupt();
  
}

pub struct CPU {
  gdt: GDT,
  idt: IDT
  //ports: Ports
}

impl CPU {

  pub fn new() -> CPU {
    let mut gdt = GDT::new();
    gdt.identity_map();
    gdt.enable();
    
    //PIC::master().remap_to(0x20);
    //PIC::slave().remap_to(0x28);
  
    let mut idt = IDT::new();
    
    let mut i: u32 = 0;
    while i < idt.len() as u32 {
      idt.add_entry(i, test);
      i += 1;
    }
    
    idt.enable();
    
    CPU { gdt: gdt, idt: idt }
  }
  
  pub unsafe fn enable_interrupts(&mut self) {
    IDT::enable_interrupts();
  }
  
  pub fn disable_interrupts(&mut self) {
    IDT::disable_interrupts();
  }
  
  pub unsafe fn test_interrupt(&mut self) {
    interrupt();
  }
  
}
/*
struct PIC {
  controlPort: Port,
  maskPort: Port,
  is_master: bool
}

impl PIC {

  fn master() -> PIC {
    PIC { controlPort: Port::new(0x20), maskPort: Port::new(0x21), is_master: true}
  }

  fn slave() -> PIC {
    PIC { controlPort: Port::new(0xA0), maskPort: Port::new(0xA1), is_master: false}
  }
  
  unsafe fn remap_to(&mut self, start: u8) {
    let ICW1 = 0x11;
    let ICW4 = 0x1;
    let enable_all = 0x00;
    let typ = if self.is_master { 0x2 } else { 0x4 };
    
    controlPort.write_u8(ICIW);
    maskPort.write(&[start, typ, ICW4, enable_all]);
  }

}

struct Port {
  port_number: u16
}

impl Stream for Port {
  
  fn new(number: u16) -> Port {
    Port { port_number: number }
  }
  
  fn in_b(&mut self) -> u8 {
    let mut ret: u8;
    asm!("inb $0, $1" 
	:"=r"(ret) 
	:"r"(self.port)
	:
	:)
    return ret;
  }
  
  fn out_b(&mut self, byte: u8) {
    asm!("out_b $0, $1" 
	: 
	:"r"(byte), "r"(self.port)
	:
	:)
  }
  
  fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
    for &i in range(0, buf.len()) {
      buf[i] = self.in_b();
    }
    Ok(buf.len())
  }
  
  fn write(&mut self, buf: &[u8]) -> IoResult<()> {
    for &byte in buf.iter() {
      self.out_b(byte);
    }
    Ok(())
  }
  
}
*/