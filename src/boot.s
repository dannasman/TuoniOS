.section ".text.boot"

.globl _start

_start:
    ldr     x30, =_start
    mov     sp, x30
    bl      kernel_main

.equ PSCI_SYSTEM_OFF, 0x84000008
.globl system_off
system_off:
    ldr     x0, =PSCI_SYSTEM_OFF
    hvc     #0
