
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
use terminal::TERMINAL;
use pci::Pci;
use driver::DriverManager;

macro_rules! print(
    ($($arg:tt)*) => (
        unsafe {
          use terminal::TERMINAL;
	  TERMINAL.write(format!($($arg)*).as_bytes()).ok();
	}
    )
)

macro_rules! log( // TODO(ryan): ugly place for this, but want it accessible by the modules
    ($lvl: expr, $($arg:tt)*) => (
        unsafe {
          use terminal::TERMINAL;
	  print!("[{}:{} {}]: ", $lvl, file!(), line!())
	  print!($($arg)*)
	  print!("\n")
	}
    )
)

macro_rules! debug( 
  ($($arg:tt)*) => (log!("DEBUG", $($arg)*))
)

macro_rules! warn( 
  ($($arg:tt)*) => (log!("WARN", $($arg)*))
)

macro_rules! info( 
  ($($arg:tt)*) => (log!("INFO", $($arg)*))
)

macro_rules! trace( 
  ($($arg:tt)*) => (log!("TRACE", $($arg)*))
)

macro_rules! kassert(
  ($b: expr) => (
        if (!$b) {
	  debug!("assertion failed {}", stringify!(b))
	  loop {}
        }
    )
)

macro_rules! kpanic(
  ($($arg:tt)*) => (
    log!("PANIC", $($arg)*)
    // TODO(ryan): macro compiler not letting me put stuff here...
  )
)

pub mod arch;
mod terminal;
mod panic;
mod multiboot;
mod allocator;
mod scheduler;
mod pci;
mod rtl8139;
mod driver;
mod net;

extern "rust-intrinsic" {
  pub fn transmute<T, U>(x: T) -> U;
}


#[no_mangle]
pub extern "C" fn callback() {
  debug!("    in an interrupt!");
}

#[no_mangle]
pub extern "C" fn callback_i(u: u32) {
  debug!("    got interrupt number: {}", u)
}

fn test_allocator() {
  let mut v = vec::Vec::new();
  
  debug!("Testing allocator with a vector push");
  v.push("   hello from a vector!");
  debug!("   push didn't crash");
  match v.pop() {
    Some(string) => debug!("{}", string),
    None => debug!("    push was weird...")
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
    set_allocator((15u * 1024 * 1024) as *mut u8, (20u * 1024 * 1024) as *mut u8);
    test_allocator();
    
    if magic != multiboot::MULTIBOOT_BOOTLOADER_MAGIC {
      kpanic!("Multiboot magic is invalid");
    } else {
      debug!("Multiboot magic is valid. Info at 0x{:x}", info as u32);
      (*info).multiboot_stuff();
    }
    
    debug!("{}", (*TEST)[0]);
    let cpu = cpu::CPU::current();
    
    (*cpu).make_keyboard(put_char);
    
    (*cpu).enable_interrupts();
    debug!("Going to interrupt: ");
    (*cpu).test_interrupt();
    debug!("    back from interrupt!");
    
    debug!("start scheduling...");
    //(*cpu).disable_interrupts();
    scheduler::thread_stuff(); // <-- currently broken :(
    
    pci_stuff();
    
    info!("Kernel is done!");
    //(*cpu).enable_interrupts();
    loop {
      (*cpu).idle()
    }
  }
}

fn pci_stuff() {
  let mut pci = Pci::new();
  pci.init();
  let mut drivers = (&mut pci as &mut DriverManager).get_drivers();
  debug!("Found drivers for {} pci devices", drivers.len());
  match drivers.pop() {
    Some(mut driver) => {
      driver.init();
      net::NetworkStack::new(driver).test().ok();
    }
    None => ()
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
  debug!("{} 0x{:x}", s, u)
}

#[no_mangle]
pub extern "C" fn __morestack() {
  loop { } //TODO(ryan) should I do anything here?
}
