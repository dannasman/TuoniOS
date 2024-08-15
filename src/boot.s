.section ".text.boot"

    .org 0x80000

.globl _start

_start:
    ldr     x5, =_start
    mov     sp, x5

    ldr     x5, =__bss_start
    ldr     w6, =__bss_size
1:  cbz     w6, 2f
    str     xzr, [x5], #8
    sub     w6, w6, #1
    cbnz    w6, 1b
2:  bl      main
halt:
    wfe
    b halt

