macro_rules! print(
    ($($arg:tt)*) => ({
        use std::prelude::*;
        use panic::term;
        term().write(format!($($arg)*).as_bytes()).ok();
    })
)

macro_rules! log(
    ($lvl: expr, $($arg:tt)*) => (
          {
          print!("[{}:{} {}]: ", $lvl, file!(), line!())
          print!($($arg)*)
          print!("\n")
          }
    )
)

macro_rules! debug( 
  ($($arg:tt)*) => (log!("DEBUG", $($arg)*))
)

macro_rules! warn( 
  ($($arg:tt)*) => (log!("WARN", $($arg)*))
)

macro_rules! info( 
  ($($arg:tt)*) => (log!("INFO", $($arg)*))
)

macro_rules! trace( 
  ($($arg:tt)*) => (log!("TRACE", $($arg)*))
)

macro_rules! kassert(
  ($b: expr) => ({
        if !$b {
          debug!("assertion failed {}", stringify!(b))
          ::panic::abort();
        }
    })
)

macro_rules! kpanic(
  ($($arg:tt)*) => ({
    log!("PANIC", $($arg)*)
    kassert!(false);
  })
)

macro_rules! not_reached(
  ($($arg:tt)*) => ({
    log!("PANIC", $($arg)*)
    ::panic::abort();
  })
)
