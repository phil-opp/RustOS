extern "C" {
  
  fn lgdt(ptr: *GDTReal);

}

extern "rust-intrinsic" {
    pub fn transmute<T, U>(x: T) -> U;

    fn offset<T>(dst: *T, offset: int) -> *T;
}

struct GDTEntry {
  limit: u32,
  base: u32,
  typ: u8
}


pub struct GDT {
  index: u32,
  real: GDTReal
}

#[packed]
struct GDTReal {
  limit: u16,
  base: u32
}

impl GDT {
  
  pub fn new(mem: u32, size: u16) -> GDT {
    GDT {index: 0, real: GDTReal { limit: size * 8, base: mem }} 
  }
  
  pub fn add_entry(&mut self, base: u32, limit: u32, typ: u8) {
    unsafe {
      encodeGdtEntry(transmute(self.index*8 + self.real.base), limit, base, typ);
    }
    self.index += 1;
  }
  
  pub fn enable(&mut self) {
    unsafe {
      lgdt(&self.real);
    }
  }

}

unsafe fn offset_mut(dst: *mut u8, offset: int) -> *mut u8 {
  transmute((dst as u32) + offset as u32)
}

unsafe fn encodeGdtEntry(target: *mut u8, mut limit: u32, base: u32, typ: u8) {
    // adapted from http://wiki.osdev.org/GDT_Tutorial
    // Check the limit to make sure that it can be encoded
    //let mut target: u32 = transmute(targ);
    if (limit > 65536) && (limit & 0xFFF) != 0xFFF {
        //kerror("You can't do that!");
    }
    if limit > 65536 {
        // Adjust granularity if required
        limit = limit >> 12;
        *offset_mut(target, 6) = 0xC0;
    } else {
        *offset_mut(target, 6) = 0x40;
    }
 
    // Encode the limit
    *offset_mut(target, 0) = (limit & 0xFF) as u8;
    *offset_mut(target, 1) = ((limit >> 8) & 0xFF) as u8;
    *offset_mut(target, 6) |= ((limit >> 16) & 0xF) as u8;
 
    // Encode the base 
    *offset_mut(target, 2) = (base & 0xFF) as u8;
    *offset_mut(target, 3) = ((base >> 8) & 0xFF) as u8;
    *offset_mut(target, 4) = ((base >> 16) & 0xFF) as u8;
    *offset_mut(target, 7) = ((base >> 24) & 0xFF) as u8;
 
    // And... Type
    *offset_mut(target, 5) = typ;
}