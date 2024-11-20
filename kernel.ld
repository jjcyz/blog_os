OUTPUT_ARCH("riscv64")
ENTRY(_start)

MEMORY {
    /* Note: Load at 0x80200000 as shown in OpenSBI output */
    RAM : ORIGIN = 0x80200000, LENGTH = 128M
}

SECTIONS {
    . = ORIGIN(RAM);

    .text : {
        PROVIDE(_text_start = .);
        *(.text.init)          /* Startup code */
        *(.text.trap_vector)   /* Trap handler */
        *(.text .text.*)       /* Everything else */
        . = ALIGN(4K);
        PROVIDE(_text_end = .);
    } > RAM

    .rodata : {
        PROVIDE(_rodata_start = .);
        *(.rodata .rodata.*)   /* Read-only data */
        *(.srodata .srodata.*) /* Read-only small data */
        . = ALIGN(4K);
        PROVIDE(_rodata_end = .);
    } > RAM

    .data : {
        . = ALIGN(4K);
        PROVIDE(_data_start = .);
        *(.sdata .sdata.*)     /* Small data */
        *(.data .data.*)       /* Regular data */
        PROVIDE(_data_end = .);
    } > RAM

    .bss : {
        . = ALIGN(4K);
        PROVIDE(_bss_start = .);
        *(.sbss .sbss.*)       /* Small BSS */
        *(.bss .bss.*)         /* Regular BSS */
        *(COMMON)
        . = ALIGN(4K);
        PROVIDE(_bss_end = .);
    } > RAM

    .stack (NOLOAD) : {
        . = ALIGN(4K);
        PROVIDE(_stack_start = .);
        . += 64K;              /* 64KB stack */
        PROVIDE(_stack_end = .);
    } > RAM
}
