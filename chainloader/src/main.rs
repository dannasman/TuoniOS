#![no_std]
#![no_main]
#![feature(const_mut_refs)]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("stub.s"));

const MMIO_BASE: usize = 0xFE00_0000;
const LOAD_ADDR: usize = 0x0020_0000;

// const FLAG_REGISTER_OFFSET: usize = 0x18;
const FR_BUSY: u8 = 1 << 3;
const FR_RXFE: u8 = 1 << 4;
const FR_TXFF: u8 = 1 << 5;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
enum Offset {
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

#[inline(always)]
fn mmio_write<T>(offset: usize, data: T) {
    unsafe {
        let base = MMIO_BASE as *mut u8;
        let adr = base.add(offset) as *mut T;
        adr.write_volatile(data)
    }
}

#[inline(always)]
fn mmio_read<T>(offset: usize) -> T {
    unsafe {
        let base = MMIO_BASE as *mut u8;
        let adr = base.add(offset) as *mut T;
        adr.read_volatile()
    }
}

#[inline(always)]
fn flush() {
    while mmio_read::<u8>(Offset::UART0_FR as usize) & FR_BUSY != 0 {}
}

#[inline(always)]
fn write_byte(byte: u8) {
    while read_flag_register() & FR_TXFF != 0 {}

    mmio_write(Offset::UART0_BASE as usize, byte);

    while read_flag_register() & FR_BUSY != 0 {}
}

#[inline(always)]
fn read_byte() -> u8 {
    while read_flag_register() & FR_RXFE != 0 {}
    mmio_read(Offset::UART0_BASE as usize)
}

#[inline(always)]
fn read_flag_register() -> u8 {
    mmio_read(Offset::UART0_FR as usize)
}

fn uart_init() {
    let mut r: u32 = mmio_read::<u32>(Offset::GPFSEL1 as usize);
    r = (r | (1 << 17) | (1 << 14)) & !(0b11 << 15) & !(0b11 << 12);

    mmio_write(Offset::GPFSEL1 as usize, r);

    mmio_write(
        Offset::GPIO_PUP_PDN_CNTRL_REG0 as usize,
        ((0b01 << 30) | (0b01 << 28)) as u32,
    );

    flush();

    mmio_write(Offset::UART0_CR as usize, 0 as u16);

    let icr_val: u16 = mmio_read::<u16>(Offset::UART0_ICR as usize);
    mmio_write(Offset::UART0_ICR as usize, icr_val & 0xf800u16);

    mmio_write(Offset::UART0_IBRD as usize, 26u16);

    mmio_write(Offset::UART0_FBRD as usize, 3u8);

    mmio_write(
        Offset::UART0_LCRH as usize,
        ((1 << 4) | (1 << 5) | (1 << 6)) as u8,
    );

    mmio_write(
        Offset::UART0_CR as usize,
        ((1 << 0) | (1 << 8) | (1 << 9)) as u32,
    );
}

fn uart_reset() {
    mmio_write::<u32>(Offset::UART0_CR as usize, 0);

    mmio_write::<u32>(Offset::UART0_ICR as usize, 0x7FF);

    mmio_write::<u32>(Offset::UART0_IBRD as usize, 0);
    mmio_write::<u32>(Offset::UART0_FBRD as usize, 0);

    mmio_write::<u32>(Offset::UART0_LCRH as usize, 0);

    mmio_write::<u32>(Offset::UART0_IFLS as usize, 0);
    mmio_write::<u32>(Offset::UART0_DMACR as usize, 0);
    mmio_write::<u32>(Offset::UART0_CR as usize, 0);

    flush()
}

#[no_mangle]
pub extern "C" fn chainloader_main() -> ! {
    uart_init();

    for _ in 0..3 {
        write_byte(3);
    }

    let mut size: u32 = u32::from(read_byte());
    size |= u32::from(read_byte()) << 8;
    size |= u32::from(read_byte()) << 16;
    size |= u32::from(read_byte()) << 24;

    write_byte('O' as u8);
    write_byte('K' as u8);

    let kernel_addr: *mut u8 = LOAD_ADDR as *mut u8;
    unsafe {
        for i in 0..size {
            kernel_addr.offset(i as isize).write_volatile(read_byte());
        }
    }

    flush();

    uart_reset();

    let kernel: fn() -> ! = unsafe { core::mem::transmute(kernel_addr) };

    kernel();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
