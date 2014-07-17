extern crate core;
use core::mem::transmute;

static STACK_SIZE: uint = 1024*100; //TODO(ryan) better way for this?

pub struct Thread {
  stack_pointer: *u8,
  instruction_pointer: *u8
}

extern "C" {
  
  fn stack_pointer() -> *u8;
  
  fn instruction_pointer() -> *u8;
  
  fn set_stack_pointer_and_jump(stack_pointer: *u8, instruction_pointer: *u8);
  
}

impl Thread {

  pub fn new(func: extern "C" fn() -> (), mem: *u8) -> Thread {
    unsafe {
      Thread { stack_pointer: mem, instruction_pointer: transmute(func) }
    }
  }
  
  pub fn current_state() -> Thread {
    unsafe {
      Thread {stack_pointer: stack_pointer(), instruction_pointer: instruction_pointer()}
    }
  }
  
  pub unsafe fn resume(&self) {
    set_stack_pointer_and_jump(self.stack_pointer, self.instruction_pointer);
  }
  
}
