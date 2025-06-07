ENTRY(_start)

_peripherals_begin = 0xfe000000;

SECTIONS
{
    . = 0x2000000;

    .physical_boot.text : {
        _boot_start = .;
        *(.init .init.*);
        _boot_end = ABSOLUTE(.);
    }

    . += 0xffff000000000000;
    _kernel_begin = .;


    HIDDEN(_offset = _kernel_begin - _boot_end);
    .text : AT(ADDR(.text) - _offset) {
        *(text .text.*);
    }

    .rodata ALIGN(4) : AT(ADDR(.rodata) - _offset) {
        *(.rodata*);
        . = ALIGN(4);
    }

    .data ALIGN(4) : AT(ADDR(.data) - _offset) {
        *(.data*);
        . = ALIGN(8);
    }

    .bss ALIGN(4)  : AT(ADDR(.bss) - _offset) {
        _bss_start = .;
        *(.bss .bss.*);
        . = ALIGN(4);
        _bss_end = .;
    }

    _kernel_end = .;

    . = ALIGN(0x1000);
    _early_heap_begin = .;
    . += 0x100000;
    _early_heap_end = .;
    
    . = ALIGN(0x1000);
    _stack_begin = .;
    . += 0x4000;
    _stack_end = .;
}
