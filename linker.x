ENTRY(_start)
SECTIONS
{
    . = 0x40080000;
    .text : { *(.text) }
    .data : { *(.data) }
    .bss : { *(.bss) }

    /* Stack */
    . = ALIGN(8);
    . = . + 0x4000;
    stack_top = .;
}
