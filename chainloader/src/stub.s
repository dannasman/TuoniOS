.section .text._start
.global _start

_start:
    mrs     x0, mpidr_el1
    and     x0, x0, 0xff;
    cbz     x0, 1f
0:
    wfe
    b       0b
1:
    ldr     x0, =_stack_top
    mov     sp, x0
    bl      chainloader_main
2:
    b 2b

.section .bss
.align 12
_stack:
    .space 0x1000
_stack_top:
