#![no_std]
#![no_main]
use core::arch::global_asm;
use core::fmt::Write;

pub mod arch;
pub mod drivers;
pub mod panic;

const MMIO_BASE: *mut u32 = 0xffff_0000_0800_0000 as *mut u32;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let mmio = unsafe { arch::mmio::Mmio::new(MMIO_BASE) };
    let mut uart = unsafe { drivers::serial::Uart::new(mmio) };

    writeln!(uart, "Hello World!").unwrap();

    loop {
        uart.write_byte(uart.read_byte() as u8)
    }
}
