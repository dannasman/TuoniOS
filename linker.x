__ram_start = 0x40080000;
__kernel_start = __ram_start + 0x200000;

__physical_stack = __kernel_start + 16M;
__virtual_stack = 0xffff000000000000;

__stack_len = 2M;
__kernel_code_virtual = 0xffff000000400000;

__kernel_virtual_ram_start = __kernel_code_virtual + (__kernel_start - __ram_start);

ENTRY(_start)
SECTIONS
{
    . = __kernel_start;

    .physical_boot.text : {
        __boot_start = .;
        *(.init)
        __boot_end = ABSOLUTE(.);
    }

    . = __virtual_stack;

    .stack (NOLOAD) : AT(__physical_stack)
    {
        __estack = .;
        . += __stack_len;
        __sstack = .;
    }

    __kernel_virtual_code_start = __kernel_virtual_ram_start + (__boot_end - __kernel_start);
    HIDDEN(__kernel_va_code_offset = __kernel_virtual_code_start - __boot_end);
    . = __kernel_virtual_code_start;

    .text : AT(ADDR(.text) - __kernel_va_code_offset) {
        *(.text.abort);
        *(text .text.*)
    }

    .rodata ALIGN(4) : AT(ADDR(.rodata) - __kernel_va_code_offset) {
        *(.srodata .srodata.*);
        *(.rodata .rodata.*);
        . = ALIGN(4);
    }

    .data ALIGN(8) : AT(ADDR(.data) - __kernel_va_code_offset) {
        __sidata = LOADADDR(.data);
        __sdata = .;
        PROVIDE(__global_pointer$ = . + 0x800);
        *(.sdata .sdata.* .sdata2 .sdata2.*);
        *(.data .data.*);
        . = ALIGN(8);
        __edata = .;
    }

    .bss ALIGN(8) (NOLOAD) : AT(ADDR(.data) - __kernel_va_code_offset) {
        __sbss = .;
        *(.sbss .sbss.* .bss .bss.*);
        . = ALIGN(8);
        __ebss = .;
    }
}
