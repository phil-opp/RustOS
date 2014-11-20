# TODO(ryan): I've changed as/ld to native cross compiler ones.
# Could this cause dependency problems in the future?
AS=as -march=i386 --32
LD=ld -melf_i386 -nostdlib
RUSTC=rustc
QEMU=qemu-system-i386
TARGET=i686-unknown-linux-gnu
DYLIBFLAG=--crate-type=dylib --emit=obj
LIBFLAG=--crate-type=lib
RUSTFLAGS=-g --cfg=kernel --target $(TARGET) -L . -Z no-landing-pads
QEMUARGS=-device rtl8139,vlan=0 -net user,id=net0,vlan=0 -net dump,vlan=0,file=/tmp/rustos-dump.pcap

all: boot.bin

.SUFFIXES: .o .s .rlib .a .so

.PHONY: clean cleanproj run debug vb

.s.o:
	$(AS) -g -o $@ $<

# rustc's call to ld requires libcompiler-rt.a, but apparently
# nothing in it is used, so we'll just as an empty file 
libcompiler-rt.a: dummy-compiler-rt.s # needed for dylib creation
	$(AS) $< -o $@

libmorestack.a: rust/src/rt/arch/i386/morestack.S # needed for dylib creation
	$(AS) $< -o $@

%.rlib: rust/src/%/*.rs rust/src/%/lib.rs
	$(RUSTC) rust/src/%/lib.rs $(RUSTFLAGS)

liblazy_static.rlib: lazy-static/src/lazy_static.rs libstd.rlib
	$(RUSTC) $< $(RUSTFLAGS) $(LIBFLAG)

liballoc.rlib: rust/src/liballoc/lib.rs libcore.rlib liblibc.rlib
	$(RUSTC) $< $(RUSTFLAGS) $(LIBFLAG)
	$(RUSTC) $< $(RUSTFLAGS) $(DYLIBFLAG)

libcore.rlib: rust/src/libcore/lib.rs libmorestack.a libcompiler-rt.a
	$(RUSTC) $< $(RUSTFLAGS) $(LIBFLAG)
	$(RUSTC) $< $(RUSTFLAGS) $(DYLIBFLAG)

libcollections.rlib: rust/src/libcollections/lib.rs libcore.rlib liballoc.rlib libunicode.rlib
	$(RUSTC) $< $(RUSTFLAGS) $(LIBFLAG)
	$(RUSTC) $< $(RUSTFLAGS) $(DYLIBFLAG)

librand.rlib: rust/src/librand/lib.rs libcore.rlib
	$(RUSTC) $< $(RUSTFLAGS) $(LIBFLAG)
	$(RUSTC) $< $(RUSTFLAGS) $(DYLIBFLAG)

libstd.rlib: rust/src/libstd/lib.rs libcore.rlib liballoc.rlib libcollections.rlib librand.rlib liblibc.rlib librustrt.rlib
	$(RUSTC) $< $(RUSTFLAGS) $(LIBFLAG)
	$(RUSTC) $< $(RUSTFLAGS) $(DYLIBFLAG)

liblibc.rlib: rust/src/liblibc/lib.rs libmorestack.a libcompiler-rt.a
	$(RUSTC) $< $(RUSTFLAGS) $(LIBFLAG)
	$(RUSTC) $< $(RUSTFLAGS) $(DYLIBFLAG)

librustrt.rlib: rust/src/librustrt/lib.rs libcore.rlib liballoc.rlib liblibc.rlib libcollections.rlib
	$(RUSTC) $< $(RUSTFLAGS) $(LIBFLAG)
	$(RUSTC) $< $(RUSTFLAGS) $(DYLIBFLAG)

libunicode.rlib: rust/src/libunicode/lib.rs libcore.rlib
	$(RUSTC) $< $(RUSTFLAGS) $(LIBFLAG)
	$(RUSTC) $< $(RUSTFLAGS) $(DYLIBFLAG)

main.o: main.rs libstd.rlib liballoc.rlib libcore.rlib libcollections.rlib liblazy_static.rlib
	$(RUSTC) $< -o $@ --emit obj $(RUSTFLAGS) $(LIBFLAG)

rlibc.o: rust/src/librlibc/lib.rs 
	$(RUSTC) $< -o $@ --emit obj $(RUSTFLAGS) $(LIBFLAG)

run: boot.bin
	$(QEMU) $(QEMUARGS) -kernel $<

debug: boot.bin
	$(QEMU) $(QEMUARGS) -S -gdb tcp::3333 -kernel $< &
	gdb $< -ex "target remote :3333" -ex "break _start" -ex "c"

%.o: arch/x86/%.s
	$(AS) -g -o $@ $<

boot.bin: linker.ld main.o boot.o interrupt.o thread.o rlibc.o dependencies.o libstd.rlib
	$(LD) -o $@ -T $< *.o

iso: boot.bin
	cp boot.bin isodir/boot/
	grub-mkrescue -o boot.iso isodir

vb: iso
	virtualbox --debug --startvm rustos

clean: cleanproj
	rm -f *.rlib *.a *.so *.o

cleanproj:
	rm -f *.bin main.o *.img *.iso
