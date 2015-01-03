use core::prelude::*;

use arch::vga;

// TODO(ryan): next line is breaking abstractions (but can't find a nice way to init it...)
lazy_static_spin! {
  pub static TERMINAL: Terminal = {
    Terminal::init()
  };
}

struct Point {
  x: uint,
  y: uint
}

pub struct Terminal {
  current: Point,
  vga: vga::VGA
}

impl Terminal {

  pub fn init() -> Terminal {
    Terminal::new(vga::VGA::new())
  }

  pub fn new(vga: vga::VGA) -> Terminal {
    Terminal { vga: vga, current: Point {x: 0, y: 0} }
  }

  pub fn put_char(&mut self, c: u8) {
    if c == '\n' as u8 {
      self.current = Point { x : 0, y : (self.current.y + 1) };
    } else {
      self.vga.put((self.current.x, self.current.y), c, vga::Color::White, vga::Color::Black);
    }

    self.current.x += 1;
    if self.current.x >= self.vga.x_max() {
      self.current.x = 0;
      self.current.y += 1;
    }
    if self.current.y >= self.vga.y_max() {
      self.scroll();
      self.current.y = self.vga.y_max() - 1;
    }
  }


  fn scroll(&mut self) {
    for j in range(1, self.vga.y_max()) {
      for i in range(0, self.vga.x_max()) {
        let (chr, fg, bg) = match self.vga.get((i, j)) {
          Some(tup) => tup,
          None => panic!("error in Terminal.scroll")
        };
        self.vga.put((i, j - 1), chr, fg, bg);
      }
    }
    for i in range(0, self.vga.x_max()) {
      let y_max = self.vga.y_max();
      self.vga.put((i, y_max - 1), 'a' as u8, vga::Color::Black, vga::Color::Black);
    }
  }

  pub fn clear_screen(&mut self) {
    for i in range(0, self.vga.x_max()) {
      for j in range(0, self.vga.y_max()) {
        self.vga.put((i, j), 0 as u8, vga::Color::Black, vga::Color::Black);
      }
    }
  }

  pub fn print(&mut self, s:  &'static str) {
    for c in s.chars() {
      self.put_char(c as u8);
    }
  }

  pub fn println(&mut self, s:  &'static str) {
    self.print(s);
    self.put_char('\n' as u8);
  }

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
