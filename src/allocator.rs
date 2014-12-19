use core::prelude::*;
use panic::{print, println};

static mut allocator: GoodEnoughForNow = GoodEnoughForNow {current: 0 as *mut u8, size: 0};

pub fn set_allocator(start: *mut u8, stop: *mut u8) {
  unsafe {
    allocator = GoodEnoughForNow::new(start, (stop as uint) - (start as uint));
  }
}

pub trait Allocator {

  fn allocate(&mut self, size: uint) -> Option<*mut u8>; //TODO(ryan) uint big enough to hold pointer?
  
  fn free(&mut self, ptr: *mut u8);
 
  fn debug(&mut self) -> (*mut u8, uint);
 
}

struct GoodEnoughForNow {
  current: *mut u8,
  size: uint
}

impl GoodEnoughForNow {

  fn new(start: *mut u8, size: uint) -> GoodEnoughForNow {
    return GoodEnoughForNow {current: start, size: size}; 
  }
  
}

impl Allocator for GoodEnoughForNow {

  fn allocate(&mut self, size: uint) -> Option<*mut u8> {
    if size >= self.size { //TODO(ryan) overflow
      //loop{}
      print("no mem left :(");
      None
    } else {
      let ptr = self.current;
      self.current = ((self.current as uint) + size) as *mut u8;
      Some(ptr)
    }
  }
  
  fn free(&mut self, _: *mut u8) {
  }
  
  
  fn debug(&mut self) -> (*mut u8, uint) {
    (self.current, self.size)
  }

  
}

pub fn malloc(size: uint) -> *mut u8 {
  unsafe {
    match allocator.allocate(size) {
    Some(ptr) => ptr,
    _ => 0 as *mut u8
    }
  }
}

pub fn free(ptr: *mut u8) {
  unsafe {
    allocator.free(ptr)
  }
}

extern "C" {
  fn memmove(dest: *mut u8, src: *mut u8, count: int);
}

pub fn realloc(old: *mut u8, size: uint) -> *mut u8 {
  let new = malloc(size);
  unsafe { memmove(new, old, size as int); } //TODO(ryan): size may be too large
  new
}
