use core::fmt::{self, Write};

const FLAG_REGISTER_OFFSET: usize = 0x18;
const FR_BUSY: u8 = 1 << 3;
const FR_TXFF: u8 = 1 << 5;

#[derive(Debug)]
pub struct Uart {
    base_address: *mut u8,
}

impl Uart {
    pub unsafe fn new(base_address: *mut u8) -> Self {
        Self { base_address }
    }

    pub fn write_byte(&self, byte: u8) {
        while self.read_flag_register() & FR_TXFF != 0 {}

        unsafe {
            self.base_address.write_volatile(byte);
        }

        while self.read_flag_register() & FR_BUSY != 0 {}
    }

    fn read_flag_register(&self) -> u8 {
        unsafe {
            self
                .base_address
                .add(FLAG_REGISTER_OFFSET)
                .read_volatile()
        }
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
