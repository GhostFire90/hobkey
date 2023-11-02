CSRC=${wildcard src/*.c}
ASRC= ${wildcard src/*.s}
SOURCES := $(ASRC) $(CSRC)
BUILDDIR=build/src
OUTDIR=out
OBJS:= $(SOURCES:%=build/%.o)
SRCDIR=src


all: setup $(OBJS) preLD LD 

.PHONY: setup	
setup:
	@if [ ! -d ${BUILDDIR} ]; then\
		mkdir -p "${BUILDDIR}";\
	fi
	@if [ ! -d ${OUTDIR} ]; then\
		mkdir -p "${OUTDIR}";\
	fi

$(BUILDDIR)%.s.o: $(SRCDIR)/%.s
	nasm -felf64 $< -o $@
$(BUILDDIR)%.c.o: $(SRCDIR)/%.c
	clang -target x86_64-elf -ffreestanding -nostdlib -c -Wno-pointer-sign $< -o $@
	
casm:
	i686-elf-gcc -S ${CSRC}
.PHONY: preLD
preLD:
LD:
	clang -T linker.ld -no-pie -o ${OUTDIR}/kernel.bin -ffreestanding -nostdlib ${wildcard ${BUILDDIR}/*.o}
grub:
	cp ${OUTDIR}/kernel.bin iso/boot/
	grub-mkrescue -o ${OUTDIR}/boot.iso iso


.PHONY: qemu
qemu:
	qemu-system-i386 -kernel ${OUTDIR}/kernel.bin



	

