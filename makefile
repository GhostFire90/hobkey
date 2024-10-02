CSRC=${wildcard src/*.c src/*/*.c}
ASRC= ${wildcard src/*.s}
SOURCES := $(ASRC) $(CSRC)
BUILDDIR=build/
OUTDIR=out
OBJS:= $(patsubst %.c,$(BUILDDIR)%.o,${notdir ${CSRC}}) $(patsubst %.s,$(BUILDDIR)%.o,${notdir ${ASRC}})
SRCDIR=src/
LIMINE_ROOT = limine_iso
LIMINE_INSTALL_DIR = /usr/local/share/limine
MEMORY=3G
LIBS=

all: $(BUILDDIR) $(OUTDIR) LD limine 

${BUILDDIR}:
	echo ${OBJS}
	mkdir -p ${BUILDDIR}
${OUTDIR}:
	mkdir -p ${OUTDIR}

.PHONY: setup	
setup:
	@if [ ! -d ${BUILDDIR} ]; then\
		mkdir -p "${BUILDDIR}";\
	fi
	@if [ ! -d ${OUTDIR} ]; then\
		mkdir -p "${OUTDIR}";\
	fi

$(BUILDDIR)%.o: $(SRCDIR)%.s
	@echo "[Assembling] $< -> $@"
	@nasm -felf64 -Fdwarf -g $< -o $@
$(BUILDDIR)%.o: $(SRCDIR)%.c
	@echo "[Compiling] $< -> $@"
	@clang -target x86_64-elf -g -O0 -ffreestanding -nostdlib -mno-red-zone -c -Wno-pointer-sign $< -o $@
	
casm:
	i686-elf-gcc -S ${CSRC}
.PHONY: preLD
preLD:
LD: $(OBJS)
	@echo [Linking] kernel.bin
	@clang -T linker.ld -no-pie -o ${OUTDIR}/kernel.bin -ffreestanding -nostdlib ${wildcard ${BUILDDIR}/*.o} $(LIBS)



limine: LIMINE_SETUP ramdisc
	@cp ${OUTDIR}/kernel.bin ${LIMINE_ROOT}/boot
	@xorriso -as mkisofs -b limine-bios-cd.bin \
        -no-emul-boot -boot-load-size 4 -boot-info-table \
        --efi-boot limine-uefi-cd.bin \
        -efi-boot-part --efi-boot-image --protective-msdos-label \
        ${LIMINE_ROOT} -o ${OUTDIR}/boot.iso 1> /dev/null 2>&1

${LIMINE_ROOT}:
	@mkdir -p "${LIMINE_ROOT}"/EFI/BOOT
	@mkdir -p "${LIMINE_ROOT}"/boot

LIMINE_SETUP: ${LIMINE_ROOT}
	@cp "${LIMINE_INSTALL_DIR}"/limine-uefi-cd.bin "${LIMINE_ROOT}" 
	@cp "${LIMINE_INSTALL_DIR}"/limine-bios-cd.bin "${LIMINE_ROOT}" 
	@cp "${LIMINE_INSTALL_DIR}"/limine-bios.sys "${LIMINE_ROOT}"
	@cp "${LIMINE_INSTALL_DIR}"/BOOTX64.EFI "${LIMINE_ROOT}"/EFI/BOOT 
	@cp limine.cfg "${LIMINE_ROOT}"
		

.PHONY: qemu
qemu: LD
	qemu-system-x86_64 -bios OVMF.fd -m $(MEMORY) -cdrom ${OUTDIR}/boot.iso -no-reboot -no-shutdown -D qemu_log.txt

.PHONY: qemu_gdb
qemu_gdb: LD
	qemu-system-x86_64 -cdrom out/boot.iso -bios OVMF.fd -m $(MEMORY) -s -S -d int -D qemu_log.txt -M smm=off &
	gdb -x debug.gdb

mkimg:
	@sh mkimg.sh 1> /dev/null 2>&1

.PHONY: clean
clean :
	rm -r build/
	rm -r out/
	rm -r limine_iso

ramdisc: LIMINE_SETUP
	@cd initrd && tar -cvf ../${LIMINE_ROOT}/boot/initrd.tar --format=ustar * 1> /dev/null 2>&1 && cd ../



	

