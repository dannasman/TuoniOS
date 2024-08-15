#![no_std]
#![no_main]
use core::arch::global_asm;
use core::fmt::Write;

mod mmio;
mod panic;
mod serial;

use crate::mmio::*;
use crate::serial::Uart;

const MMIO_BASE: *mut u32 = 0x3f00_0000 as *mut u32;

const TUONI_MMIO: MMIO = MMIO {
    base: MMIO_BASE
};

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut uart = unsafe { Uart::new(TUONI_MMIO) };

    writeln!(uart, "Hello World!\r\n").unwrap();

    loop {}
}
