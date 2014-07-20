use core::mem::transmute;
use core::prelude::*;
use allocator::malloc;
use panic::*;

pub struct Thread {
  stack_pointer: *mut u8,
  base_pointer: *u8,
  instruction_pointer: *u8
}

extern "C" {
  
  fn stack_pointer() -> *mut u8;
  
  fn instruction_pointer() -> *u8;
  
  fn base_pointer() -> *u8;
  
  fn set_pointers_and_jump(stack_pointer: *mut u8, base_pointer: *u8, instruction_pointer: *u8);
    
}

impl Thread {

  pub fn new(func: extern "C" fn() -> (), mem: *u8) -> Thread {
    unsafe {
      Thread { stack_pointer: transmute(mem), instruction_pointer: transmute(func), base_pointer: mem }
    }
  }
  
  pub unsafe fn resume(&self) {
    set_pointers_and_jump(self.stack_pointer, self.base_pointer, self.instruction_pointer);
  }
  
  /*
  If called normally, return the current ThreadState
  If resumed via the resume method, return None
  */
  pub fn current_state_or_resumed() -> Option<Thread> {
    current_state_or_resumed_impl()
  }
  
  pub fn debug(&self) {
    print("eip is "); put_int(self.instruction_pointer as u32); println("");
    wait();
  }
   
}

fn wait() -> u64 {
  let mut i: u64 = 0;
  let mut sum: u64 = 0;
  while i < 0x8ffffff {
    i += 1;
    sum += i*2*i - 4;
    if (i * 213423) as u32 & 0xff == 1 {
      i += 2;
    }
  }
  sum
}

extern "C" fn current_state_or_resumed_impl() -> Option<Thread> {
  unsafe {
      let ebp = base_pointer();
      let esp = stack_pointer();
      let eip = instruction_pointer();
      
      if eip as u32 == 0 {
	return None
      }
      
      let thread = Thread {stack_pointer: esp, base_pointer: ebp, instruction_pointer: eip};
      
      Some(thread)
      
    }
}
 
