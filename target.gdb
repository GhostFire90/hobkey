target remote :1234
add-symbol-file target/x86_64-unknown-none/debug/hobkey-rs
b kmain
b main.rs:17
c