use arch::idt::IDT;
use arch::gdt::GDT;

extern "C" {
  
  fn test();
  
  fn interrupt();
  
}

pub struct CPU {
  gdt: GDT,
  idt: IDT
}

impl CPU {

  pub fn new() -> CPU {
    let mut gdt = GDT::new();
    gdt.identity_map();
    gdt.enable();
    
    let mut idt = IDT::new();
    
    let mut i: u32 = 0;
    while i < idt.len() as u32 {
      idt.add_entry(i, test);
      i += 1;
    }
    
    idt.enable();
    
    CPU { gdt: gdt, idt: idt }
  }
  
  pub unsafe fn enable_interrupts(&self) {
    IDT::enable_interrupts();
  }
  
  pub fn disable_interrupts(&self) {
    IDT::disable_interrupts();
  }
  
  pub unsafe fn test_interrupt(&self) {
    interrupt();
  }

}
