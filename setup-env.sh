#!/bin/bash

# Get the directory where riscv64-unknown-elf-gcc is installed
RISCV_PATH=$(dirname $(which riscv64-unknown-elf-gcc))

# Add it to PATH if not already there
if [[ ":$PATH:" != *":$RISCV_PATH:"* ]]; then
    export PATH="$RISCV_PATH:$PATH"
fi

# Now try to run the debugger
if command -v riscv64-unknown-elf-gdb >/dev/null 2>&1; then
    echo "RISC-V GDB found at: $(which riscv64-unknown-elf-gdb)"
else
    echo "RISC-V GDB not found. Installing..."
    brew install riscv-software-src/riscv/riscv-gdb
fi

# Print status
echo "RISC-V toolchain status:"
echo "GCC: $(which riscv64-unknown-elf-gcc)"
echo "GDB: $(which riscv64-unknown-elf-gdb)"
