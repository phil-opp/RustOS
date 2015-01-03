// stack interface for RustOS

use core::prelude::*;

use collections::Vec;

pub struct Stack {
    v: Vec<u8>
}

impl Stack {

    pub fn new(size: uint) -> Stack {
        Stack { v: Vec::with_capacity(size) } 
    }
    
    /// Point to the low end of the allocated stack
    pub fn start(&self) -> *const uint {
        self.v.as_ptr() as *const uint
    }

    /// Point one uint beyond the high end of the allocated stack
    pub fn end(&self) -> *const uint {
        unsafe { self.v.as_ptr().offset(self.v.capacity() as int) as *const uint } // TODO(ryan) overflow on cast?
    }

}
