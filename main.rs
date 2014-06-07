#![no_std]
#![allow(ctypes)]
extern crate core;
use core::ptr;
use core::vec;
use core::option::Option;
use core::option::None;
use core::option::Some;
use multiboot::multiboot_info;
use allocator::set_allocator;
use allocator::get_allocator;
mod idt;
mod vga;
mod multiboot;
mod gdt;
mod allocator;

extern {
  
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

unsafe fn vstuff() {
  let mut v = vec::Vec::new();
  //loop{};
  
  vga::TERMINAL.println("in vstuff");
  v.push("hello from a vector!");
  vga::TERMINAL.println("pushed workded");
  match v.pop() {
    Some(string) => vga::TERMINAL.println(string),
    None => vga::TERMINAL.println("uh oh!")
  }

}

#[no_mangle]
pub extern "C" fn abort() {
  unsafe {
    vga::TERMINAL.println("kernel panic! (from abort())");
  }
  loop {}
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
      //(*info).multiboot_stuff();
    }
    
    let mut gdt = ::gdt::GDT::new(0x100000*11, 0x18);
    identity_map(gdt);
    gdt.enable();

    set_allocator((0x100000*12) as *u8, 0x1c9c380 as *u8);
    
    let alloc = get_allocator();
    
    let (_, size) = alloc.debug();
    vga::TERMINAL.print("size of allocator is: ");
    vga::TERMINAL.put_int(size as u32);
    vga::TERMINAL.println("");
    match alloc.allocate(10) {
      Some(_) => vga::TERMINAL.print("got mem\n"),
      None => vga::TERMINAL.print("allocator failed")
    }
    
    
    match alloc.allocate(10) {
      Some(mem) => vga::TERMINAL.put_int(mem as u32),
      None => vga::TERMINAL.println("allocator failed")
    }
        vga::TERMINAL.println("");

    vga::TERMINAL.put_int(realloc(1 as *u8, 10) as u32);
    vga::TERMINAL.put_int(allocator::realloc(1 as *u8, 10) as u32);
    
    
    vstuff();
    
    
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

#[no_mangle]
pub extern "C" fn malloc(size: uint) -> *u8 {
    allocator::malloc(size)
}

#[no_mangle]
pub extern "C" fn free(ptr: *u8) {
  allocator::free(ptr)
}

#[no_mangle]
pub extern "C" fn realloc(ptr: *u8, size: uint) -> *u8 {
  allocator::realloc(ptr, size)
}
