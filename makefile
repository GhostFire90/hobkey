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
	nasm -felf64 -Fdwarf -g $< -o $@
$(BUILDDIR)%.c.o: $(SRCDIR)/%.c
	clang -target x86_64-elf -g -ffreestanding -nostdlib -c -Wno-pointer-sign $< -o $@
	
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
	qemu-system-x86_64 -bios OVMF.fd -m 2G -cdrom out/boot.img

mkimg:
	sh mkimg.sh > /dev/null

.PHONY: clean
clean :
	rm -r build/



	

