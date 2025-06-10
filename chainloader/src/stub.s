.macro ld_abs reg, adr
movz    \reg, #:abs_g2:\adr
movk    \reg, #:abs_g1_nc:\adr
movk    \reg, #:abs_g0_nc:\adr
.endm

.macro ld_rel reg, adr
adrp    \reg, \adr
add     \reg, \reg, #:lo12:\adr
.endm

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
    ld_abs  x0, _bss_start
    ld_abs  x1, _bss_end
clear_bss:
    cmp     x0, x1
    b.eq    copy_binary
    stp     xzr, xzr, [x0], 16
    b       clear_bss
copy_binary:
    ld_rel  x0, _binary_start
    ld_abs  x1, _binary_start
    ld_abs  x2, _binary_end
copy_loop:
    ldr     x3, [x0], 8
    str     x3, [x1], 8
    cmp     x1, x2
    b.lo    copy_loop
    ld_abs  x0, _stack_top
    mov     sp, x0
    ld_abs  x1, chainloader_main
    br      x1
2:
    b 2b

.section .bss
.align 12
_stack:
    .space 0x1000
_stack_top:
