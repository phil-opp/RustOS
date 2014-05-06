LD=i686-elf-ld
RUSTC=rustc
QEMU=qemu-system-i386
AS=i686-elf-as

all: boot.bin

.SUFFIXES: .o .rs .s

.PHONY: clean run

.rs.o:
	$(RUSTC) -g -O --target i386-intel-linux --crate-type lib -o $@ --emit obj $< -L .

.s.o:
	$(AS) -g -o $@ $<

core: rust-core/core/lib.rs
	rustc --crate-type=lib -C passes=inline $<
	
run: boot.bin
	$(QEMU) -kernel $<

debug: boot.bin
	$(QEMU) -S -gdb tcp::3334 -kernel $<

boot.bin: linker.ld main.o boot.o runtime.o idt.o interrupt.o vga.o
	$(LD) -o $@ -T $^

iso: boot.bin
	cp boot.bin isodir/boot/
	grub-mkrescue -o boot.iso isodir

vb: iso
	virtualbox --debug --startvm rynux
	
clean:
	rm -f *.bin *.o *.img *.iso
