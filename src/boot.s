.section ".text.boot"
.extern __stack_top

.globl _start

_start:
    mrs     x0, CurrentEL
    cmp     x0, 0b0100
    beq     in_el1
    blo     in_el0
    cmp     x0, 0b1000
    beq     in_el2
in_el3:
    msr     sctlr_el2, xzr
    msr     hcr_el2, xzr

    mrs     x0, scr_el3
    orr     x0, x0, (1 << 10)
    orr     x0, x0, (1 << 0)
    and     x0, x0, ~(1 << 3)
    and     x0, x0, ~(1 << 2)
    and     x0, x0, ~(1 << 1)
    msr     scr_el3, x0
    mov     x0, 0b01001
    msr     spsr_el3, x0

    adr     x0, in_el2
    msr     ELR_EL3, x0
    eret
in_el2:
    msr     sctlr_el1, xzr
    mrs     x0, HCR_EL2
    orr     x0, x0, (1 << 31)
    and     x0, x0, ~(1 << 5)
    and     x0, x0, ~(1 << 4)
    and     x0, x0, ~(1 << 3)
    msr     hcr_el2, x0
    mov     x0, 0b00101
    msr     spsr_el2, x0

    adr     x0, in_el1
    msr     elr_el2, x0
    eret
in_el1:
    mov     x0, 0b0101
    msr     spsr_el1, x0

    msr     DAIFSet, 0b1111
    //msr     DAIFClr, 0b1111

    ldr     x1, =vector_table_el1
    msr     vbar_el1, x1
    
    ldr     x30, =__stack_top
    mov     sp, x30
    msr     SP_EL1, x30

    bl      kernel_main
in_el0:
    b .

exception_entry:
    sub sp, sp, #192
    stp x0, x1, [sp, #0]
    stp x2, x3, [sp, #16]
    stp x4, x5, [sp, #32]
    stp x6, x7, [sp, #48]
    stp x8, x9, [sp, #64]
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    stp x18, x29, [sp, #144]
    stp x30, xzr, [sp, #160]

    mrs x0, esr_el1
    mrs x1, far_el1
    stp x0, x1, [sp, #176]

    mov x0, sp
    bl exception

    ldp x0, x1, [sp, #0]
    ldp x2, x3, [sp, #16]
    ldp x4, x5, [sp, #32]
    ldp x6, x7, [sp, #48]
    ldp x8, x9, [sp, #64]
    ldp x10, x11, [sp, #80]
    ldp x12, x13, [sp, #96]
    ldp x14, x15, [sp, #112]
    ldp x16, x17, [sp, #128]
    ldp x18, x29, [sp, #144]
    ldp x30, xzr, [sp, #160]
    add sp, sp, #192
    eret

interrupt_entry:
    sub sp, sp, #192
    stp x0, x1, [sp, #0]
    stp x2, x3, [sp, #16]
    stp x4, x5, [sp, #32]
    stp x6, x7, [sp, #48]
    stp x8, x9, [sp, #64]
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    stp x18, x29, [sp, #144]
    stp x30, xzr, [sp, #160]

    mov x0, xzr
    mov x1, xzr
    stp x0, x1, [sp, #176]

    mov x0, sp
    bl exception

    ldp x0, x1, [sp, #0]
    ldp x2, x3, [sp, #16]
    ldp x4, x5, [sp, #32]
    ldp x6, x7, [sp, #48]
    ldp x8, x9, [sp, #64]
    ldp x10, x11, [sp, #80]
    ldp x12, x13, [sp, #96]
    ldp x14, x15, [sp, #112]
    ldp x16, x17, [sp, #128]
    ldp x18, x29, [sp, #144]
    ldp x30, xzr, [sp, #160]
    add sp, sp, #192
    eret

.balign 0x800
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

.equ PSCI_SYSTEM_OFF, 0x84000008
.globl system_off
system_off:
    ldr     x0, =PSCI_SYSTEM_OFF
    hvc     #0
