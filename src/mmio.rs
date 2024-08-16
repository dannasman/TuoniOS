use core::arch::asm;

pub struct MMIO {
    pub base: *mut u32,
}

impl MMIO {
    pub unsafe fn new(base: *mut u32) -> Self {
        MMIO { base }
    }

    #[inline(always)]
    pub unsafe fn write(&self, reg: usize, data: u32) {
        self.base.byte_add(reg).write_volatile(data)
    }

    #[inline(always)]
    pub unsafe fn read(&self, reg: usize) -> u32 {
        self.base.byte_add(reg).read_volatile()
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
    GPIO_BASE = 0x200000,
    GPPUD = 0x200000 + 0x94,
    GPPUDCLK0 = 0x200000 + 0x98,
    UART0_BASE = 0x200000 + 0x1000,
    UART0_RSRECR = 0x200000 + 0x1000 + 0x04,
    UART0_FR = 0x200000 + 0x1000 + 0x18,
    UART0_ILPR = 0x200000 + 0x1000 + 0x20,
    UART0_IBRD = 0x200000 + 0x1000 + 0x24,
    UART0_FBRD = 0x200000 + 0x1000 + 0x28,
    UART0_LCRH = 0x200000 + 0x1000 + 0x2c,
    UART0_CR = 0x200000 + 0x1000 + 0x30,
    UART0_IFLS = 0x200000 + 0x1000 + 0x34,
    UART0_IMSC = 0x200000 + 0x1000 + 0x38,
    UART0_RIS = 0x200000 + 0x1000 + 0x3c,
    UART0_MIS = 0x200000 + 0x1000 + 0x40,
    UART0_ICR = 0x200000 + 0x1000 + 0x44,
    UART0_DMACR = 0x200000 + 0x1000 + 0x48,
    UART0_ITCR = 0x200000 + 0x1000 + 0x80,
    UART0_ITIP = 0x200000 + 0x1000 + 0x84,
    UART0_ITOP = 0x200000 + 0x1000 + 0x88,
    UART0_TDR = 0x200000 + 0x1000 + 0x8C,
    MBOX_READ = 0xB880,
    MBOX_STATUS = 0xB880 + 0x18,
    MBOX_WRITE = 0xB880 + 0x20,
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
