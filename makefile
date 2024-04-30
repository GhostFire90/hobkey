CSRC=${wildcard src/*.c}
ASRC= ${wildcard src/*.s}
SOURCES := $(ASRC) $(CSRC)
BUILDDIR=build/src
OUTDIR=out
OBJS:= $(SOURCES:%=build/%.o)
SRCDIR=src
LIMINE_ROOT = limine_iso
LIMINE_INSTALL_DIR = /usr/local/share/limine


all: $(BUILDDIR) $(OUTDIR) $(OBJS) preLD LD limine 

${BUILDDIR}:
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



limine: LIMINE_SETUP ramdisc
	@cp ${OUTDIR}/kernel.bin ${LIMINE_ROOT}/boot
	xorriso -as mkisofs -b limine-bios-cd.bin \
        -no-emul-boot -boot-load-size 4 -boot-info-table \
        --efi-boot limine-uefi-cd.bin \
        -efi-boot-part --efi-boot-image --protective-msdos-label \
        ${LIMINE_ROOT} -o ${OUTDIR}/boot.iso
LIMINE_SETUP:
	@if [ ! -d "${LIMINE_ROOT}" ]; then \
		mkdir -p "${LIMINE_ROOT}"/EFI/BOOT; \
		cp "${LIMINE_INSTALL_DIR}"/limine-uefi-cd.bin "${LIMINE_ROOT}"; \
		cp "${LIMINE_INSTALL_DIR}"/limine-bios-cd.bin "${LIMINE_ROOT}"; \
		cp "${LIMINE_INSTALL_DIR}"/limine-bios.sys "${LIMINE_ROOT}"; \
		cp "${LIMINE_INSTALL_DIR}"/BOOTX64.EFI "${LIMINE_ROOT}"/EFI/BOOT; \
		cp limine.cfg "${LIMINE_ROOT}"; \
		mkdir "${LIMINE_ROOT}"/boot; \
	fi
		

.PHONY: qemu
qemu:
	qemu-system-x86_64 -bios OVMF.fd -m 2G -cdrom ${OUTDIR}/boot.iso -no-reboot -no-shutdown -D qemu_log.txt

.PHONY: qemu_gdb
qemu_gdb:
	qemu-system-x86_64 -cdrom out/boot.iso -bios OVMF.fd -m 2G -s -S -d int -D qemu_log.txt -M smm=off &
	gdb -x debug.gdb

mkimg:
	sh mkimg.sh > /dev/null

.PHONY: clean
clean :
	rm -r build/
	rm -r out/
	rm -r limine_iso

ramdisc: LIMINE_SETUP
	@cd initrd && tar -cvf ../${LIMINE_ROOT}/boot/initrd.tar --format=ustar * && cd -



	

