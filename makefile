CSRC=${wildcard src/*.c}
ASRC=${wildcard src/*.s}
BUILDDIR=build
OUTDIR=out

all: setup asm gcc preLD LD 

.PHONY: setup	
setup:
	@if [ ! -d ${BUILDDIR} ]; then\
		mkdir -p "${BUILDDIR}";\
	fi
	@if [ ! -d ${OUTDIR} ]; then\
		mkdir -p "${OUTDIR}";\
	fi

asm:
	nasm -felf32 ${ASRC}
gcc:
	i686-elf-gcc -c ${CSRC}
.PHONY: preLD
preLD:
	mv ${wildcard *.o} ${BUILDDIR}
	mv ${wildcard src/*.o} ${BUILDDIR}
LD:
	i686-elf-gcc -T linker.ld -o ${OUTDIR}/kernel.bin -ffreestanding -nostdlib ${wildcard ${BUILDDIR}/*.o}
grub:
	cp ${OUTDIR}/kernel.bin iso/boot/
	grub-mkrescue -o ${OUTDIR}/boot.iso iso


.PHONY: qemu
qemu:
	qemu-system-i386 -kernel ${OUTDIR}/kernel.bin



	

