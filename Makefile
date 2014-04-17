LD=i686-elf-ld
RUSTC=rustc
QEMU=qemu-system-i386
AS=i686-elf-as

all: floppy.img

.SUFFIXES:

.SUFFIXES: .o .rs .s

.PHONY: clean run

.rs.o:
	$(RUSTC) -g -O --target i386-intel-linux --crate-type lib -o $@ --emit obj $<

.s.o:
	$(AS) -g -o $@ $<

main.bin: linker.ld runtime.o main.o
	$(LD) -g -o $@ -T $^

run: boot.bin
	$(QEMU) -kernel $<

debug: boot.bin
	$(QEMU) -S -gdb tcp::3334 -kernel $<

boot.bin: linker.ld main.o boot.o runtime.o
	$(LD) -o $@ -T $^
	
clean:
	rm -f *.bin *.o *.img *.iso
