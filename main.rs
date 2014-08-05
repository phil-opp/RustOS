
#![feature(phase)]
#![feature(lang_items)]
#![allow(ctypes)]
#![feature(intrinsics)]
#![feature(globs)]
#![feature(asm)]

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
use std::fmt;
use arch::keyboard::Keyboard;
use std::ty::Unsafe;
use terminal::TERMINAL;

pub mod arch;
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
  println("    in an interrupt!");
}

#[no_mangle]
pub extern "C" fn callback_i(u: u32) {
  print("    got interrupt number: "); put_int(u); println("");
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

fn put_char(c: u8) {
  unsafe { TERMINAL.put_char(c);}
}

#[no_mangle]
pub extern "C" fn main(magic: u32, info: *mut multiboot_info) {
  panic::init();
  unsafe {
    set_allocator((0x100000u32*12) as *mut u8, 0x1c9c380 as *mut u8);
    test_allocator();
    
    if magic != multiboot::MULTIBOOT_BOOTLOADER_MAGIC {
      panic::panic_message("Multiboot magic is invalid");
    } else {
      println("Multiboot magic is valid");
      put_int(info as u32);
      (*info).multiboot_stuff();
    }

    let cpu = cpu::CPU::current(); //&mut cpu::CPU::new();
    (*cpu).make_keyboard(put_char);
  
    (*cpu).enable_interrupts();
    
    println("Going to interrupt: ");
    (*cpu).test_interrupt();
    println("    back from interrupt!");
    
    let t2: &mut Writer = transmute(&panic::TERMINAL as &Writer);

    t2.write("Hello world from writer\n".as_bytes());
    t2.write(concat!("con", "cat\n").as_bytes());
    
    //println("start scheduling?");
    //scheduler::thread_stuff(); // <-- currently broken :(
    
    println("Kernel is done!");
    
    loop { }
  }
}

#[no_mangle]
pub extern "C" fn malloc(size: uint) -> *mut u8 {
    allocator::malloc(size)
}

#[no_mangle]
pub extern "C" fn free(ptr: *mut u8) {
  allocator::free(ptr)
}

#[no_mangle]
pub extern "C" fn realloc(ptr: *mut u8, size: uint) -> *mut u8 {
  allocator::realloc(ptr, size)
}

#[no_mangle]
pub extern "C" fn debug(s: &'static str, u: u32) {
  print(s); put_int(u); println("");
}

#[no_mangle]
pub extern "C" fn __morestack() {
  loop { } //TODO(ryan) should I do anything here?
}
