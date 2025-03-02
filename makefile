LIMINE_ROOT=target/limine
LIMINE_INSTALL_DIR = /usr/local/share/limine
INITRD=initrd.tar


limine: iso_fs
	@cargo build
	@cp target/x86_64-unknown-none/debug/hobkey-rs ${LIMINE_ROOT}/boot
	@xorriso -as mkisofs -b limine-bios-cd.bin \
        -no-emul-boot -boot-load-size 4 -boot-info-table \
        --efi-boot limine-uefi-cd.bin \
        -efi-boot-part --efi-boot-image --protective-msdos-label \
        ${LIMINE_ROOT} -o boot.iso 1> /dev/null 2>&1

${LIMINE_ROOT}: ${INITRD}
	@mkdir -p "${LIMINE_ROOT}"/EFI/BOOT
	@mkdir -p "${LIMINE_ROOT}"/boot
	@cp initrd.tar "${LIMINE_ROOT}"/boot/

${INITRD}:
	@tar -uf initrd.tar -C initrd -H ustar .
iso_fs: ${LIMINE_ROOT}
	@cp "${LIMINE_INSTALL_DIR}"/limine-uefi-cd.bin "${LIMINE_ROOT}" 
	@cp "${LIMINE_INSTALL_DIR}"/limine-bios-cd.bin "${LIMINE_ROOT}" 
	@cp "${LIMINE_INSTALL_DIR}"/limine-bios.sys "${LIMINE_ROOT}"
	@cp "${LIMINE_INSTALL_DIR}"/BOOTX64.EFI "${LIMINE_ROOT}"/EFI/BOOT 
	@cp limine.conf "${LIMINE_ROOT}"

.PHONY: qemu
qemu: limine
	qemu-system-x86_64 -bios OVMF.fd -d int -cdrom boot.iso -no-reboot -no-shutdown -D qemu_log.txt -serial file:os_log.txt

.PHONY: qemu_db
qemu_db: limine
	qemu-system-x86_64 -bios OVMF.fd -d int -cdrom boot.iso -no-reboot -no-shutdown -D qemu_log.txt -serial file:os_log.txt -s -S &
	rust-gdb -x target.gdb 
.PHONY: clean
clean:
	@cargo clean
	rm initrd.tar
	rm boot.iso