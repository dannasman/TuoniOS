#![no_std]
use core::fmt::{self, Write};

use crate::arch;

const FLAG_REGISTER_OFFSET: usize = 0x18;
const FR_BUSY: u8 = 1 << 3;
const FR_RXFE: u8 = 1 << 4;
const FR_TXFF: u8 = 1 << 5;

#[derive(Debug)]
pub struct Uart {}

impl Uart {
    pub const fn new() -> Self {
        Self {}
    }

    #[inline(always)]
    pub fn write_byte(&self, byte: u8) {
        while self.read_flag_register() & FR_TXFF != 0 {}

        unsafe {
            arch::mmio::write(arch::mmio::Offset::UART0_BASE as usize, byte);
        }

        while self.read_flag_register() & FR_BUSY != 0 {}
    }

    #[inline(always)]
    pub fn read_byte(&self) -> u8 {
        while self.read_flag_register() & FR_RXFE != 0 {}
        unsafe { arch::mmio::read(arch::mmio::Offset::UART0_BASE as usize) }
    }

    #[inline(always)]
    fn read_flag_register(&self) -> u8 {
        unsafe { arch::mmio::read(arch::mmio::Offset::UART0_FR as usize) }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.as_bytes() {
            self.write_byte(*c);
        }
        Ok(())
    }
}

unsafe impl Send for Uart {}
