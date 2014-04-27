#![no_std]
#![allow(ctypes)]

mod idt;
mod vga;

#[inline]
#[lang="fail_"]
pub fn fail_(_: *u8, _: *u8, _: uint) -> ! {
    unsafe {
      abort()
    }
}

extern "C" {
  
  fn abort() -> !;
  
}

extern "rust-intrinsic" {
    pub fn transmute<T, U>(x: T) -> U;
}

pub fn panic() {
  unsafe {
    ::vga::TERMINAL.println("panic!");
  }
}

extern "C" fn callback() {
  unsafe {
    ::vga::TERMINAL.println("its an interrupt!");
  }
}

fn float_to_int(x: f32) -> u32 {
  unsafe {
    let i: *u32 = transmute(&x);
    *i
  }
}


#[no_mangle]
pub unsafe fn main() {
    vga::clear_screen(::vga::Black);
    vga::TERMINAL = vga::Terminal::new(0xb8000, 160, 24);
    
    vga::TERMINAL.println("hello, world");
    vga::TERMINAL.print("behold a kernel! and here is float division: ");
    vga::TERMINAL.put_int(float_to_int(10.0 / 4.0));
    vga::TERMINAL.println("");
    
    let mut idt = ::idt::IDT::new(0xb900, 0x400);
    let mut i = 0;
    
    vga::TERMINAL.print("callback bytes: ");
    vga::TERMINAL.put_int(callback as u32);
    
    while i < 0x400 {
      idt.add_entry(i, callback);
      i += 1;
    }
    idt.enable();
    
    loop { }
}
