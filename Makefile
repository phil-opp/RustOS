CC=i686-elf-gcc
LD=i686-elf-ld
RUSTC=rustc
NASM=nasm
QEMU=qemu-system-i386

all: floppy.img

.SUFFIXES:

.SUFFIXES: .o .rs .asm

.PHONY: clean run

.rs.o:
	$(RUSTC) -g -O --target i386-intel-linux --crate-type lib -o $@ --emit obj $<

.asm.o:
	$(NASM) -g -f elf32 -o $@ $<

floppy.img: loader.bin main.bin
	cat $^ > $@

loader.bin: loader.asm
	$(NASM) -o $@ -f bin $<

main.bin: linker.ld runtime.o main.o
	$(LD) -o $@ -T $^

run: floppy.img
	$(QEMU) -fda $<

debug: floppy.img
	$(QEMU) -S -gdb tcp::3334 -fda $<

clean:
	rm -f *.bin *.o *.img
