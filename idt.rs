#![feature(intrinsics)]
extern crate core;

#[packed]
struct IDTEntry {
  offset_lower: u16, // offset bits 0..15
  selector: u16, // a code segment selector in GDT or LDT
  zero: u8,      // unused, set to 0
  type_attr: u8, // type and attributes, see below
  offset_upper: u16 // offset bits 16..31
}

extern "rust-intrinsic" {
    fn transmute<T, U>(x: T) -> U;

    fn offset<T>(dst: *T, offset: int) -> *T;
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

  pub fn new(mem: u32, size: u16) -> IDT {
    IDT {limit: size * 8, base: mem + 6 }
  }
  
  pub fn add_entry(&mut self, index: u32, f: unsafe extern "C" fn() -> ()) {
    assert(index < self.limit as u32);
    unsafe {
      let start: *IDTEntry = transmute(self.base);
      let e: *mut IDTEntry = transmute(offset(start, index as int)); 
      *e = IDTEntry::new(f);
    }
  }
  
  pub fn enable(&mut self) {
    unsafe {
      lidt(self);
      enable_interrupts();
    }
  }
 
}