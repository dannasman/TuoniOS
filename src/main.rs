#![no_std]
#![no_main]
use core::arch::global_asm;
use core::fmt::Write;

mod panic;
mod serial;

use crate::serial::Uart;

const PL011_BASE_ADDRESS: *mut u8 = 0x900_0000 as *mut u8;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut uart = unsafe { Uart::new(PL011_BASE_ADDRESS) };

    uart.write_byte(0x21);

    writeln!(uart, "Hello World!").unwrap();

    loop {}
}

