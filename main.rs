
#![feature(phase)]
#![feature(lang_items)]
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
use arch::cpu;
use panic::{print, println, put_int};

mod arch;
mod terminal;
mod panic;
mod multiboot;
mod allocator;
mod scheduler;


extern "rust-intrinsic" {
  pub fn transmute<T, U>(x: T) -> U;
}


#[no_mangle]
pub extern "C" fn callback() {
  //println("    in an interrupt!");
}

fn float_to_int(x: f32) -> u32 {
  unsafe {
    let i: *u32 = transmute(&x);
    *i
  }
}


fn test_allocator() {
  let mut v = vec::Vec::new();
  
  println("Testing allocator with a vector push");
  v.push("   hello from a vector!");
  println("   push didn't crash");
  match v.pop() {
    Some(string) => println(string),
    None => println("    push was weird...")
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
    set_allocator((0x100000*12) as *u8, 0x1c9c380 as *u8);
    test_allocator();
    
    let mut cpu = cpu::CPU::new();
    
    if magic != multiboot::MULTIBOOT_BOOTLOADER_MAGIC {
      panic::panic_message("Multiboot magic is invalid");
    } else {
      println("Multiboot magic is valid");
      put_int(info as u32);
      (*info).multiboot_stuff();
    }
        
    cpu.enable_interrupts();
    println("Going to interrupt: ");
    cpu.test_interrupt();
    println("    back from interrupt!");
    
    let t2: &mut Writer = transmute(&panic::TERMINAL as &Writer);
    t2.write("Hello world from writer\n".as_bytes());
    
    //println("start scheduling?");
    //scheduler::thread_stuff(); // <-- currently broken :(
    
    println("Kernel is done!");
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
