
Hobkey is an x86_64 kernel I am currently writing as a side project. It originally started as a playground using bios interrupt calls, and eventually spread to trying to make a bootloader. I stepped away from that idea and just dove into making a full blown kernel (using the [limine](https://codeberg.org/Limine/Limine) protocol) in C. About a year later I decided to rewrite it in Rust to grow my understanding of the language as well as to get some more creature-comforts that weren't available in C.

**Technical:**  
Arch: x86_64
Tested in qemu

Current features:
 - PML4 Paging
 - USTAR file reading for initrd purposes
 - Panic handling
 - Free-list allocator
 - Serial IO
 - Dynamic mapping of arbitrary physical pages
 - ACPI parsing
 - PSF font parsing

Missing features (features I have yet to port to the rust kernel):
 - Frame-buffer print functionality

<!-- more -->

## Build

### Prereqs
 - [Follow the Limine Install guide](https://github.com/limine-bootloader/limine#installing-limine-binaries)  
 - qemu (for emulated running)
 - lldb (for debugging)
 - make
 - libisoburn/xorriso

### steps
Run:
make or make all to compile the binary kernel
then for quickstart run make limine qemu and it will run in qemu if installed


## Acknowledgments
As mentioned previously this uses the limine protocol and my image creation uses the limine bootloader, you can find those [here](https://github.com/limine-bootloader/limine) 


## Screenshots

![Memory layout](info_mem.png)