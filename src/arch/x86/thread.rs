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
  
  fn switch_and_save(old_thread: Box<Thread>, new_thread: &Thread, transferred_info: *const u8, save_to_new_thread: bool);
  
}

impl Thread {

  fn empty_regs() -> Registers {
    Registers { eax: 1, ebx: 2, ecx: 3, edx: 4, ebp: 5, esi: 6, edi: 7}
  }

  pub fn new(func: extern "C" fn() -> (), stack: Box<[u8]>, esp: u32) -> Thread {
    unsafe {
      //let ref s = &stack; // TODO(ryan): having trouble extracting esp from the box ...
      //let sli: &Slice<u8> = transmute(s);
      //let esp = sli.data as u32;
      let t = Thread { stack: stack, instruction_pointer: transmute(func), regs: Thread::empty_regs(), esp: esp};
      debug!("new thread:");
      t.debug();
      t
    }
  }
  
  pub unsafe fn switch_to(&mut self, passed_info: Option<&|old: Box<Thread>, new: &Thread| -> ()>) {
    let mut current: Box<Thread> = box Thread::new(unsafe {transmute(0_u32)} , box [0,..0], 0);
    debug!("switching to:")
    self.debug();
    debug!("current is:")
    current.debug();
    
    match passed_info {
      Some(info) => switch_and_save(current, self, unsafe { transmute(info) }, true),
      None => switch_and_save(current, self, unsafe { transmute(0_u32) }, false)
    }
    
  }
  
  pub fn debug(&self) {
    debug!("   self is 0x{:x}", transmute::<&Thread, u32>(self));
    debug!("   eip is 0x{:x}", self.instruction_pointer as u32);
    debug!("   esp is 0x{:x}", self.esp as u32);
  }
   
}
