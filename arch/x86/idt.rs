use std::mem;
use panic::*;
use std::ptr::RawPtr;
use std::mem::transmute;

#[packed]
struct IDTEntry {
  offset_lower: u16, // offset bits 0..15
  selector: u16, // a code segment selector in GDT or LDT
  zero: u8,      // unused, set to 0
  type_attr: u8, // type and attributes, see below
  offset_upper: u16 // offset bits 16..31
}

extern "C" {

  fn disable_interrupts();

  fn lidt(ptr: *mut IDT);
  
  fn enable_interrupts();

}

impl IDTEntry {
  
  fn new(f: unsafe extern "C" fn() -> ()) -> IDTEntry {
    unsafe {
      let (lower, upper): (u16, u16) = transmute(f);
      IDTEntry { offset_lower: lower, selector: 0x08, zero: 0, type_attr: 0x8E, offset_upper: upper }
    }
  }
  
}

#[packed]
pub struct IDT {
  limit: u16,
  base: u32
}

fn assert(b: bool) {
  if !b {
    // TODO(ryan) implement
  }
}

impl IDT {

  pub fn new() -> IDT {
    unsafe { 
      let mem: Vec<u8> = Vec::with_capacity(0x399 * 9);
      let (raw, len): (u32, u32) = transmute(mem.as_slice()); 
      IDT {limit: 0x399*9 as u16, base: raw + 6 } 
    }
  }
  
  pub fn add_entry(&mut self, index: u32, f: unsafe extern "C" fn() -> ()) {
    assert(index < self.limit as u32);
    unsafe {
      let start: *mut IDTEntry = transmute(self.base);
      let e: *mut IDTEntry = transmute(start.offset(index as int)); 
      *e = IDTEntry::new(f);
    }
  }
  
  pub fn enable(&mut self) {
    unsafe {
      lidt(self);
    }
  }
  
  pub fn disable_interrupts() {
    unsafe { disable_interrupts(); }
  }
  
  pub unsafe fn enable_interrupts() {
    enable_interrupts();
  }
  
  pub fn len(&self) -> uint {
    return (self.limit / 8) as uint;
  }
 
}