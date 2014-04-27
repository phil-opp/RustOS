#![no_std]
#![allow(ctypes)]
mod idt;
mod vga;

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
    let mut t = vga::TERMINAL;
    
    t.println("hello, world");
    t.print("behold a kernel! and here is float division: ");
    t.put_int(float_to_int(10.0 / 4.0));
    
    let mut idt = ::idt::IDT::new(0xc8000 as *u8, 0x400);
    let mut i = 0;
    while i < 0x400 {
      idt.add_entry(i, callback);
      i += 1;
    }
    //idt.enable(); <- still not working :(
    
    loop {
    
    }
}
