#![no_std]

#[packed]
struct IDTEntry {
  offset_lower: u16, // offset bits 0..15
  selector: u16, // a code segment selector in GDT or LDT
  zero: u16,      // unused, set to 0
  type_attr: u8, // type and attributes, see below
  offset_upper: u16 // offset bits 16..31
}

extern "rust-intrinsic" {
    fn transmute<T, U>(x: T) -> U;

    fn offset<T>(dst: *T, offset: int) -> *T;
}

extern "C" {
  fn disable_interrupts();
  fn lidt(ptr: *IDTPointer);
  fn enable_interrupts();
}

impl IDTEntry {
  
  fn new(f: extern "C" fn() -> ()) -> IDTEntry {
    unsafe {
      let (lower, upper): (u16, u16) = transmute(f);
      IDTEntry { offset_lower: lower, selector: 0x08, zero: 0, type_attr: 0x8E, offset_upper: upper }
    }
  }
  
}

#[packed]
struct IDTPointer {
  limit: u16,
  base: u32
}

#[packed]
pub struct IDT {
  pointer: *IDTPointer,
  table: *IDTEntry,
  size: u32
}

fn assert(b: bool) {
  if !b {
    //::main::panic();
  }
}

impl IDT {

  pub fn new(mem: *u8, size: u32) -> IDT {
    //assert(size - (2 + 4) %);
    unsafe {
      let ptr: *mut IDTPointer = transmute(mem);
      (*ptr).limit = 256;
      (*ptr).base = offset(mem, 6) as u32;
      IDT { pointer: ptr as *IDTPointer, table: (offset(mem, 6) as *IDTEntry), size: size }
    }
  }
  
  pub fn add_entry(&mut self, index: u32, f: extern "C" fn() -> ()) {
    assert(index < self.size);
    unsafe {
      let e: *mut IDTEntry = offset(self.table, index as int) as *mut IDTEntry; 
      *e = IDTEntry::new(f);
    }
  }
  
  pub fn enable(&mut self) {
    unsafe {
      lidt(self.pointer);
      enable_interrupts();
    }
  }
 

}