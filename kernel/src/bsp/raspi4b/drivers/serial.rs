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

    // We require 115200 baud rate and UARTCLK is set to 3 MHz in config.txt
    // Baud Rate divisor: 3000000/(16*115200)=1.6276041666666667.
    // Integer part: 1
    // Float part: 0.6276041666666667
    // Fractional part m: int((0.6276041666666667*64)+0.5)=40
    // Generated baud rate divider: 1+40/64=1.625
    // Generated baud rate: 3000000/(16*1.625)=115385
    // Error: (115385-115200)/115200=0.00161=0.161%
    pub fn init(&mut self) {
        mmio::write(mmio::Offset::UART0_CR as usize, 0);

        let icr_val: u16 = mmio::read(mmio::Offset::UART0_ICR as usize);
        mmio::write(mmio::Offset::UART0_ICR as usize, icr_val & 0xf800);

        mmio::write(mmio::Offset::UART0_IBRD as usize, 1u8);

        mmio::write(mmio::Offset::UART0_FBRD as usize, 40u8);

        mmio::write(mmio::Offset::UART0_LCRH as usize, ((1 << 4) | (1 << 5) | (1 << 6)) as u8);

        mmio::write(mmio::Offset::UART0_IMSC as usize, ((1 << 1) | (1 << 4) | (1 << 5) | (1 << 5) |
                                               (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10)) as u16);

        mmio::write(mmio::Offset::UART0_CR as usize, 1 | (1 << 8) | (9 << 1));

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
