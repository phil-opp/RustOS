use std::mem;
use panic::*;
use std::ptr::RawPtr;
use std::mem::{transmute, size_of};

static IDT_SIZE: uint = 256;

#[packed]
struct IDTEntry {
  offset_lower: u16, // offset bits 0..15
  selector: u16, // a code segment selector in GDT or LDT
  zero: u8,      // unused, set to 0
  type_attr: u8, // type and attributes, see below
  offset_upper: u16 // offset bits 16..31
}

impl IDTEntry {
  
  fn new(f: unsafe extern "C" fn() -> ()) -> IDTEntry {
    unsafe {
      let (lower, upper): (u16, u16) = transmute(f);
      IDTEntry { offset_lower: lower, selector: 0x08, zero: 0, type_attr: 0x8E, offset_upper: upper }
    }
  }
  
  fn no_op() -> IDTEntry {
    IDTEntry::new(test)
  }
  
}

extern "C" {
  fn no_op() -> ();
  
  fn test() -> ();
  
  fn register_all_callbacks(idt: &mut IDT);
  
  fn callback_0();
  
  fn debug(s: &str, u: u32) -> ();
}

#[packed]
struct IDTLayout {
  limit: u16,
  base: u32
}

// TODO(ryan) this should be allocated for two reasons:
// 1. It's really big (256 * 8 == 2 KB)
// 2. Its location in memory needs to stay the same
pub struct IDT {
  table: Vec<IDTEntry> 
}

impl IDT {

  pub fn new() -> IDT {
    let mut me = IDT { table: Vec::from_fn(IDT_SIZE, |_| IDTEntry::no_op() ) };
    /*unsafe { 
      raw = 
      register_all_callbacks(&mut me.table.); 
      
    }*/
    me
  }
  
  pub fn add_entry(&mut self, index: u32, f: unsafe extern "C" fn() -> ()) {
    *self.table.get_mut(index as uint) =  IDTEntry::new(f);
  }
  
  pub unsafe fn enable(&mut self) {
    let (base, _): (u32, u32) = transmute(self.table.as_slice());
    let limit: u16 = (self.table.len() * size_of::<IDTEntry>()) as u16;
    let layout = IDTLayout { base: base, limit: limit};
    asm!("lidt ($0)"
	:
	:"{eax}"(&layout)
	:
	:
	:"volatile"); 
  }
  
  pub fn disable_interrupts() {
    unsafe { asm!("cli" :::: "volatile"); }
  }
  
  pub unsafe fn enable_interrupts() {
    asm!("sti" :::: "volatile");
  }
  
  pub fn len(&self) -> uint {
    return self.table.len();
  }
 
}
