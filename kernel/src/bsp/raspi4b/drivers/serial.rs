use crate::bsp::drivers::mmio;

// const FLAG_REGISTER_OFFSET: usize = 0x18;
const FR_BUSY: u8 = 1 << 3;
const FR_RXFE: u8 = 1 << 4;
const FR_TXFF: u8 = 1 << 5;

#[derive(Debug)]
pub struct Uart {}

impl Uart {
    pub const fn new() -> Self {
        Self {}
    }

    // We require 115_200 baud rate and UARTCLK is set to 48 MHz in config.txt
    // Baud Rate divisor: 48_000_000/(16*115_200)=26.041667.
    // Integer part: 26
    // Fractional part: 0.041667
    // Fractional part m: int((0.041667*64)+0.5)=3
    // Generated baud rate divider: 3+16/64=26.046875
    // Generated baud rate: 48_000_000/(16*26.046875)=115_176
    // Error: (115_176-115_200)/115_200=0.021%
    pub fn init(&mut self) {
        self.flush();

        mmio::write(mmio::Offset::UART0_CR as usize, 0 as u16);

        let icr_val: u16 = mmio::read::<u16>(mmio::Offset::UART0_ICR as usize);
        mmio::write(mmio::Offset::UART0_ICR as usize, icr_val & 0xf800u16);

        mmio::write(mmio::Offset::UART0_IBRD as usize, 26u16);

        mmio::write(mmio::Offset::UART0_FBRD as usize, 3u8);

        mmio::write(
            mmio::Offset::UART0_LCRH as usize,
            ((1 << 4) | (1 << 5) | (1 << 6)) as u8,
        );

        mmio::write(
            mmio::Offset::UART0_CR as usize,
            ((1 << 0) | (1 << 8) | (1 << 9)) as u32,
        );
    }

    #[inline(always)]
    pub fn flush(&self) {
        while mmio::read::<u8>(mmio::Offset::UART0_FR as usize) & FR_BUSY != 0 {}
    }

    #[inline(always)]
    pub fn write_byte(&self, byte: u8) {
        while self.read_flag_register() & FR_TXFF != 0 {}

        mmio::write(mmio::Offset::UART0_BASE as usize, byte);

        while self.read_flag_register() & FR_BUSY != 0 {}
    }

    #[inline(always)]
    pub fn read_byte(&self) -> u8 {
        while self.read_flag_register() & FR_RXFE != 0 {}
        mmio::read(mmio::Offset::UART0_BASE as usize)
    }

    #[inline(always)]
    fn read_flag_register(&self) -> u8 {
        mmio::read(mmio::Offset::UART0_FR as usize)
    }
}

// FIXME: look into necessity of this
unsafe impl Send for Uart {}
