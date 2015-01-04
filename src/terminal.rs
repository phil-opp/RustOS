use core::prelude::*;
use core::cell::UnsafeCell;

use spinlock::Spinlock;

use arch::vga;

// TODO(john): next line is still breaking abstractions (but I can't
// find a nice way to init it either...)
pub static GLOBAL: Spinlock<Terminal> = Spinlock {
  lock: ::core::atomic::INIT_ATOMIC_BOOL,
  data: UnsafeCell {
    value: Terminal {
      current: Point(0,0),
      vga: unsafe { 0 as *mut vga::Buffer } //&mut vga::GLOBAL.value
    }
  },
};

struct Point(uint, uint);

pub struct Terminal {
  current: Point,
  vga:     *mut vga::Buffer
}

impl Terminal
{
  fn get_vga_mut(&mut self) -> &mut vga::Buffer {
    unsafe { &mut *self.vga }
  }

  fn put_char(&mut self, c: u8) {
    if c == '\n' as u8 {
      self.current = Point(0, self.current.1 + 1);
    } else {
      self.get_vga_mut()[self.current.1][self.current.0] =
        vga::Entry::new(c, vga::Color::White, vga::Color::Black);
      self.current.0 += 1;
    }

    // line wrap
    if self.current.0 >= vga::X_MAX {
      self.current.0 = 0;
      self.current.1 += 1;
    }

    if self.current.1 >= vga::Y_MAX {
      self.scroll();
      self.current.1 = vga::Y_MAX - 1;
    }
  }


  fn scroll(&mut self)
  {
    let mut rows = self.get_vga_mut().iter_mut();

    let mut n     = rows.next();
    let mut suc_n = rows.next();

    while let (&Some(ref mut a), &Some(ref mut b)) = (&mut n, &mut suc_n) {
      ::core::mem::swap(*a, *b); // TODO(john) just need to copy b -> a
      n = suc_n;
      suc_n = rows.next();
    }
    unsafe {
      *n.unwrap() = ::core::mem::zeroed(); // last row
    }
  }

  pub fn clear_screen(&mut self) {
    unsafe {
      *self.get_vga_mut() = ::core::mem::zeroed();
    }
  }
}

pub fn init_global() {
  let mut guard = GLOBAL.lock();
  unsafe {
    guard.vga = vga::GLOBAL.get();
  }
  guard.clear_screen();
}


impl ::io::Writer for Terminal
{
  //type Err = ();

  fn write(&mut self, buf: &[u8]) -> Result<uint, ()> {
    for &c in buf.iter() {
      self.put_char(c);
    }
    Ok(buf.len())
  }

  fn flush(&mut self) -> Result<(), ()> {
    Ok(())
  }
}
