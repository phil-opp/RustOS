use arch::cpu::Port;

static KEY_CODE_TO_ASCII: &'static [u8] = b"??1234567890-=??qwertyuiop[]\n?asdfghjkl;'`?\\zxcvbnm,./?*? ?"; 

pub struct Keyboard {
  callback: fn (u8) -> (),
  control_port: Port,
  data_port: Port
}

bitflags!(
  flags Status: u8 {
    static output_full = 0b00000001,
    static input_full = 0b00000010,
    static system = 0b00000100,
    static command = 0b00001000,
    static keyboard_locked = 0b00010000,
    static auxiliary_output_full = 0b00100000,
    static timeout = 0b01000000,
    static parity_error = 0b10000000
  }
)

impl Keyboard {

  pub fn new(callback: fn (u8) -> (), control_port: Port, data_port: Port) -> Keyboard {
    Keyboard { callback: callback, control_port: control_port, data_port: data_port }
  }
  
  pub fn register_callback(&mut self, callback: fn (u8) -> ()) {
    self.callback = callback;
  }
  
  fn get_status(&mut self) -> Status {
    Status::from_bits(self.control_port.read_u8().unwrap()).unwrap()
  }
  /*
  fn send_command(&mut self, command: Command) {
    while get_status().output_full as bool {}
    control_port.write_u8(command);
  }*/
  
  pub fn got_interrupted(&mut self) {
    let keycode = self.data_port.read_u8().unwrap();
    match KEY_CODE_TO_ASCII.get(keycode as uint) {
      Some(ascii) => {
	let func = self.callback;
	func(*ascii);
      },
      None => ()
    }
  }
  
  
}

extern "C" {

  fn debug(s: &str, u: u32);

}