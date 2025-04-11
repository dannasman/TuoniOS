.section ".init"
.globl _start
.extern _kernel_begin
.extern _early_heap_begin
.extern _stack_end
.extern _peripherals_begin

_start:
    mrs     x0, mpidr_el1
    and     x0, x0, 0xff;
    cbz     x0, 1f
0:
    wfe
    b       0b
1:
    msr     spsel, 1
    mrs     x0, CurrentEL
    and     x0, x0, 0b1100
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
    msr     elr_el3, x0
    eret
in_el2:
    msr     sctlr_el1, xzr
    mrs     x0, hcr_el2
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

    mrs     x0, cpacr_el1
    orr     x0, x0, (0b11 << 20)
    msr     cpacr_el1, x0

    ldr     x0, =MAIR_EL1_VALUE
    msr     mair_el1, x0

    ldr     x0, =TCR_EL1_VALUE
    msr     tcr_el1, x0
    isb

    ldr     x0, =pagetable_level0
    ldr     x1, =pagetable_level1
    orr     x2, x1, 3
    str     x2, [x0]
    str     x2, [x0, 8]

    ldr     x4, =PERIPHERALS_BASE
    lsr     x5, x4, 30
    and     x5, x5, 0x1ff
    lsl     x4, x5, 30
    ldr     x6, =PERIPHERALS_ATTR
    orr     x4, x4, x6
    str     x4, [x1, x5, lsl 3]
    
    ldr     x4, =_kernel_begin
    lsr     x5, x4, 30
    and     x5, x5, 0x1ff
    lsl     x4, x5, 30
    ldr     x6, =CODE_ATTR
    orr     x4, x4, x6
    str     x4, [x1, x5, lsl 3]

    msr     ttbr0_el1, x0
    msr     ttbr1_el1, x0
    isb

    ldr     x0, =_stack_end
    mov     sp, x0

    mrs     x0, sctlr_el1
    orr     x0, x0, (1 << 0)
    orr     x0, x0, (1 << 2)
    orr     x0, x0, (1 << 12)
    msr     sctlr_el1, x0
    isb

    ldr     x0, =_kernel_begin
    ldr     x1, =_early_heap_begin
    ldr     x2, =_early_heap_end
    ldr     x3, =_stack_begin
    ldr     x4, =_stack_end
    mov     sp, x4
    ldr     x30, =kernel_main
    blr     x30
in_el0:
    b       .

.balign 0x1000
pagetable_level0:
    .space 0x1000
.balign 0x1000
pagetable_level1:
    .space 0x1000

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

.equ        PERIPHERALS_BASE, _peripherals_begin

//          UXN:    0b1     << 54
//          PXN:    0b1     << 53
//          AF:     0b1     << 10
//          SH:     0b10    << 8
//          AP:     0b00    << 6
//          NS:     0b0     << 5
//          INDX:   0b000   << 2
//          ENTRY:  0b01    << 0
.equ        PERIPHERALS_ATTR, 0x60000000000601

//          UXN:    0b0     << 54
//          PXN:    0b0     << 53
//          AF:     0b1     << 10
//          SH:     0b11    << 8
//          AP:     0b00    << 6
//          NS:     0b0     << 5
//          INDX:   0b001   << 2
//          ENTRY:  0b01    << 0
.equ        CODE_ATTR, 0x00000000000705
