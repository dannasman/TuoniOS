_phys_binary_begin = 0x200000;

ENTRY(_phys_binary_begin)

SECTIONS {
    . = 0x2000000;

    _binary_start = .;
    .text : {
        KEEP(*(.text._start))
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data*)
    }

    . = ALIGN(8);
    _binary_end = .;

    .bss (NOLOAD) : ALIGN(16) {
        _bss_start = .;
        *(.bss*);
        . = ALIGN(16);
        _bss_end = .;
    }
}
