#![no_std]
#![feature(associated_types)]
#![feature(phase)]
#![allow(ctypes)]
#![feature(globs)]
#![feature(asm)]
#![feature(macro_rules)]
#![feature(lang_items)]

// not directly used, but needed to link to llvm emitted calls
extern crate rlibc;

#[phase(plugin, link)]
//extern crate std; // for useful macros and IO interfaces
extern crate core;
extern crate alloc;
extern crate collections;

extern crate "external" as bump_ptr;

#[phase(plugin, link)]
extern crate lazy_static_spin;

#[phase(plugin)]
extern crate bitflags;


use core::prelude::*;

use collections::Vec;

use multiboot::multiboot_info;
use arch::cpu;/*
use pci::Pci;
use driver::DriverManager;
use thread::scheduler;
*/
#[macro_escape]
mod log;
pub mod arch;
mod terminal;
//mod panic;
mod multiboot;
//mod thread;
//mod pci;
//mod rtl8139;
//mod driver;
//mod net;

mod io;


fn test_allocator() {
  let mut v = Vec::new();

  debug!("Testing allocator with a vector push");
  v.push("   hello from a vector!");
  debug!("   push didn't crash");
  match v.pop() {
    Some(string) => debug!("{}", string),
    None => debug!("    push was weird...")
  }

}

fn put_char(c: u8) {
    print!("{:c}", c as char);
}

lazy_static_spin! {
  static TEST: *mut Vec<&'static str> = {
    let mut v = box Vec::new();
    v.push("hi from lazy sttaic");
    unsafe {
      use core::mem::transmute;
      let ptr = transmute(&*v);
      core::mem::forget(v);
      ptr
    }
  };
}

#[no_mangle]
pub extern "C" fn main(magic: u32, info: *mut multiboot_info) {
  unsafe {
    bump_ptr::set_allocator((15u * 1024 * 1024) as *mut u8, (20u * 1024 * 1024) as *mut u8);
    //panic::init();
    test_allocator();

    /*
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

    scheduler::thread_stuff();

    pci_stuff();

    info!("Kernel is done!");
    */
    loop {
      (**cpu::CURRENT_CPU).idle()
    }
  }
}
/*
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
*/
#[no_mangle]
pub extern "C" fn debug(s: &'static str, u: u32) {
  debug!("{} 0x{:x}", s, u)
}

#[no_mangle]
pub extern "C" fn __morestack() {
  loop { } //TODO(ryan) should I do anything here?
}

#[no_mangle]
pub extern "C" fn abort() -> ! {
    unsafe { core::intrinsics::abort(); }
}

#[no_mangle]
pub extern "C" fn callback() {
  debug!("    in an interrupt!");
}

// TODO(ryan): figure out what to do with these:

#[lang = "stack_exhausted"]
extern fn stack_exhausted() {}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt(_fmt: &core::fmt::Arguments, _file: &'static str, _line: uint) -> ! {
  loop {}
}

// for deriving
#[doc(hidden)]
mod std {
  pub use core::clone;
  pub use core::cmp;
  pub use core::kinds;
  pub use core::option;
  pub use core::fmt;
  pub use core::hash;
}
