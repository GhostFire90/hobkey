CSRC=${wildcard src/*.c}
ASRC=${wildcard src/*.s}
BUILDDIR=build
OUTDIR=out

all: setup asm cc preLD LD 

.PHONY: setup	
setup:
	@if [ ! -d ${BUILDDIR} ]; then\
		mkdir -p "${BUILDDIR}";\
	fi
	@if [ ! -d ${OUTDIR} ]; then\
		mkdir -p "${OUTDIR}";\
	fi

asm:
	nasm -felf64 ${ASRC}
cc:
	clang -target x86_64-elf -ffreestanding -nostdlib -c ${CSRC}
casm:
	i686-elf-gcc -S ${CSRC}
.PHONY: preLD
preLD:
	mv ${wildcard *.o} ${BUILDDIR}
	mv ${wildcard src/*.o} ${BUILDDIR}
LD:
	clang -T linker.ld -no-pie -o ${OUTDIR}/kernel.bin -ffreestanding -nostdlib ${wildcard ${BUILDDIR}/*.o}
grub:
	cp ${OUTDIR}/kernel.bin iso/boot/
	grub-mkrescue -o ${OUTDIR}/boot.iso iso


.PHONY: qemu
qemu:
	qemu-system-i386 -kernel ${OUTDIR}/kernel.bin



	

