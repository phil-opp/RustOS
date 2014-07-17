LD=i686-elf-ld
RUSTC=rustc
QEMU=qemu-system-i386
AS=i686-elf-as

all: boot.bin

.SUFFIXES: .o .s

.PHONY: clean run

.s.o:
	$(AS) -g -o $@ $<

alloc: rust/src/liballoc/lib.rs core
	$(RUSTC) --crate-type=lib --cfg=rynux -C passes=inline rust/src/liballoc/lib.rs --target i686-unknown-linux-gnu -L . -Z no-landing-pads 
	
core: rust/src/libcore/lib.rs
	$(RUSTC) --crate-type=lib -C passes=inline rust/src/libcore/lib.rs --target i686-unknown-linux-gnu -L . -Z no-landing-pads

collections: rust/src/libcollections/lib.rs core alloc
	$(RUSTC) --crate-type=lib -C passes=inline rust/src/libcollections/lib.rs --target i686-unknown-linux-gnu -L . -Z no-landing-pads
	
main.o: main.rs alloc core collections
	$(RUSTC) -g -O main.rs --crate-type=lib -o main.o --emit=obj --cfg=x86_32 --target=i686-unknown-linux-gnu -L . -Z no-landing-pads
	
support.o: support.rs
	$(RUSTC) -g -O support.rs --crate-type lib -o support.o --emit obj --target i686-unknown-linux-gnu -L . -Z no-landing-pads
	
run: boot.bin
	$(QEMU) -kernel $<

debug: boot.bin
	$(QEMU) -S -gdb tcp::3333 -kernel $<

thread.o: arch/x86_32/thread.s
	$(AS) -g -o $@ $<
	
boot.bin: linker.ld main.o boot.o interrupt.o thread.o support.o
	$(LD) -o $@ -T $^

iso: boot.bin
	cp boot.bin isodir/boot/
	grub-mkrescue -o boot.iso isodir

vb: iso
	virtualbox --debug --startvm rynux
	
clean:
	rm -f *.bin *.o *.img *.iso *.rlib
