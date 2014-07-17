use core::option::Option;
use core::option::None;
use core::option::Some;
use terminal::TERMINAL;

pub fn init() {
  unsafe { TERMINAL.clear_screen(); }
}

pub fn print(string: &'static str) {
  unsafe {
    TERMINAL.print(string);
  }
}

pub fn println(string: &'static str) {
  unsafe {
    TERMINAL.println(string);
  }
}

pub fn put_int(integer: u32) {
  unsafe {
    TERMINAL.put_int(integer);
  }
}
pub fn panic() {
  unsafe {
    println("panic!");
  }
  loop {}
}

pub unsafe fn abort() {
  unsafe {
    println("kernel panic! (from abort())");
  }
  loop {}
}
