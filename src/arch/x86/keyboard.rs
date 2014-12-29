use core::prelude::*;

use arch::cpu::Port;

static KEY_CODE_TO_ASCII: &'static [u8] = b"??1234567890-=??qwertyuiop[]\n?asdfghjkl;'`?\\zxcvbnm,./?*? ?"; 

pub struct Keyboard {
  callback: fn (u8) -> (),
  control_port: Port,
  data_port: Port
}

bitflags! {
  flags Status: u8 {
    const OUTPUT_FULL     = 0b_00000001,
    const INPUT_FULL      = 0b_00000010,
    const SYSTEM          = 0b_00000100,
    const COMMAND         = 0b_00001000,
    const KEYBOARD_LOCKED = 0b_00010000,
    const AUX_OUTPUT_FULL = 0b_00100000,
    const TIMEOUT         = 0b_01000000,
    const PARITY_ERROR    = 0b_10000000
  }
}

impl Keyboard {

  pub fn new(callback: fn (u8) -> (), control_port: Port, data_port: Port) -> Keyboard {
    Keyboard { callback: callback, control_port: control_port, data_port: data_port }
  }
  
  pub fn register_callback(&mut self, callback: fn (u8) -> ()) {
    self.callback = callback;
  }
  
  #[allow(dead_code)]
  fn get_status(&mut self) -> Status {
    Status::from_bits(self.control_port.in_b()).unwrap()
  }
  
  /*
  fn send_command(&mut self, command: Command) {
    while get_status().output_full as bool {}
    control_port.write_u8(command);
  }*/
  
  pub fn got_interrupted(&mut self) {
    let keycode = self.data_port.in_b();
    match KEY_CODE_TO_ASCII.get(keycode as uint) {
      Some(ascii) => {
	let func = self.callback;
	func(*ascii);
      },
      None => ()
    }
  }
    
}
