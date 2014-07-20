#![no_std]
extern crate core;
use core::option::Option;
use core::option::None;
use core::option::Some;
use panic::{print, println};

static mut allocator: GoodEnoughForNow = GoodEnoughForNow {current: 0 as *u8, size: 0};

pub fn get_allocator() -> &mut Allocator {
  unsafe {
    &mut allocator as &mut Allocator
  }
}

pub fn set_allocator(start: *u8, stop: *u8) {
  unsafe {
    allocator = GoodEnoughForNow::new(start, (stop as uint) - (start as uint));
  }
}

pub trait Allocator {

  fn allocate(&mut self, size: uint) -> Option<*u8>; //TODO(ryan) uint big enough to hold pointer?
  
  fn free(&mut self, ptr: *u8);
 
  fn debug(&mut self) -> (*u8, uint);
 
}

struct GoodEnoughForNow {
  current: *u8,
  size: uint
}

impl GoodEnoughForNow {

  fn new(start: *u8, size: uint) -> GoodEnoughForNow {
    return GoodEnoughForNow {current: start, size: size}; 
  }
  
}

impl Allocator for GoodEnoughForNow {

  fn allocate(&mut self, size: uint) -> Option<*u8> {
    if size >= self.size { //TODO(ryan) overflow
      //loop{}
      unsafe {print("no mem left :("); }
      None
    } else {
      let ptr = self.current;
      self.current = ((self.current as uint) + size) as *u8;
      Some(ptr)
    }
  }
  
  fn free(&mut self, ptr: *u8) {
  }
  
  
  fn debug(&mut self) -> (*u8, uint) {
    (self.current, self.size)
  }

  
}

pub fn malloc(size: uint) -> *u8 {
  unsafe {
    match allocator.allocate(size) {
    Some(ptr) => ptr,
    _ => 0 as *u8
    }
  }
}

pub fn free(ptr: *u8) {
  unsafe {
    allocator.free(ptr)
  }
}

extern "C" {
  fn memmove(dest: *u8, src: *u8, count: int);
}

pub fn realloc(old: *u8, size: uint) -> *u8 {
  let new = malloc(size);
  unsafe { memmove(new, old, size as int); } //TODO(ryan): size may be too large
  new
}
