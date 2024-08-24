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
    msr     hcr_el2, x0
    mov     x0, 0b00101
    msr     spsr_el2, x0

    adr     x0, in_el1
    msr     elr_el2, x0
    eret
in_el1:
    ldr     x30, =__stack_top
    mov     sp, x30

    bl      kernel_main
in_el0:
    b .

.equ PSCI_SYSTEM_OFF, 0x84000008
.globl system_off
system_off:
    ldr     x0, =PSCI_SYSTEM_OFF
    hvc     #0
