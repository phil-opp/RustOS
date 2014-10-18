RustOS
=====

A simple, [language-based](https://en.wikipedia.org/wiki/Language-based_system) OS.


### Current features:
  * Simple VGA for seeing output
  * Some Rust libraries (core, alloc, collections) already in
  * Working (but limited) keyboard input

### Building:
1. Dependencies:
  * qemu (emulator) or grub-mkrescue (run an iso on a VM or hardware)
  * as
  * ld
  * rustc (0.12.0)
2. Pull this repo `git clone https://github.com/ryanra/RustOS.git`
3. Make sure to pull the submodules as well: `git submodule update --init --recursive`
4. Run:
  * On qemu: `make run`
  * Or, make an iso `make iso` and run it on a VM or real hardware!

### Design goals:
1. Implement the entire Rust standard library *on bare metal*. Essentially, 
you should be able to write your program, link against `std`, add a bootloader, and run
on bare metal. Of course, we'll need a little more to make the operating system extensible (specifically,
an interface for adding drivers and libraries)

2. The OS will be as simple as possible with as little as possible in it. Specifically, Rust type safety allows us to omit:
  * Paging. CPU memory protection is unecessary if you can only execute safe code
  * Syscalls. You can only call functions exported in `std` (there is the issue of `unsafe` though, which will need to be considered at some point)
  * (This simplicitly may also end up scoring in terms of performance!)

3. Micro/Monolithic kernel is really irrelevant because everything is running in kernel mode and safety
  is enforced by the language, so there's no need for user mode. That said, the goal is to keep this code 
  base small and enforce least-privledge with tight modules that also allow future additions.

3. Security. That's the big advantage that Rust would bring to an OS (i.e., memory safety) and that current OSes are really lacking.
  
### Short-term goals:
1. ~~Handle interrupts, specifically get the keyboard working.~~ done!
2. Threading/Multiprocessing
  * There's the beginnings of a single-core implementation, but it looks like `libgreen` can be slightly modified to this end
3. Other architectures:
  * There's some beginnings of architecture-agnostic code, but it needs to be taken further

### Longer-term goals:

1. Basic drivers
  * This should include a modular and secure (least privledge) interface for adding your own drivers as well
2. A filesystem
3. Network stack
4. Port `rustc` to RustOS
5. That's probably it!

### Current issues:
1. ~~Linkage probelms~~ fixed!
2. Threading is buggy and needs more attention.
3. The current allocator never actually frees data and is just there to get `collections` working.

### Organization:
1. Architecture-specific files (mostly) are now in arch/
2. `std` had been stripped out of dependencies on an OS/libc and is usable (so, we can use stuff `libcore`, `libcollections`, `librand`)
  * The idea is to move most of the functionality into a runtime library in a fork of rust so we can support `libstd`

### License
[Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) or the [MIT license](http://opensource.org/licenses/MIT), at your option. See See LICENSE-APACHE and LICENSE-MIT for details.
