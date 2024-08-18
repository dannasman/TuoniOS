use core::arch::asm;

pub struct Mmio {
    pub base: *mut u32,
}

impl Mmio {
    pub unsafe fn new(base: *mut u32) -> Self {
        Mmio { base }
    }

    #[inline(always)]
    pub unsafe fn write(&self, offset: usize, data: u32) {
        self.base.byte_add(offset).write_volatile(data)
    }

    #[inline(always)]
    pub unsafe fn read(&self, offset: usize) -> u32 {
        self.base.byte_add(offset).read_volatile()
    }

    #[inline(always)]
    pub fn delay(ticks: u32) {
        for _ in 0..ticks {
            unsafe {
                asm!("nop");
            }
        }
    }
}

pub enum Offset {
    UART0_BASE = 0x0100_0000,
    UART0_RSRECR = 0x0100_0000+ 0x04,
    UART0_FR = 0x0100_0000 + 0x18,
    UART0_ILPR = 0x0100_0000 + 0x20,
    UART0_IBRD = 0x0100_0000 + 0x24,
    UART0_FBRD = 0x0100_0000 + 0x28,
    UART0_LCRH = 0x0100_0000 + 0x2c,
    UART0_CR = 0x0100_0000 + 0x30,
    UART0_IFLS = 0x0100_0000 + 0x34,
    UART0_IMSC = 0x0100_0000 + 0x38,
    UART0_RIS = 0x0100_0000 + 0x3c,
    UART0_MIS = 0x0100_0000 + 0x40,
    UART0_ICR = 0x0100_0000 + 0x44,
    UART0_DMACR = 0x0100_0000 + 0x48,
    UART0_ITCR = 0x0100_0000 + 0x80,
    UART0_ITIP = 0x0100_0000 + 0x84,
    UART0_ITOP = 0x0100_0000 + 0x88,
    UART0_TDR = 0x0100_0000 + 0x8C,
}

impl Into<usize> for Offset {
    fn into(self) -> usize {
        self as usize
    }
}

impl Into<isize> for Offset {
    fn into(self) -> isize {
        self as isize
    }
}
