.section .text.init
.global _start

_start:
    # Disable all interrupts
    csrw mie, zero
    csrw mip, zero

    # Set up stack pointer
    la sp, _stack_end

    # Clear BSS section
    la t0, _bss_start
    la t1, _bss_end
clear_bss:
    bgeu t0, t1, clear_bss_done
    sd zero, (t0)
    addi t0, t0, 8
    j clear_bss
clear_bss_done:

    # Jump to Rust code
    tail kernel_main

    # Should never return, but if it does, loop forever
1:  wfi
    j 1b

.section .bss.stack
.align 12
.global _stack_start
_stack_start:
    .zero 4096 * 4  # 16KB stack
.global _stack_end
_stack_end:
