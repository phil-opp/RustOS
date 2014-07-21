#![feature(phase)]

#![allow(ctypes)]
#![feature(intrinsics)]
#![feature(globs)]

#[phase(plugin)]
extern crate lazy_static;

use std::vec;

use multiboot::multiboot_info;
use allocator::set_allocator;
use allocator::get_allocator;
use panic::print;
use terminal::Terminal;
use arch::vga;
use panic::{print, println, put_int};

mod arch;
mod idt;
mod terminal;
mod panic;
mod multiboot;
mod gdt;
mod allocator;
mod scheduler;

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
    //print("its an interrupt!");
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
  
  println("in vstuff");
  v.push("hello from a vector!");
  println("pushed workded");
  match v.pop() {
    Some(string) => println(string),
    None => println("uh oh!")
  }

}

#[no_mangle]
pub extern "C" fn abort() -> ! {
  unsafe {
    panic::abort();
  }
}

#[no_mangle]
pub extern "C" fn main(magic: u32, info: *multiboot_info) {
  
  panic::init();
  unsafe {
    println("hiiii");
    println("bye");
    
    if magic != multiboot::MULTIBOOT_BOOTLOADER_MAGIC {
      println("no good!");
    } else {
      println("valid header!");
      put_int(info as u32);
      (*info).multiboot_stuff();
    }
    
    
    let mut gdt = ::gdt::GDT::new(0x100000*11, 0x18);
    identity_map(gdt);
    gdt.enable();

    set_allocator((0x100000*12) as *u8, 0x1c9c380 as *u8);
    
    vstuff();
    
    
    println("");
    
    let mut idt = ::idt::IDT::new(0x100000*10, 0x400);
    let mut i = 0;
    
    while i < 0x400 {
      idt.add_entry(i, test);
      i += 1;
    }
    
    idt.enable();
    interrupt();
    println("and, we're back");
    
    println("start scheduling?");
    scheduler::thread_stuff();
    println("kernel is done!");
    
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

#[no_mangle]
pub extern "C" fn __morestack() {
  loop { } //TODO(ryan) should I do anything here?
}
