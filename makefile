CSRC=${wildcard src/*.c src/*/*.c}
ASRC= ${wildcard src/*.s}
SOURCES := $(ASRC) $(CSRC)
BUILDDIR=build/
OUTDIR=out
OBJS:= $(patsubst %.c,$(BUILDDIR)%.o,${notdir ${CSRC}}) $(patsubst %.s,$(BUILDDIR)%.o,${notdir ${ASRC}})

LIMINE_ROOT = limine_iso
LIMINE_INSTALL_DIR = /usr/local/share/limine
MEMORY=3G
LIBS=

all: sc

sc:
	@scons
limine: ramdisc LIMINE_SETUP
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
qemu: limine
	qemu-system-x86_64 -bios OVMF.fd -m $(MEMORY) -cdrom ${OUTDIR}/boot.iso -no-reboot -no-shutdown -D qemu_log.txt

.PHONY: qemu_gdb
qemu_gdb: limine
	qemu-system-x86_64 -cdrom out/boot.iso -bios OVMF.fd -m $(MEMORY) -s -S -d int -D qemu_log.txt -M smm=off &
	gdb -x debug.gdb


.PHONY: clean
clean :
	scons -c
	rm -r build/
	rm -r out/
	rm -r limine_iso

ramdisc: LIMINE_SETUP
	@cd initrd && tar -cvf ../${LIMINE_ROOT}/boot/initrd.tar --format=ustar * 1> /dev/null 2>&1 && cd ../



	

