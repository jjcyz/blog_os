OUTPUT_ARCH(riscv)
ENTRY(_start)

SECTIONS
{
    . = 0x80000000;

    .text : {
        *(.text.init)    /* Put boot code first */
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        . = ALIGN(4096);
        *(.sdata .sdata.*)
        *(.data .data.*)
    }

    .bss : {
        . = ALIGN(4096);
        *(.sbss .sbss.*)
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(4096);
    }

    .stack : {
        . = ALIGN(4096);
        . += 4096 * 4;  /* 16KB stack */
        PROVIDE(_stack_end = .);
    }

    /DISCARD/ : {
        *(.eh_frame*)
    }
}
