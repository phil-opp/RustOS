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
use panic::print;
use terminal::Terminal;
use arch::vga;

mod arch;
mod idt;
mod terminal;
mod panic;
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


#[no_mangle]
pub extern "C" fn callback() {
  unsafe {
    print("its an interrupt!");
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

unsafe fn vstuff(mut terminal: Terminal) {
  let mut v = vec::Vec::new();
  //loop{};
  
  terminal.println("in vstuff");
  v.push("hello from a vector!");
  terminal.println("pushed workded");
  match v.pop() {
    Some(string) => terminal.println(string),
    None => terminal.println("uh oh!")
  }

}

#[no_mangle]
pub extern "C" fn abort() {
  unsafe {
    panic::abort();
  }
}


#[no_mangle]
pub extern "C" fn main(magic: u32, info: *multiboot_info) {
  go(magic, info, Terminal::new(vga::VGA::new()));
}
  
fn go(magic: u32, info: *multiboot_info, mut terminal: Terminal) {
  unsafe {
    terminal.clear_screen();
        
    if magic != multiboot::MULTIBOOT_BOOTLOADER_MAGIC {
      terminal.println("no good!");
    } else {
      terminal.println("valid header!");
      terminal.put_int(info as u32);
      //(*info).multiboot_stuff();
    }
    
    let mut gdt = ::gdt::GDT::new(0x100000*11, 0x18);
    identity_map(gdt);
    gdt.enable();

    set_allocator((0x100000*12) as *u8, 0x1c9c380 as *u8);
    
    let alloc = get_allocator();
    
    let (_, size) = alloc.debug();
    terminal.print("size of allocator is: ");
    terminal.put_int(size as u32);
    terminal.println("");
    match alloc.allocate(10) {
      Some(_) => terminal.print("got mem\n"),
      None => terminal.print("allocator failed")
    }
    
    
    match alloc.allocate(10) {
      Some(mem) => terminal.put_int(mem as u32),
      None => terminal.println("allocator failed")
    }
        terminal.println("");

    terminal.put_int(realloc(1 as *u8, 10) as u32);
    terminal.put_int(allocator::realloc(1 as *u8, 10) as u32);
    
    
    vstuff(terminal);
    
    
    terminal.println("");
    
    let mut idt = ::idt::IDT::new(0x100000*10, 0x400);
    let mut i = 0;
    
    while i < 0x400 {
      idt.add_entry(i, test);
      i += 1;
      
    }
    
    
    idt.enable();
    
    interrupt();
    
    terminal.println("and, we're back");
    
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
