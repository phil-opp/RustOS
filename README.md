RustOS
=====

A simple, [language-based](https://en.wikipedia.org/wiki/Language-based_system) OS.


### Current features:
  * Simple VGA for seeing output
  * Some Rust libraries (core, alloc, collections) already in

### Building:
1. Dependencies:
  * qemu (emulator) or grub-mkrescue (run an iso on a VM or hardware)
  * as (x86 32 bit)
  * ld (x86 32 bit)
  * rustc (version 0.11)

2. Pull the repo `git clone https://github.com/ryanra/RustOS.git`

3. Make sure to pull the submodules as well: `git submodule update --init --recursive`

4. Run:
  * On qemu: `make run`
  * Or, make an iso `make iso` and run it on a VM or real hardware!

### Design goals:
1. Implement the entire Rust standard library *on bare metal*. Essentially, 
you should be able to write your program, link against `std`, add a bootloader, and run
on bare metal. Of course, we'll need a little more to make the operating system extensible (specifically,
an interface for adding drivers and libraries)

2. *KISS* The OS is as simple as possible with as little as possible in it. Specifically, 
  1. Rust type safety allows us to omit:
  * Paging. CPU memory protection is unecessary if you can only execute safe code
  * Syscalls. You can only call functions exported in `std` (there is the issue of `unsafe` though, which
will need to be considered at some point)
  * (This simplicitly may also end up scoring in terms of performance!)

  2. Micro/Monolithic kernel is really irrelevant because everything is running in kernel mode and safety
  is enforced by the language, so there's no need for user mode. That said, the goal is to keep this code 
  base small and enforce least-privledge with tight modules that also allow future additions.

3. Security. That's the big advantage that Rust would bring to an OS and that current OSes are really
lacking.
  
### Short-term goals:
1. Handle interrupts, specifically get the keyboard working.

2. Multiprocessing
  * A current (buggy) single-core implementation already exists
  * Implement locking primitives and enable Rust's `sync` library

3. Other architectures:
  * There's some beginnings of architecture-agnostic code, but it needs to be taken further

### Longer-term goals:

1. Basic drivers
  * This should include a modular and secure (least privledge) interface for adding your own drivers as well
2. A filesystem
3. Network stack
4. Bring in a rustified core-utils port?
5. That's probably it!

### Current issues:
1. Linkage probelms: a stripped-down version of ``std'' is compiled from the Rust sources and is used as a crate. 
Non-inlined functions aren't actually added to the object file and so it will cause problems when linking the final
kernel. The current (horrible) solution is to inline used methods.

2. Threading is buggy and needs more attention.

3. The current allocator never actually frees data and is just there to get collections working.

### Organization:
1. Not that much right now because it's so simple. It feels like some (or maybe all?) of this stuff could be moved 
into the Rust code-base (especially the allocator) as an alternative set of primitives to the current Win/Unix ones.

### License
[Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) or the [MIT license](http://opensource.org/licenses/MIT), at your option
