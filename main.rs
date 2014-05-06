#![no_std]
#![allow(ctypes)]
extern crate core;
use core::ptr;
use multiboot::multiboot_info;
mod idt;
mod vga;
mod multiboot;
mod gdt;

extern {
  
  fn abort() -> !;
  
  fn test();
  
  fn interrupt();
  
}

extern "rust-intrinsic" {
  pub fn transmute<T, U>(x: T) -> U;
}

pub fn panic() {
  unsafe {
    ::vga::TERMINAL.println("panic!");
  }
}

#[no_mangle]
pub extern "C" fn callback() {
  unsafe {
    ::vga::TERMINAL.print("its an interrupt!");
  }
}

fn float_to_int(x: f32) -> u32 {
  unsafe {
    let i: *u32 = transmute(&x);
    *i
  }
}

fn identity_map(mut gdt: gdt::GDT) {
  gdt.add_entry(0, 0, 0);                     // Selector 0x00 cannot be used
  gdt.add_entry(0, 0xffffffff, 0x9A);         // Selector 0x08 will be our code
  gdt.add_entry(0, 0xffffffff, 0x92);         // Selector 0x10 will be our data
  //gdt.add_entry( = {.base=&myTss, .limit=sizeof(myTss), .type=0x89}; // You can use LTR(0x18)
}

#[no_mangle]
pub extern "C" fn main(magic: u32, info: *multiboot_info) {
  unsafe {
    vga::clear_screen(::vga::Black);
    vga::TERMINAL = vga::Terminal::new(0xb8000, 160, 24);
    
    if magic != multiboot::MULTIBOOT_BOOTLOADER_MAGIC {
      vga::TERMINAL.println("no good!");
    } else {
      vga::TERMINAL.println("valid header!");
      vga::TERMINAL.put_int(info as u32);
      (*info).multiboot_stuff();
    }
    
    let mut gdt = ::gdt::GDT::new(0x100000*11, 0x18);
    identity_map(gdt);
    gdt.enable();
    
    vga::TERMINAL.println("");
    let mut idt = ::idt::IDT::new(0x100000*10, 0x400);
    let mut i = 0;
    
    while i < 0x400 {
      idt.add_entry(i, test);
      i += 1;
      
    }
    
    idt.enable();
    
    interrupt();
    vga::TERMINAL.println("and, we're back");
    loop { }
  }
}
