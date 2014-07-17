extern crate core;
use core::prelude::*;
use core::mem::transmute;

pub static VGA_START: *u16 = 0xb8000 as *u16; // TODO(ryan) this shouldn't be exposed
pub static VGA_MAX: (uint, uint) = (80, 24);

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

pub struct VGA { // TODO(ryan) struct fields shouldn't be exposed
  pub mapped: *u16,
  pub max: (uint, uint)
}

impl VGA {

  #[inline(always)]
  pub fn new() -> VGA {
    VGA { mapped: VGA_START, max: VGA_MAX }
  }

  pub fn put(&mut self, point: (uint, uint), chr: u8, fg: Color, bg: Color) -> bool {
      let (desiredX, desiredY) = point;
      let (myX, myY) = self.max;
      if desiredX >= myX || desiredY >= myY {
	false
      } else {
	unsafe {
	  let as_mut: *mut u16 = transmute(self.mapped.offset((myX * desiredY + desiredX) as int));
	  *as_mut = make_vgaentry(chr, make_color(fg, bg));
	}
	true
      }
  }
  
  pub fn x_max(&self) -> uint {
    let (x, _) = self.max;
    x as uint
  }

  pub fn y_max(&self) -> uint {
    let (_, y) = self.max;
    y as uint
  }
  
}

fn make_color(fg: Color, bg: Color) -> u8 {
  (fg as u8) | (bg as u8) << 4
}


fn make_vgaentry(c: u8, color_entry: u8) -> u16 {
  let c8: u8 = c as u8;
  let c16: u16 = c8 as u16;
  let color16: u16 = color_entry as u16;
  c16 | color16 << 8
}
