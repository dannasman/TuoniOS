use core::fmt::{self, Write};

use crate::mmio::*;

const FLAG_REGISTER_OFFSET: usize = 0x18;
const FR_BUSY: u32 = 1 << 3;
const FR_RXFE: u32 = 1 << 4;
const FR_TXFF: u32 = 1 << 5;

#[repr(C)]
pub struct Uart {
    mmio: MMIO,
}

impl Uart {
    pub unsafe fn new(mmio: MMIO) -> Self {
        /*mmio.write(Offset::UART0_CR as isize, 0x0000_0000);

        mmio.write(Offset::GPPUD as isize, 0x0000_0000);
        MMIO::delay(150);

        mmio.write(Offset::GPPUDCLK0 as isize, (1 << 14) | (1 << 15));
        MMIO::delay(150);

        mmio.write(Offset::GPPUDCLK0 as isize, 0x0000_0000);

        mmio.write(Offset::UART0_ICR as isize, 0x7ff);

        // TODO: Set UART_CLOCK to 3MHz

        mmio.write(Offset::UART0_IBRD as isize, 1);
        mmio.write(Offset::UART0_FBRD as isize, 40);

        mmio.write(Offset::UART0_LCRH as isize, (1 << 4) | (1 << 5) | (1 << 6));

        mmio.write(
            Offset::UART0_IMSC as isize,
            (1 << 1) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10)
        );

        mmio.write(Offset::UART0_CR as isize, (1 << 0) | (1 << 8) | (1 << 9)); 
        */
        Self { mmio }
    }

    pub fn write_byte(&self, byte: u8) {
        while self.read_flag_register() & FR_TXFF != 0 {}

        unsafe {
            self.mmio.write(Offset::UART0_BASE as usize, byte as u32);
        }

        while self.read_flag_register() & FR_BUSY != 0 {}
    }

    pub fn read_byte(&self) -> u32 {
        while self.read_flag_register() & FR_RXFE != 0 {}
        self.mmio.read(Offset::UART0_BASE as usize)
    }

    fn read_flag_register(&self) -> u32 {
        self.mmio.read(Offset::UART0_FR as usize)
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
