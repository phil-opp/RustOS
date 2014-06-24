use core::option::Option;
use core::option::None;
use core::option::Some;
use terminal::Terminal;

mod arch;
mod terminal;


static mut TERMINAL: Option<Terminal> = None;

unsafe fn do_with(something: |mut t: Terminal| -> ()) {
  match TERMINAL {
    Some(term) => something(term),
    None => ()
  }
}

pub unsafe fn print(string: &'static str) {
  do_with(|mut term| term.print(string));
}

pub unsafe fn panic() {
  unsafe {
    do_with(|mut term| term.println("panic!"));
  }
  loop {}
}

pub unsafe fn abort() {
  unsafe {
    do_with(|mut term| term.println("kernel panic! (from abort())"));
  }
  loop {}
}
