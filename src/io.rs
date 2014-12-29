//! proposed new Reader-Writer traits here until core gets them

use core::prelude::*;
/*
trait Reader {
  type Err; // new associated error type

  // unchanged except for error type
  fn read(&mut self, buf: &mut [u8]) -> Result<uint, Reader::Err>;

  // these all return partial results on error
  //fn read_to_end(&mut self) -> NonatomicResult<Vec<u8>, Vec<u8>, Err> { ... }
  //fn read_to_string(&self) -> NonatomicResult<String, Vec<u8>, Err> { ... }
  //fn read_at_least(&mut self, min: uint,  buf: &mut [u8]) -> NonatomicResult<uint, uint, Err>  { ... }
}

trait Writer {
  type Err;
  
  fn write(&mut self, buf: &[u8]) -> Result<uint, Reader::Err>;

  //fn write_all(&mut self, buf: &[u8]) -> NonatomicResult<(), uint, Err> { ... };
  //fn write_fmt(&mut self, fmt: &fmt::Arguments) -> Result<(), Err> { ... }
  //fn flush(&mut self) -> Result<(), Err> { ... }
}
*/
