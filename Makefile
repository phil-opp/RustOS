LD=i686-elf-ld
RUSTC=rustc
QEMU=qemu-system-i386
AS=i686-elf-as

all: boot.bin

.SUFFIXES: .o .s

.PHONY: clean run

.s.o:
	$(AS) -g -o $@ $<

main.o: core main.rs
	$(RUSTC) -g -O --cfg x86_32 --target i386-intel-linux --crate-type lib -o main.o --emit obj main.rs -L . -Z no-landing-pads

support.o: rust-core/support.rs
	$(RUSTC) -g -O --target i386-intel-linux --crate-type lib -o support.o --emit obj $< -L . -Z no-landing-pads
	
core: rust-core/core/lib.rs
	$(RUSTC) -g --crate-type=lib -C passes=inline $<  -Z no-landing-pads
	
run: boot.bin
	$(QEMU) -kernel $<

debug: boot.bin
	$(QEMU) -S -gdb tcp::3334 -kernel $<

boot.bin: linker.ld main.o boot.o interrupt.o support.o
	$(LD) -o $@ -T $^

iso: boot.bin
	cp boot.bin isodir/boot/
	grub-mkrescue -o boot.iso isodir

vb: iso
	virtualbox --debug --startvm rynux
	
clean:
	rm -f *.bin *.o *.img *.iso *.rlib
