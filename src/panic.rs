use core::prelude::*;
use terminal;
use core::mem::transmute;

//pub const GLOBAL_WRITER: &'static terminal::Terminal = ;

#[lang = "panic_fmt"] #[inline(never)] #[cold]
pub extern fn panic_impl(msg: ::core::fmt::Arguments,
                         file: &'static str,
                         line: uint) -> !
{
  //use io::Writer;
  unsafe {
    use io::Writer;
    let _ = write!(terminal::GLOBAL.lock(), "{}:{} {}", file, line, msg);
    ::core::intrinsics::abort();
  }
}

pub unsafe fn init() {
  terminal::GLOBAL.lock().clear_screen()
}
