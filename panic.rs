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

pub fn panic_message(string: &'static str) -> ! {
  unsafe {
    println(string);
    println("^ panic ->");
  }
  loop {}
}

pub unsafe fn abort() -> ! {
  panic_message("(from abort)");
  loop {}
}
