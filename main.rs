
#![feature(phase)]
#![feature(lang_items)]
#![allow(ctypes)]
#![feature(intrinsics)]
#![feature(globs)]
#![feature(asm)]
#![feature(macro_rules)]

#[phase(plugin)]
extern crate lazy_static;

use std::vec;
use std::string;
use std::str;

use multiboot::multiboot_info;
use allocator::set_allocator;
use arch::cpu;
use panic::{print, println};
use terminal::TERMINAL;
use pci::Pci;


macro_rules! debug( // TODO(ryan): ugly place for this, but want it accessible by the modules
    ($($arg:tt)*) => (
        unsafe {
          use terminal::TERMINAL;
	  TERMINAL.write(format!("[{}:{} DEBUG]:    ", file!(), line!()).as_bytes()).ok();
	  TERMINAL.write(format!($($arg)*).as_bytes()).ok();
	  TERMINAL.write("\n".as_bytes()).ok();
	}
    )
)

macro_rules! kassert(
  ($b: expr) => (
        if (!$b) {
	  debug!("assertion failed {}", stringify!(b))
	  loop {}
        }
    )
)

pub mod arch;
mod terminal;
mod panic;
mod multiboot;
mod allocator;
mod scheduler;
mod pci;

extern "rust-intrinsic" {
  pub fn transmute<T, U>(x: T) -> U;
}


#[no_mangle]
pub extern "C" fn callback() {
  println("    in an interrupt!");
}

#[no_mangle]
pub extern "C" fn callback_i(u: u32) {
  debug!("    got interrupt number: {}", u)
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

lazy_static! {
  static ref TEST: vec::Vec<&'static str> = {
    let mut v = vec::Vec::new();
    v.push("hi from lazy sttaic");
    v
  };
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
      debug!("Multiboot magic is valid. Info at 0x{:x}", info as u32);
      (*info).multiboot_stuff();
    }
    
    println((*TEST)[0]);
    let cpu = cpu::CPU::current();
    
    (*cpu).make_keyboard(put_char);
    
    (*cpu).enable_interrupts();
    println("Going to interrupt: ");
    (*cpu).test_interrupt();
    println("    back from interrupt!");
    
    let t2: &mut Writer = transmute(&panic::TERMINAL as &Writer);

    t2.write("Hello world from writer\n".as_bytes()).ok();
    t2.write(concat!("con", "cat\n").as_bytes()).ok();
    t2.write(format!("for{} {}\n", "mat", 2i).as_bytes()).ok();
    
    debug!("debugging :)");
    //println("start scheduling?");
    //scheduler::thread_stuff(); // <-- currently broken :(
    
    pci_stuff();
    
    println("Kernel is done!");
    
    loop {
      (*cpu).idle()
    }
  }
}

fn pci_stuff() {
  let address_port = cpu::Port::new(0xcf8);
  let data_port = cpu::Port::new(0xcfc);
  let mut pci = Pci::new(address_port, data_port);
  pci.init();
  let (not_found, found) = pci.check_devices();
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
  debug!("{} 0x{:x}", s, u)
}

#[no_mangle]
pub extern "C" fn __morestack() {
  loop { } //TODO(ryan) should I do anything here?
}
