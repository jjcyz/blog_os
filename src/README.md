# Blog OS - A RISC-V Operating System in Rust

This is a learning project for building a RISC-V operating system in Rust, following the "Writing an OS in Rust" blog series.

## Project Status

Current Progress:
1. ✅ Successfully set up interrupt handling in OS
2. 🔄 Chapter 2: The Batch System (In Progress)
3. ⏳ Chapter 3: Multichannel Programming and Time-Sharing Multitasking
4. ⏳ Chapter 4: Address Spaces
5. ⏳ Chapter 5: Processes and Process Management
6. ⏳ Chapter 6: File Systems and I/O Redirection
7. ⏳ Chapter 7: Interprocess Communication
8. ⏳ Chapter 8: Concurrency

## Prerequisites

Before running this project, you need to install:

1. Rust (nightly version)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup install nightly
   rustup override set nightly
   rustup target add riscv64gc-unknown-none-elf
   ```

2. QEMU (for RISC-V emulation)
   ```bash
   brew install qemu
   ```

3. RISC-V Toolchain
   ```bash
   brew install riscv-software-src/riscv/riscv-tools
   ```

## Running the OS

1. Set up the environment:
   ```bash
   chmod +x setup-env.sh
   ./setup-env.sh
   ```

2. Build and run the OS:
   ```bash
   chmod +x run.sh
   ./run.sh
   ```

3. For debugging:
   ```bash
   chmod +x debug.sh
   ./debug.sh
   ```

## Project Structure

- `src/` - Source code directory
  - `main.rs` - Entry point of the OS
  - `lib.rs` - Core library code
  - `interrupts.rs` - Interrupt handling
  - `memory.rs` - Memory management
  - `uart.rs` - UART communication
  - `vga_buffer.rs` - VGA output handling
  - `scheduler.rs` - Task scheduling
  - `task.rs` - Task management
  - `batch_system.rs` - Batch processing system

## Current Features

- Basic interrupt handling
- UART communication
- VGA text output
- Memory management
- Task scheduling framework

## Next Steps

1. Implement batch system for running programs
2. Add time-sharing multitasking
3. Implement address spaces
4. Add process management
5. Implement file system
6. Add interprocess communication
7. Implement concurrency features
