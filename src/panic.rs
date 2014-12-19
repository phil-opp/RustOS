use core::prelude::*;
pub use terminal::TERMINAL;

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

pub fn panic_message(string: &'static str) -> ! {
  println(string);
  println("^ panic ->");
  loop {}
}

pub fn abort() -> ! {
  panic_message("(from abort)");
  loop {}
}
