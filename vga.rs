#![no_std]
#![allow(ctypes)]

pub static mut TERMINAL: Terminal = Terminal { start: 0, max: Point { x: 0, y: 0 }, current: Point { x: 0, y: 0 } };

extern "rust-intrinsic" {
    pub fn transmute<T, U>(x: T) -> U;

    fn offset<T>(dst: *T, offset: int) -> *T;
}

extern "C" {
  
  fn abort() -> !;
  
}

pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Pink       = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    LightPink  = 13,
    Yellow     = 14,
    White      = 15,
}

struct Point {
    x: u32,
    y: u32
}

pub struct Terminal {
    start: u32, // TODO should be generic pointer
    max: Point,
    current: Point,
}

#[inline]
#[lang="fail_"]
pub fn fail_(_: *u8, _: *u8, _: uint) -> ! {
    unsafe {
      abort()
    }
}

impl Terminal {


  pub fn put_char(&mut self, c: u8) {
    if c == '\n' as u8 {
      self.current = Point { x : 0, y : (self.current.y + 1) };
    } else {
      unsafe {
	self.current = match self.current {
	  Point { x: x, y: y }  if x == self.max.x - 1 && y == self.max.y - 1  => Point {x: 0, y: 0},
	  Point { y: y, ..} if y == self.max.y - 1 => Point { x: 0, y: y + 1 },
	  Point { x: x, y: y } => Point { x: x + 1 * 2, y: y } // TODO sizeof
	};
	let entry = make_vgaentry(c, make_color(White, Black));
	*((self.start + self.current.y * self.max.x + self.current.x) as *mut u16) = entry;
      }
    }
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
    let (ptr, buflen): (*u8, u32) = unsafe {
      transmute(s)
    };
    let mut i = 0;
    while i < buflen {
      unsafe {
	self.put_char(*offset(ptr, i as int)); 
      }
      i += 1;
    }
  }
  
  pub fn println(&mut self, s:  &'static str) {
    self.print(s);
    self.put_char('\n' as u8);
  }
  
  pub fn new(start: u32, x: u32, y: u32) -> Terminal {
    Terminal { start: start, max: Point { x: x, y: y}, current: Point { x: 0, y: 0 } }
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
  
  (a, b, c, d, e, f, g, h)
}

fn hex(i: u8) -> u8 {
  match i {
    0..9 => 0x30 + i,
    0xA..0xF => 0x41 + (i - 0xA),
    _ => 122 // 'z'
  }
}

fn make_color(fg: Color, bg: Color) -> u8 {
  (fg as u8) | (bg as u8) << 4
}

fn make_vgaentry(c: u8, color_entry: u8) -> u16{
  let c8: u8 = c as u8;
  let c16: u16 = c8 as u16;
  let color16: u16 = color_entry as u16;
  c16 | color16 << 8
}


fn range(lo: uint, hi: uint, it: |uint| -> ()) {
    let mut iter = lo;
    while iter < hi {
	it(iter);
	iter += 1;
    }
}


pub unsafe fn clear_screen(background: Color) {
    range(0, 80*25, |i| {
	*((0xb8000 + i * 2) as *mut u16) = (background as u16) << 12;
    });
}
