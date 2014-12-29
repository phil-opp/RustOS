pub use self::imp::*;

#[cfg(target_arch = "x86")]
#[path="x86"]
mod imp {
  pub mod vga;
  pub mod context;
  pub mod cpu;
  pub mod idt; // TODO shouldn't be pub
  pub mod gdt; // TODO shouldn't be pub
  pub mod keyboard;
}
