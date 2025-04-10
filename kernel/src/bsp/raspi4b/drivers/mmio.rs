use core::arch::asm;
use core::ptr::null_mut;

use crate::sync;

static mut MMIO: sync::mutex::Mutex<Mmio> = sync::mutex::Mutex::new(Mmio::new());

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum Offset {
    GPIO_BASE = 0x200000,
    GPFSEL1 = 0x200000 + 0x04,
    GPIO_PUP_PDN_CNTRL_REG0 = 0x200000 + 0xE4,
    GPPUD = 0x200000 + 0x94,
    GPPUDCLK0 = 0x200000 + 0x98,

    UART0_BASE = 0x201000,
    UART0_RSRECR = 0x201000 + 0x04,
    UART0_FR = 0x201000 + 0x18,
    UART0_ILPR = 0x201000 + 0x20,
    UART0_IBRD = 0x201000 + 0x24,
    UART0_FBRD = 0x201000 + 0x28,
    UART0_LCRH = 0x201000 + 0x2c,
    UART0_CR = 0x201000 + 0x30,
    UART0_IFLS = 0x201000 + 0x34,
    UART0_IMSC = 0x201000 + 0x38,
    UART0_RIS = 0x201000 + 0x3c,
    UART0_MIS = 0x201000 + 0x40,
    UART0_ICR = 0x201000 + 0x44,
    UART0_DMACR = 0x201000 + 0x48,
    UART0_ITCR = 0x201000 + 0x80,
    UART0_ITIP = 0x201000 + 0x84,
    UART0_ITOP = 0x201000 + 0x88,
    UART0_TDR = 0x201000 + 0x8C,

    MBOX_BASE = 0xB880, // MBOX_READ
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

#[derive(Debug)]
pub struct Mmio {
    pub base: *mut u8,
}

impl Mmio {
    pub const fn new() -> Self {
        Mmio { base: null_mut() }
    }

    pub fn init(&mut self, base: usize) {
        self.base = base as *mut u8;

        self.map_pl011_uart();
    }

    #[inline(always)]
    pub unsafe fn write<T>(&self, offset: usize, data: T) {
        let b = self.base.add(offset) as *mut T;
        b.write_volatile(data)
    }

    #[inline(always)]
    pub unsafe fn read<T>(&self, offset: usize) -> T {
        let b = self.base.add(offset) as *mut T;
        b.read_volatile()
    }

    pub fn map_pl011_uart(&mut self) {
        let mut r: u32 = unsafe { self.read::<u32>(Offset::GPFSEL1 as usize) };
        r = (r | (1 << 17) | (1 << 14)) & !(0b11 << 15) & !(0b11 << 12);

        unsafe { self.write(Offset::GPFSEL1 as usize, r) };

        unsafe {
            self.write(
                Offset::GPIO_PUP_PDN_CNTRL_REG0 as usize,
                ((0b01 << 30) | (0b01 << 28)) as u32,
            )
        };
    }
}

pub fn init(base: usize) {
    unsafe { MMIO.lock().init(base) }
    unsafe { MMIO.lock().map_pl011_uart() }
}

#[inline(always)]
pub fn write<T>(offset: usize, data: T) {
    unsafe { MMIO.lock().write(offset, data) }
}

#[inline(always)]
pub fn read<T>(offset: usize) -> T {
    unsafe { MMIO.lock().read(offset) }
}

#[allow(dead_code)]
#[inline(always)]
pub fn delay(ticks: usize) {
    for _ in 0..ticks {
        unsafe {
            asm!("nop");
        }
    }
}
