.section ".init"
.globl _start
.extern __physical_stack

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

    // TOOD: msr     SP_EL1, x30 (causes exception)

    ldr     x0, =page_table_l0
    ldr     x1, =page_table_l1
    ldr     x2, =page_table_l2
    ldr     x3, =page_table_l3

    mov     x4, 0
    stp     x4, x4, [x0]
    stp     x4, x4, [x1]
    stp     x4, x4, [x2]
    stp     x4, x4, [x3]

    ldr     x5, =page_table_l1
    orr     x5, x5, 0x3
    str     x5, [x0]

    ldr     x5, =page_table_l2
    orr     x5, x5, 0x3
    str     x5, [x1]

    ldr     x5, =page_table_l3
    orr     x5, x5, 0x3
    str     x5, [x2]

    ldr     x6, =__ram_start
    ldr     x7, =__kernel_code_virtual
    orr     x6, x6, 0x3
    lsr     x7, x7, 12
    and     x7, x7, 0x1ff
    str     x6, [x3, x7, LSL 3]

    ldr     x6, =__physical_stack
    ldr     x7, =__virtual_stack
    orr     x6, x6, 0x3
    lsr     x7, x7, 12
    and     x7, x7, 0x1ff
    str     x6, [x3, x7, LSL 3]

    ldr     x0, =page_table_l0
    msr     ttbr1_el1, x0
    isb

    ldr     x0, =0xFF000004FF
    msr     mair_el1, x0

    mov     x0, (16 << 0)
    orr     x0, x0, (1 << 30)
    orr     x0, x0, (3 << 28)
    orr     x0, x0, (1 << 26)
    orr     x0, x0, (1 << 24)
    mov     x1, (16 << 16)
    orr     x0, x0, x1
    //orr     x0, x0, (1 << 14)
    orr     x0, x0, (3 << 12)
    orr     x0, x0, (1 << 10)
    orr     x0, x0, (1 << 8)
    msr     tcr_el1, x0
    isb

    mrs     x0, sctlr_el1
    orr     x0, x0, 1
    msr     sctlr_el1, x0
    isb

    ldr     x30, =__sstack
    mov     sp, x30

    ldr     x0, =kernel_main
    blr     x0

in_el0:
    b .

.align 12
page_table_l0:
    .skip 4096, 0

.align 12
page_table_l1:
    .skip 4096, 0

.align 12
page_table_l2:
    .skip 4096, 0

.align 12
page_table_l3:
    .skip 4096, 0

exception_entry:
    stp     x20, x21, [sp, -16]!

    mov     x21, sp
    sub     x20, sp, 192
    and     sp, x20, ~0b1111

    stp     x0, x1, [sp, 0]

    add     x1, x2, 16
    ldp     x20, x21, [x21]

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
    bl      exception

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
    stp     x20, x21, [sp, -16]!

    mov     x21, sp
    sub     x20, sp, 192
    and     sp, x20, ~0b1111

    stp     x0, x1, [sp, 0]

    add     x1, x2, 16
    ldp     x20, x21, [x21]

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
    bl      interrupt

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
