#[cfg(target_arch = "x86")]
#[path="x86/vga.rs"]
pub mod vga;

#[cfg(target_arch = "x86")]
#[path="x86/context.rs"]
pub mod context;

#[cfg(target_arch = "x86")]
#[path="x86/cpu.rs"]
pub mod cpu;

#[cfg(target_arch = "x86")]
#[path="x86/idt.rs"]
mod idt;

#[cfg(target_arch = "x86")]
#[path="x86/gdt.rs"]
mod gdt;

#[cfg(target_arch = "x86")]
#[path="x86/keyboard.rs"]
pub mod keyboard;