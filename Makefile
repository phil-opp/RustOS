LD=i686-elf-ld
RUSTC=rustc
QEMU=qemu-system-i386
AS=i686-elf-as
RUSTFLAGS=-g -O --crate-type=lib --cfg=rynux --cfg=x86_32 -C linker=$(LD) --target i686-unknown-linux-gnu -L . -Z no-landing-pads

all: boot.bin

.SUFFIXES: .o .s .rlib

.PHONY: clean run

.s.o:
	$(AS) -g -o $@ $<

lazy_static: lazy-static/src/lazy_static.rs core collections
	$(RUSTC) $< $(RUSTFLAGS)
	
alloc: rust/src/liballoc/lib.rs core
	$(RUSTC) $< $(RUSTFLAGS)
	
core: rust/src/libcore/lib.rs
	$(RUSTC) $< $(RUSTFLAGS)

collections: rust/src/libcollections/lib.rs core alloc
	$(RUSTC) $< $(RUSTFLAGS)
	
main.o: main.rs alloc core collections lazy_static
	$(RUSTC) $< -o $@ --emit=obj $(RUSTFLAGS)
	
rlibc.o: rust/src/librlibc/lib.rs 
	$(RUSTC) $< -o $@ --emit obj $(RUSTFLAGS)
		
run: boot.bin
	$(QEMU) -kernel $<

debug: boot.bin
	$(QEMU) -S -gdb tcp::3333 -kernel $<

thread.o: arch/x86_32/thread.s
	$(AS) -g -o $@ $<
	
boot.bin: linker.ld main.o boot.o interrupt.o thread.o rlibc.o
	$(LD) -o $@ -T $^

iso: boot.bin
	cp boot.bin isodir/boot/
	grub-mkrescue -o boot.iso isodir

vb: iso
	virtualbox --debug --startvm rynux
	
clean:
	rm -f *.bin *.o *.img *.iso *.rlib
