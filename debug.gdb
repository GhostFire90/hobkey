add-symbol-file out/kernel.bin 0xffffffff80000000
target remote localhost:1234
break kernel_main
layout src