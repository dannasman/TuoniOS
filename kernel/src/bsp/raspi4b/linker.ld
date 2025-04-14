_peripherals_begin = 0xfe000000;

_phys_binary_begin = 0x80000;
_binary_begin = 0x4000000;


ENTRY(_binary_begin)

SECTIONS
{
    . = _binary_begin;

    .physical_boot.text : {
        _boot_start = .;
        KEEP(*(.init .init.*));
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

    .data ALIGN(16) : AT(ADDR(.data) - _offset) {
        _data_start = .;
        *(.data*);
        . = ALIGN(16);
        _data_end = .;
    }

    _binary_end = _data_end - _offset;

    .bss ALIGN(16)  : AT(ADDR(.bss) - _offset) {
        _bss_start = .;
        *(.bss .bss.*);
        . = ALIGN(16);
        _bss_end = .;
    }

    _bss_begin_phys = _bss_start - _offset;
    _bss_end_phys = _bss_end - _offset;

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

