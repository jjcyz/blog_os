#!/bin/bash

# Build the kernel
echo "Building kernel..."
cargo build --target riscv64gc-unknown-none-elf

if [ $? -eq 0 ]; then
    # Start QEMU in the background
    echo "Starting QEMU..."
    qemu-system-riscv64 \
        -machine virt \
        -cpu rv64 \
        -smp 1 \
        -m 128M \
        -nographic \
        -kernel target/riscv64gc-unknown-none-elf/debug/blog_os \
        -s -S &

    # Wait a moment for QEMU to start
    sleep 1

    # Try different possible GDB binary names
    for gdb in riscv64-unknown-elf-gdb riscv64-elf-gdb gdb-multiarch; do
        if command -v $gdb >/dev/null 2>&1; then
            echo "Found GDB: $gdb"
            $gdb target/riscv64gc-unknown-none-elf/debug/blog_os \
                -ex "target remote localhost:1234" \
                -ex "break kernel_main" \
                -ex "continue"
            break
        fi
    done

    # If no GDB was found
    if ! command -v riscv64-unknown-elf-gdb >/dev/null 2>&1 && \
       ! command -v riscv64-elf-gdb >/dev/null 2>&1 && \
       ! command -v gdb-multiarch >/dev/null 2>&1; then
        echo "No suitable GDB found. Installing prerequisites..."
        brew install gdb
    fi

    # Clean up QEMU when done
    killall qemu-system-riscv64
else
    echo "Build failed!"
fi
