use arch::vga;
use panic::panic_message;
use std::io::{Writer, IoResult};
use std::mem::transmute;
use std::ptr::RawPtr;

// TODO(ryan): next line is breaking abstractions (but can't find a nice way to init it...)
pub static mut TERMINAL: Terminal = Terminal { vga: vga::VGA { mapped: vga::VGA_START, max: vga::VGA_MAX }, current: Point {x: 0, y: 0} };

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
      self.vga.put((self.current.x, self.current.y), c, vga::White, vga::Black);
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
    range(1, self.vga.y_max(), |j| {
      range(0, self.vga.x_max(), |i| {
	let (chr, fg, bg) = match self.vga.get((i, j)) {
	  Some(tup) => tup,
	  None => panic_message("error in Terminal.scroll")
	};
	self.vga.put((i, j - 1), chr, fg, bg);
      })
    });
    range(0, self.vga.x_max(), |i| {
      let y_max = self.vga.y_max();
      self.vga.put((i, y_max - 1), 'a' as u8, vga::Black, vga::Black);
    });
  }
  
  pub fn clear_screen(&mut self) {
    range(0, self.vga.x_max(), |i| {
	range(0, self.vga.y_max(), |j| {
	  self.vga.put((i, j), 0 as u8, vga::Black, vga::Black);
	});
    });
  }
  
  pub fn put_hex(&mut self, c: u8) {
    let (upper, lower) = itoc(c);
    self.put_char('0' as u8);
    self.put_char('x' as u8);
    self.put_char(upper);
    self.put_char(lower);
  }
  
  pub fn put_int(&mut self, w: u32) {
    let (a, b, c, d, e, f, g, h) = wtoc(w);
    self.put_char('0' as u8);
    self.put_char('x' as u8);
    self.put_char(a);
    self.put_char(b);
    self.put_char(c);
    self.put_char(d);
    self.put_char(e);
    self.put_char(f);
    self.put_char(g);
    self.put_char(h);
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

impl Writer for Terminal {

  fn write(&mut self, buf: &[u8]) -> IoResult<()> {
    for &c in buf.iter() {
      self.put_char(c);
    }
    Ok(())
  }
  
}

fn itoc(i: u8) -> (u8, u8) {
  let lower = hex(0xf & i);
  let upper = hex((0xf0 & i) >> 4);
  (upper, lower)
}

fn wtoc(i: u32) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
  let (a, b) = itoc((i & 0xff) as u8);
  let (c, d) = itoc(((i & 0xff00) >> 8) as u8);
  let (e, f) = itoc(((i & 0xff0000) >> 16) as u8);
  let (g, h) = itoc(((i & 0xff000000) >> 24) as u8);
  
  (g, h, e, f, c, d, a, b) // TODO(ryan): why is it big endian?
}

fn hex(i: u8) -> u8 {
  match i {
    0...9 => 0x30 + i,
    0xA...0xF => 0x41 + (i - 0xA),
    _ => 122 // 'z'
  }
}

fn range(lo: uint, hi: uint, it: |uint| -> ()) {
    let mut iter = lo;
    while iter < hi {
	it(iter);
	iter += 1;
    }
}
