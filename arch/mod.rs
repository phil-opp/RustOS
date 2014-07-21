#[cfg(target_arch = "x86")]
#[path="x86_32/vga.rs"]
pub mod vga;

#[cfg(target_arch = "x86")]
#[path="x86_32/thread.rs"]
pub mod thread;