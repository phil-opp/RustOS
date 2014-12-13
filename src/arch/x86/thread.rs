use std::mem::{transmute, size_of};
use std::raw::Slice;

use panic::*;

#[repr(packed)]
pub struct Thread {
  regs: Registers,
  esp: u32,
  instruction_pointer: *mut u8,
  stack: Box<[u8]>
}

#[repr(packed)]
struct Registers {
    eax: u32, ebx: u32, ecx: u32, edx: u32,
    ebp: u32, esi: u32, edi: u32
}

extern "C" {
  
  pub fn save_context(t: &mut Thread) -> bool;
  
  pub fn restore_context(t: &Thread);
  
}

impl Thread {

  fn empty_regs() -> Registers {
    Registers { eax: 1, ebx: 2, ecx: 3, edx: 4, ebp: 0xffffffff, esi: 6, edi: 7}
  }
  
  pub fn empty() -> Thread {
    unsafe {
        Thread { stack: transmute::<u64, Box<[u8]>>(0_u64), instruction_pointer: transmute(0u), regs: Thread::empty_regs(), esp: 0}
    }
  }

  pub fn new(func: extern "C" fn() -> (), stack: Box<[u8]>, esp: uint) -> Thread {
    unsafe {
      //let ref s = &stack; // TODO(ryan): having trouble extracting esp from the box ...
      //let sli: &Slice<u8> = transmute(s);
      //let esp = sli.data as u32;
      let mut t = Thread::empty();
      save_context(&mut t);
      t.esp = esp as u32;
      t.stack = stack;
      t.instruction_pointer = transmute(func);
      //r.ebp = esp;
      //let t = Thread { stack: stack, instruction_pointer: transmute(func), regs: r, esp: esp};
      debug!("new thread:");
      t.debug();
      t
    }
  }
  
  pub fn debug(&self) {
    debug!("   self is 0x{:x}", transmute::<&Thread, u32>(self));
    debug!("   eip is 0x{:x}", self.instruction_pointer as u32);
    debug!("   esp is 0x{:x}", self.esp as u32);
  }
   
}
