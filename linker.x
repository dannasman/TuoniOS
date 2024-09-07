__ram_start = 0x0;
__kernel_start = __ram_start + 0x40080000;


__virtual_ram_start = 0xffff000000000000;
__virtual_kernel_start = __virtual_ram_start + __kernel_start;

SECTIONS
{
    . = __kernel_start;

    .physical_boot.text : {
        __boot_start = .;
        *(.init)
        __boot_end = ABSOLUTE(.);
    }

    . = __virtual_kernel_start;

    /*.stack (NOLOAD) : AT(__physical_stack)
    {
        __estack = .;
        . += __stack_len;
        __sstack = .;
    }*/

    __virtual_code_start = __virtual_kernel_start + (__boot_end - __kernel_start);
    HIDDEN(__kernel_va_code_offset = __virtual_kernel_start - __kernel_start - __boot_end);

    . = ALIGN(0x1000);
    .text : AT(ADDR(.text) - __kernel_va_code_offset) {
        *(.text.exception);
        *(.text.abort);
        *(text .text.*);
        . = ALIGN(0x1000);
    }

    .rodata ALIGN(4) : AT(ADDR(.rodata) - __kernel_va_code_offset) {
        *(.srodata .srodata.*);
        *(.rodata .rodata.*);
        . = ALIGN(0x1000);
    }

    .data ALIGN(8) : AT(ADDR(.data) - __kernel_va_code_offset) {
        __sidata = LOADADDR(.data);
        __sdata = .;
        *(.sdata .sdata.* .sdata2 .sdata2.*);
        *(.data .data.*);
        . = ALIGN(0x1000);
        __edata = .;
    }

    .bss ALIGN(8) (NOLOAD) : AT(ADDR(.data) - __kernel_va_code_offset) {
        __sbss = .;
        *(.sbss .sbss.* .bss .bss.*);
        . = ALIGN(0x1000);
        __ebss = .;
    }
    __stack_start = .;
}
