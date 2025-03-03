# Hobkey

# OLD VERSION
This is the old version of the kernel back when i was writing it in C, I've decided as an experiment to switch over to rust (the `main` branch), this eventually will be behind the `main` branch in features.
this is ***NOT*** a fully formed kernel



## Desc
Hobkey is a playground for messing around with kernel level things for me, a learning environment

## Build

### Prereq
[Follow the Limine Install guide](https://github.com/limine-bootloader/limine#installing-limine-binaries)

### steps
Run:
`make` or `make all` to compile the binary kernel
then for quickstart run `make limine qemu` and it will run in qemu if installed


## Acknowledgments
As mentioned previously this uses the limine protocol and my image creation uses the limine bootloader, you can find those [here](https://github.com/limine-bootloader/limine) 
