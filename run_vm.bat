
cargo build --target custom-os-x86_64.json

cargo bootimage 

qemu-system-x86_64 -drive format=raw,file=target/custom-os-x86_64/debug/bootimage-kernel.bin