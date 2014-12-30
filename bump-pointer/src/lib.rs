#![feature(globs)]

#![no_std]

extern crate core;

use core::prelude::*;

extern "C" {
  fn memmove(dest: *mut u8, src: *mut u8, count: u32);
}


static mut allocator: BumpPointer = BumpPointer {
  start: 0 as *mut u8,
  stop:  0 as *mut u8,
};

pub fn set_allocator(start: *mut u8, stop: *mut u8) {
  unsafe {
    allocator = BumpPointer::new(start, stop);
  }
}

pub trait Allocator
{
  fn allocate(&mut self, size: uint, align: uint) -> Option<*mut u8>;

  fn deallocate(&mut self, ptr: *mut u8, old_size: uint, align: uint);

  fn reallocate(&mut self, ptr: *mut u8, old_size: uint, size: uint,
                align: uint) -> Option<*mut u8>
  {
    let attempt = self.allocate(size, align);
    if let Some(new) = attempt {
      unsafe { memmove(new, ptr, old_size as u32) };
      self.deallocate(ptr, old_size, align);
    }
    attempt
  }

  fn reallocate_inplace(&mut self, _ptr: *mut u8, old_size: uint, _size: uint,
                        _align: uint) -> uint
  {
    old_size
  }

  fn usable_size(&mut self, size: uint, _align: uint) -> uint
  {
    size
  }
  //fn stats_print(&mut self);

  fn debug(&mut self) -> (*mut u8, uint);
}

pub struct BumpPointer {
  start: *mut u8,
  stop:  *mut u8,
}

impl BumpPointer
{
  pub fn new(start: *mut u8, stop: *mut u8) -> BumpPointer {
    return BumpPointer { start: start, stop: stop };
  }
}

impl Allocator for BumpPointer
{
  #[inline]
  fn allocate(&mut self, size: uint, align: uint) -> Option<*mut u8>
  {
    let aligned: uint = {
      let a = self.start as uint + align - 1;
      a - (a % align)
    };
    let new_start = aligned + size;

    if new_start > self.stop as uint {
      None
    } else {
      self.start = new_start as *mut u8;
      Some(aligned as *mut u8)
    }
  }

  #[inline]
  fn deallocate(&mut self, _ptr: *mut u8, _old_size: uint, _align: uint) { }

  #[inline]
  fn debug(&mut self) -> (*mut u8, uint) {
    (self.start, self.stop as uint - self.start as uint)
  }
}

pub fn allocate(size: uint, align: uint) -> *mut u8 {
  unsafe {
    match allocator.allocate(size, align) {
    Some(ptr) => ptr,
    None      => 0 as *mut u8
    }
  }
}

pub fn deallocate(ptr: *mut u8, old_size: uint, align: uint) {
  unsafe {
    allocator.deallocate(ptr, old_size, align)
  }
}

pub fn reallocate(ptr: *mut u8, old_size: uint, size: uint,
                              align: uint) -> *mut u8 {
  unsafe {
    match allocator.reallocate(ptr, old_size, size, align) {
      Some(ptr) => ptr,
      None      => 0 as *mut u8
    }
  }
}

pub fn reallocate_inplace(ptr: *mut u8, old_size: uint, size: uint,
                                      align: uint) -> uint {
  unsafe {
    allocator.reallocate_inplace(ptr, old_size, size, align)
  }
}

pub fn usable_size(size: uint, align: uint) -> uint {
  unsafe {
    allocator.usable_size(size, align)
  }
}

pub fn stats_print() {}
