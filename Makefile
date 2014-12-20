# TODO(ryan): I've changed as/ld to native cross compiler ones.
# Could this cause dependency problems in the future?
AS=as -march=i386 --32
LD=ld -melf_i386 -nostdlib
QEMU=qemu-system-i386
TARGET=i686-unknown-linux-gnu
QEMUARGS=-device rtl8139,vlan=0 -net user,id=net0,vlan=0 -net dump,vlan=0,file=/tmp/rustos-dump.pcap

all: boot.bin

.SUFFIXES: .o .s .rlib .a .so

.PHONY: clean cleanproj run debug vb

.s.o:
	$(AS) -g -o $@ $<

.o.a:
	ar rcs $@ $<
	
run: boot.bin
	$(QEMU) $(QEMUARGS) -kernel $<

debug: boot.bin
	$(QEMU) $(QEMUARGS) -S -gdb tcp::3333 -kernel $< &
	gdb $< -ex "target remote :3333" -ex "break _start" -ex "c"

%.o: src/arch/x86/%.s
	$(AS) -g -o $@ $<

target/$(TARGET)/librustos*.a: Cargo.toml libmorestack.a libcompiler-rt.a 
	cargo build --target $(TARGET) --verbose
	
boot.bin: src/linker.ld boot.o target/$(TARGET)/librustos*.a interrupt.o context.o dependencies.o
	$(LD) -o $@ -T $^

boot.iso: boot.bin
	cp boot.bin src/isodir/boot/
	grub-mkrescue -o boot.iso src/isodir

vb: boot.iso
	virtualbox --debug --startvm rustos

clean: cleanproj
	cargo clean

cleanproj:
	cargo clean -p rustos
	rm -f *.bin *.img *.iso *.rlib *.a *.so *.o

libcompiler-rt.o: src/dummy-compiler-rt.s # needed for staticlib creation
	$(AS) $< -o $@
	
libmorestack.o: src/rust/src/rt/arch/i386/morestack.S # needed for staticlib creation
	$(AS) $< -o $@

