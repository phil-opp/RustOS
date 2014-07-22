LD=i686-elf-ld
RUSTC=rustc
QEMU=qemu-system-i386
AS=i686-elf-as
TARGET=i686-unknown-linux-gnu
RUSTFLAGS=-g -O --crate-type=lib --cfg=kernel -C linker=$(LD) --target $(TARGET) -L . -Z no-landing-pads

all: boot.bin

.SUFFIXES: .o .s .rlib

.PHONY: clean run

.s.o:
	$(AS) -g -o $@ $<

lazy_static: lazy-static/src/lazy_static.rs std
	$(RUSTC) $< $(RUSTFLAGS)
	
alloc: rust/src/liballoc/lib.rs core libc
	$(RUSTC) $< $(RUSTFLAGS)
	
core: rust/src/libcore/lib.rs
	$(RUSTC) $< $(RUSTFLAGS)

collections: rust/src/libcollections/lib.rs core alloc
	$(RUSTC) $< $(RUSTFLAGS)

rand: rust/src/librand/lib.rs core
	$(RUSTC) $< $(RUSTFLAGS)
	
std: rust/src/libstd/lib.rs core alloc collections rand libc rustrt
	$(RUSTC) $< $(RUSTFLAGS)

libc: rust/src/liblibc/lib.rs
	$(RUSTC) $< $(RUSTFLAGS)
	
rustrt: rust/src/librustrt/lib.rs core alloc libc collections
	$(RUSTC) $< $(RUSTFLAGS)

	
main.o: main.rs std alloc core collections lazy_static
	$(RUSTC) $< -o $@ --emit=obj $(RUSTFLAGS)
	
rlibc.o: rust/src/librlibc/lib.rs 
	$(RUSTC) $< -o $@ --emit obj $(RUSTFLAGS)
		
run: boot.bin
	$(QEMU) -kernel $<

debug: boot.bin
	$(QEMU) -S -gdb tcp::3333 -kernel $<

%.o: arch/x86/%.s
	$(AS) -g -o $@ $<
	
boot.bin: linker.ld main.o boot.o interrupt.o thread.o rlibc.o
	$(LD) -o $@ -T $^

iso: boot.bin
	cp boot.bin isodir/boot/
	grub-mkrescue -o boot.iso isodir

vb: iso
	virtualbox --debug --startvm rustos
	
clean:
	rm -f *.bin *.o *.img *.iso *.rlib
