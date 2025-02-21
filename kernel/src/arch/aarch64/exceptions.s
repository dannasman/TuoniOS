.section ".text.exception_table"
.global vector_table_el1

exception_entry:
    sub     sp, sp, 192
    stp     x0, x1, [sp, 0]
    stp     x2, x3, [sp, 16]
    stp     x4, x5, [sp, 32]
    stp     x6, x7, [sp, 48]
    stp     x8, x9, [sp, 64]
    stp     x10, x11, [sp, 80]
    stp     x12, x13, [sp, 96]
    stp     x14, x15, [sp, 112]
    stp     x16, x17, [sp, 128]
    stp     x18, x29, [sp, 144]
    stp     x30, x1, [sp, 160]

    mrs     x0, esr_el1
    mrs     x1, far_el1
    stp     x0, x1, [sp, 176]

    mov     x0, sp
    ldr     x20, =exception
    blr     x20

    ldp     x2, x3, [sp, 16]
    ldp     x4, x5, [sp, 32]
    ldp     x6, x7, [sp, 48]
    ldp     x8, x9, [sp, 64]
    ldp     x10, x11, [sp, 80]
    ldp     x12, x13, [sp, 96]
    ldp     x14, x15, [sp, 112]
    ldp     x16, x17, [sp, 128]
    ldp     x18, x29, [sp, 144]
    ldp     x30, x0, [sp, 160]
    mov     x1, sp
    mov     sp, x0
    ldp     x0, x1, [x1, 0]

    eret

interrupt_entry:
    sub     sp, sp, 192
    stp     x0, x1, [sp, 0]
    stp     x2, x3, [sp, 16]
    stp     x4, x5, [sp, 32]
    stp     x6, x7, [sp, 48]
    stp     x8, x9, [sp, 64]
    stp     x10, x11, [sp, 80]
    stp     x12, x13, [sp, 96]
    stp     x14, x15, [sp, 112]
    stp     x16, x17, [sp, 128]
    stp     x18, x29, [sp, 144]
    stp     x30, x1, [sp, 160]

    stp     xzr, xzr, [sp, 176]

    mov     x0, sp
    ldr     x20, =interrupt
    blr     x20

    ldp     x2, x3, [sp, 16]
    ldp     x4, x5, [sp, 32]
    ldp     x6, x7, [sp, 48]
    ldp     x8, x9, [sp, 64]
    ldp     x10, x11, [sp, 80]
    ldp     x12, x13, [sp, 96]
    ldp     x14, x15, [sp, 112]
    ldp     x16, x17, [sp, 128]
    ldp     x18, x29, [sp, 144]
    ldp     x30, x0, [sp, 160]
    mov     x1, sp
    mov     sp, x0
    ldp     x0, x1, [x1, 0]

    eret

.balign 0x1000
vector_table_el1:
    b       exception_entry
    .balign 0x80
    b       interrupt_entry
    .balign 0x80
    b       interrupt_entry
    .balign 0x80
    b       exception_entry
    .balign 0x80
    b       exception_entry
    .balign 0x80
    b       interrupt_entry
    .balign 0x80
    b       interrupt_entry
    .balign 0x80
    b       exception_entry
    .balign 0x80
    b       exception_entry
    .balign 0x80
    b       interrupt_entry
    .balign 0x80
    b       interrupt_entry
    .balign 0x80
    b       exception_entry
    .balign 0x80
    b       exception_entry
    .balign 0x80
    b       interrupt_entry
    .balign 0x80
    b       interrupt_entry
    .balign 0x80
    b       exception_entry

