# TODO(ryan): I've changed as/ld to native cross compiler ones.
# Could this cause dependency problems in the future?
AS=as -march=i386 --32
LD=ld -melf_i386 -nostdlib
RUSTC=rustc
QEMU=qemu-system-i386
TARGET=i686-unknown-linux-gnu
DYLIBFLAG=--crate-type=dylib
LIBFLAG=--crate-type=lib
RUSTFLAGS=-g -O --cfg=kernel --target $(TARGET) -C link-args="--verbose" -L . -Z no-landing-pads

all: boot.bin

.SUFFIXES: .o .s .rlib .a .so

.PHONY: clean run

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
	$(QEMU) -kernel $<

debug: boot.bin
	$(QEMU) -S -gdb tcp::3333 -kernel $<

%.o: arch/x86/%.s
	$(AS) -g -o $@ $<
	
boot.bin: linker.ld main.o boot.o interrupt.o thread.o rlibc.o dependencies.o
	$(LD) -o $@ -T $^  *.so

iso: boot.bin
	cp boot.bin isodir/boot/
	grub-mkrescue -o boot.iso isodir

vb: iso
	virtualbox --debug --startvm rustos
	
clean:
	rm -f *.bin *.o *.img *.iso *.rlib *.a *.so
