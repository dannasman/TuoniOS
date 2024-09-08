.section ".init"
.globl _start
.extern __stack_start

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

    ldr     x0, =ttb1_base
    msr     ttbr0_el1, x0
    msr     ttbr1_el1, x0
    isb

    ldr     x0, =MAIR_EL1_VALUE
    msr     mair_el1, x0

    ldr     x0, =TCR_EL1_VALUE
    msr     tcr_el1, x0
    isb

    dsb     ish
    isb

    ldr     x30, =__stack_start
    mov     sp, x30

    mrs     x0, sctlr_el1
    orr     x0, x0, 0x1
    msr     sctlr_el1, x0
    isb

    ldr     x0, =kernel_main
    blr     x0
in_el0:
    b       .

.macro      PUT_64B high, low
.word       \low
.word       \high
.endm

.macro      BLOCK_1GB PA, ATTR_HI, ATTR_LO
PUT_64B     \ATTR_HI, ((\PA) & 0xC0000000) | \ATTR_LO | 0x1
.endm

.align 12
ttb1_base:
    BLOCK_1GB   0x00000000, 0x600000, 0x740
    BLOCK_1GB   0x40000000, 0x600000, 0x744
//    BLOCK_1GB   0x80000000, 0, 0x744

//          ATTR0:  0b00000000 << 0 | Device memory 
//          ATTR1:  0b11111111 << 8 | Normal memory
.equ        MAIR_EL1_VALUE, 0xff00

//          IPS:    0b101   << 32
//          TG1:    0b10    << 30
//          SH1:    0b11    << 28
//          ORGN1:  0b01    << 26
//          IRGN1:  0b01    << 24
//          EPD1:   0b0     << 23
//          A1:     0b0     << 22
//          T1SZ:   16      << 16
//          TG0:    0b00    << 14
//          SH0:    0b11    << 12
//          ORGN0:  0b01    << 10
//          IRGN0:  0b01    << 8
//          EPD0:   0b0     << 7
//          RES0:   0b0     << 6
//          T0SZ:   16      << 0
.equ        TCR_EL1_VALUE, 0x5b5103510

.section    ".text.exception"

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

.equ        PSCI_SYSTEM_OFF, 0x84000008
.globl      system_off
system_off:
    ldr     x0, =PSCI_SYSTEM_OFF
    hvc     0
