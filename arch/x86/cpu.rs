use arch::idt::IDT;
use arch::gdt::GDT;

use std::io::{Stream, IoResult};
use arch::keyboard::Keyboard;

use std::one::{ONCE_INIT, Once};
use std::ty::Unsafe;

lazy_static! {
  static ref CURRENT_CPU: Unsafe<CPU> = Unsafe::new(CPU::new());
}

enum IRQ { // after remap
  Timer = 20,
  Keyboard = 21,
  Cascade = 22,
  COM2 = 23,
  COM1 = 24,
  LPT2 = 25,
  Floppy = 26,
  LPT1 = 27,
  CmosClock = 28,
  FreeOne = 29,
  FreeTwo = 30,
  FreeThree11 = 31,
  PsMouse = 32,
  FPU = 33,
  PrimaryAta = 34,
  SecondaryAta = 35
}

extern "C" {
  
  fn test(n: u32);
  
  fn interrupt();
  
}

pub struct CPU {
  gdt: GDT,
  idt: IDT,
  keyboard: Option<Keyboard>
  //ports: Ports
}

impl CPU {

  pub unsafe fn new() -> CPU {
    let mut gdt = GDT::new();
    gdt.identity_map();
    gdt.enable();
    
    PIC::master().remap_to(0x20);
    PIC::slave().remap_to(0x28);
  
    let mut idt = IDT::new();
    
    let mut i: u32 = 0;
    //while i < idt.len() as u32 {
    //  idt.add_entry(i, test);
    //  i += 1;
    //}
    idt.enable();
    CPU { gdt: gdt, idt: idt, keyboard: None}
  }
  
  pub fn current() -> CPU {
    unsafe { *CURRENT_CPU.get() }
  }
  
  pub fn handle(&mut self, interrupt_number: u32) {
    //test(interrupt_number);
    //loop {}
  }
  
  pub unsafe fn register_irq(&mut self, irq: IRQ, handler: extern "C" fn () -> ()) {
    self.idt.add_entry(irq as u32, handler);
  }
  /*
  pub fn make_keyboard(&mut self, callback: fn (u8) -> ()) {
    self.keyboard = Some(Keyboard::new(callback, Port {port_number: 0x64}, Port {port_number: 0x60}));
    self.register_irq(Keyboard, )
  }*/
  
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

#[no_mangle]
pub extern "C" fn unified_handler(interrupt_number: u32) {
  CPU::current().handle(interrupt_number);
}

#[no_mangle]
pub extern "C" fn add_entry(idt: &mut IDT, index: u32, f: unsafe extern "C" fn() -> ()) {
  idt.add_entry(index, f);
}


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
    
    self.controlPort.write_u8(ICW1);
    self.maskPort.write(&[start, typ, ICW4, enable_all]);
  }

}

pub struct Port {
  port_number: u16
}

impl Port {

  pub fn new(number: u16) -> Port {
    Port { port_number: number }
  }
    
  fn in_b(&mut self) -> u8 {
    let mut ret: u8;
    unsafe {
      asm!("movw $1, %dx
	    inb %dx, %al
	    movb %al, $0" 
	  :"=r"(ret) 
	  :"r"(self.port_number)
	  :
	  : "dx", "al")
    }
    return ret;
  }
  
  fn out_b(&mut self, byte: u8) {
    unsafe {
      asm!("movw $0, %dx
	    movb $1, %al
	    outb %al, %dx" 
	  : 
	  :"r"(self.port_number), "r"(byte)
	  : "dx", "al"
	  :)
    }
  }


}

impl Reader for Port {
  
  fn read_u8(&mut self) -> IoResult<u8> {
    Ok(self.in_b())
  }
  
  fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
    for &mut el in buf.iter() {
      el = self.in_b();
    }
    Ok(buf.len())
  }
  
}

impl Writer for Port {

  fn write_u8(&mut self, byte: u8) -> IoResult<()> {
    self.out_b(byte);
    Ok(())
  }
  
  fn write(&mut self, buf: &[u8]) -> IoResult<()> {
    for &byte in buf.iter() {
      self.out_b(byte);
    }
    Ok(())
  }

}