use core::fmt::{self, Write};

use crate::mmio::*;

const FLAG_REGISTER_OFFSET: usize = 0x18;
const FR_BUSY: u32 = 1 << 3;
const FR_RXFE: u32 = 1 << 4;
const FR_TXFF: u32 = 1 << 5;

#[repr(C, align(16))]
pub struct MBox([u32; 9]);

static mut MBOX: MBox = MBox([
    9 * 4,
    0,
    0x38002,
    12,
    8,
    2,
    3000000,
    0,
    0,
]);

pub struct Uart {
    mmio: MMIO,
}

impl Uart {
    pub unsafe fn new(mmio: MMIO) -> Self {
        mmio.write(Offset::UART0_CR as usize, 0x0000_0000);

        mmio.write(Offset::GPPUD as usize, 0x0000_0000);
        MMIO::delay(150);

        mmio.write(Offset::GPPUDCLK0 as usize, (1 << 14) | (1 << 15));
        MMIO::delay(150);

        mmio.write(Offset::GPPUDCLK0 as usize, 0x0000_0000);

        mmio.write(Offset::UART0_ICR as usize, 0x7ff);

        // TODO: Set UART_CLOCK to 3MHz
        let mbox_ptr = &MBOX as *const MBox as usize;
        let r = (mbox_ptr & !0xf) | 8;
        while mmio.read(Offset::MBOX_STATUS as usize) & 0x8000_0000 != 0 {}
        mmio.write(Offset::MBOX_WRITE as usize, r as u32);
        while mmio.read(Offset::MBOX_STATUS as usize) & 0x4000_0000 != 0
            || mmio.read(Offset::MBOX_READ as usize) != r as u32 {}

        mmio.write(Offset::UART0_IBRD as usize, 1);
        mmio.write(Offset::UART0_FBRD as usize, 40);

        mmio.write(Offset::UART0_LCRH as usize, (1 << 4) | (1 << 5) | (1 << 6));

        mmio.write(
            Offset::UART0_IMSC as usize,
            (1 << 1) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10)
        );

        mmio.write(Offset::UART0_CR as usize, (1 << 0) | (1 << 8) | (1 << 9));

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
        unsafe { self.mmio.read(Offset::UART0_BASE as usize) }
    }

    fn read_flag_register(&self) -> u32 {
        unsafe { self.mmio.read(Offset::UART0_FR as usize) }
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
