#[cfg(target_arch = "x86")]
#[path="x86/vga.rs"]
pub mod vga;

#[cfg(target_arch = "x86")]
#[path="x86/thread.rs"]
pub mod thread;

#[cfg(target_arch = "x86")]
#[path="x86/cpu.rs"]
pub mod cpu;

#[cfg(target_arch = "x86")]
#[path="x86/idt.rs"]
mod idt;

#[cfg(target_arch = "x86")]
#[path="x86/gdt.rs"]
mod gdt;