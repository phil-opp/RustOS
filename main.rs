#![no_std]
#![allow(ctypes)]

extern "rust-intrinsic" {
    fn transmute<T, U>(x: T) -> U;

    fn offset<T>(dst: *T, offset: int) -> *T;
}

enum Color {
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

struct Terminal {
    start: u32, // TODO should be generic pointer
    max: Point,
    current: Point,
}

impl Terminal {

  fn put_char(&mut self, c: u8) {
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
  
  fn put_hex(&mut self, c: u8) {
    let (upper, lower) = itoc(c);
    self.put_char('0' as u8);
    self.put_char('x' as u8);
    self.put_char(upper);
    self.put_char(lower);
  }
  
  fn put_int(&mut self, w: u32) {
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

    
  fn print(&mut self, s:  &'static str) {
    let (ptr, buflen): (*u8, u32) = unsafe {
      transmute(s)
    };
    let mut i = 0u32;
    self.put_char('L' as u8);
    self.put_hex(buflen as u8);
    self.put_char(' ' as u8);
    
    self.put_char('A' as u8);
    self.put_int(ptr as u32);
    
    while i < buflen {
      unsafe {
	self.put_hex(*offset(ptr, i as int)); 
	self.put_char(' ' as u8)
      }
      i += 1;
    }
    i = 0;
    while i < buflen {
      unsafe {
	self.put_char(*offset(ptr, i as int)); 
      }
      i += 1;
    }
  }
  
  fn new(start: u32, max: Point) -> Terminal {
    Terminal { start: start, max: max, current: Point { x: 0, y: 0 } }
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


unsafe fn clear_screen(background: Color) {
    range(0, 80*25, |i| {
        *((0xb8000 + i * 2) as *mut u16) = (background as u16) << 12;
    });
}

#[no_mangle]
pub unsafe fn main() {
    clear_screen(Black);
    let mut t = Terminal::new(0xb8000, Point { x: 80, y: 24 });
    /*t.put_char('h' as u8);
    t.put_char('e' as u8);
    t.put_char('l' as u8);
    t.put_char('l' as u8);
    t.put_char('o' as u8);
    t.put_char('i' as u8);
    t.put_char(' ' as u8);
    */
    t.print("abc");
    t.print("defg");
    t.print("defghiasf");
    
    t.put_char('b' as u8);
    t.put_char('y' as u8);
    t.put_char('e' as u8);
    loop {
      //clear_screen(Black)
    }
}
