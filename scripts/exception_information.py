import argparse

exception_class = [
    "0x00: Unknown reason",
    "0x01: WFI or WFE instruction execution",
    "0x02: NONE",
    "0x03: MCR or MRC access to CP15",
    "0x04: MCRR or MRRC access to CP15",
    "0x05: MCR or MRC access to CP14",
    "0x06: LDC or STC access to CP14",
    "0x07: Access to SIMD or floating-point registers",
    "0x08: MCR or MRC access to CP10 (not reported via 0x07)",
    "0x09: NONE",
    "0x0A: NONE",
    "0x0B: NONE",
    "0x0C: MRRC access to CP14",
    "0x0D: NONE",
    "0x0E: Illegal Execution State",
    "0x0F: NONE",
    "0x10: NONE",
    "0x11: SVC instruction execution (AArch32)",
    "0x12: HVC instruction execution (AArch32)",
    "0x13: SMC instruction execution (AArch32)",
    "0x14: NONE",
    "0x15: SVC instruction execution (AArch64)",
    "0x16: HVC instruction execution (AArch64)",
    "0x17: SMC instruction execution (AArch64)",
    "0x18: MSR, MRS, or System instruction execution (not 0x00/01/07)",
    "0x19: NONE",
    "0x1A: NONE",
    "0x1B: NONE",
    "0x1C: NONE",
    "0x1D: NONE",
    "0x1E: NONE",
    "0x1F: NONE",
    "0x20: Instruction Abort from a lower Exception level",
    "0x21: Instruction Abort without change in Exception level",
    "0x22: Misaligned PC exception",
    "0x23: NONE",
    "0x24: Data Abort from a lower Exception level",
    "0x25: Data Abort without change in Exception level",
    "0x26: Stack Pointer Alignment exception",
    "0x27: NONE",
    "0x28: Floating-point exception (AArch32)",
    "0x29: NONE",
    "0x2A: NONE",
    "0x2B: NONE",
    "0x2C: Floating-point exception (AArch64)",
    "0x2D: NONE",
    "0x2E: NONE",
    "0x2F: SError interrupt",
    "0x30: Breakpoint exception from a lower Exception level",
    "0x31: Breakpoint exception without change in Exception level",
    "0x32: Software Step exception from a lower Exception level",
    "0x33: Software Step exception without change in Exception level",
    "0x34: Watchpoint exception from a lower Exception level",
    "0x35: Watchpoint exception without change in Exception level",
    "0x36: NONE",
    "0x37: NONE",
    "0x38: BKPT instruction execution",
    "0x39: NONE",
    "0x3A: Vector catch exception from AArch32 state",
    "0x3B: NONE",
    "0x3C: BRK instruction execution",
]

instruction_length = [
    "0: 16-bit instruction trapped",
    "1: 32-bit instruction trapped. Applies to:\n"
    "   - SError interrupt\n"
    "   - Instruction Abort exception\n"
    "   - Misaligned PC exception\n"
    "   - Misaligned Stack Pointer exception\n"
    "   - Data Abort (when ISV == 0)\n"
    "   - Illegal Execution State exception\n"
    "   - All debug exceptions except Software Breakpoint Instruction exceptions\n"
    "   - A32 BKPT or A64 BRK instruction (for Software Breakpoints)"
]

def parse_arguments():
    parser = argparse.ArgumentParser(prog='exception_information',
            description="extract information read from esr_el1 and far_el1")
    parser.add_argument("--esr_el1", type=int, help="value in esr_el1")
    parser.add_argument("--far_el1", type=int, help="value in far_el1")

    args = parser.parse_args()
    return args

def extract_esr_el1(x):
    ec = (x >> 26) & 0x3f
    il = (x >> 25) & 0x1
    iss = x & 0x1ffffff

    print("*** ESR_EL1 INFORMATION ***")
    print(f"Exception class {exception_class[ec]}")
    print(f"Instruction length bit {instruction_length[il]}")
    print(f"ISS: {iss}")

def extract_far_el1(x):

    print("*** FAR_EL1 INFORMATION ***")
    print(f"Faulting virtual address: {hex(x)}")

if __name__ == "__main__":
    args = parse_arguments()
    esr_el1 = args.esr_el1
    far_el1 = args.far_el1
    extract_esr_el1(esr_el1)
    extract_far_el1(far_el1)

