use core::option::Option;
use core::option::None;
use core::option::Some;
use terminal::Terminal;

mod arch;
mod terminal;

pub fn init() {
  unsafe { terminal::TERMINAL.clear_screen(); }
}

pub fn print(string: &'static str) {
  unsafe {
    terminal::TERMINAL.print(string);
  }
}

pub fn println(string: &'static str) {
  unsafe {
    terminal::TERMINAL.println(string);
  }
}

pub fn put_int(integer: u32) {
  unsafe {
    terminal::TERMINAL.put_int(integer);
  }
}
pub unsafe fn panic() {
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
