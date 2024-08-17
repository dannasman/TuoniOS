#![no_std]
#![no_main]
use core::arch::global_asm;
use core::fmt::Write;

mod mmio;
mod panic;
mod serial;

use crate::mmio::*;
use crate::serial::Uart;

const MMIO_BASE: *mut u32 = 0x0800_0000 as *mut u32;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut mmio = unsafe { MMIO::new(MMIO_BASE) };
    let mut uart = unsafe { Uart::new(mmio) };

    writeln!(uart, "Hello World!").unwrap();

    loop { uart.write_byte(uart.read_byte() as u8) }
}
