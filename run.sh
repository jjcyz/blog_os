#!/bin/bash

# Build the kernel
echo "Building kernel..."
cargo build --target riscv64gc-unknown-none-elf

if [ $? -eq 0 ]; then
    echo "Starting QEMU..."
    echo "To exit QEMU, press Ctrl+C"
    qemu-system-riscv64 \
        -machine virt \
        -cpu rv64 \
        -smp 1 \
        -m 128M \
        -nographic \
        -monitor none \
        -kernel target/riscv64gc-unknown-none-elf/debug/blog_os
else
    echo "Build failed!"
fi
