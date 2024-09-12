#![no_std]
#![no_main]
use core::arch::global_asm;
use core::fmt::Write;
use core::mem::MaybeUninit;
use core::ptr::addr_of_mut;

pub mod arch;
pub mod drivers;
pub mod mem;
pub mod panic;

const EARLY_HEAP_LEN: usize = 0x100000;
const MMIO_BASE: *mut u32 = 0xffff_0000_0800_0000 as *mut u32;

global_asm!(include_str!("boot.s"));

extern "C" {
    #[link_name = "_early_heap_begin"]
    static mut EARLY_HEAP: u8;
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let early_heap = unsafe {
        core::slice::from_raw_parts_mut(
            addr_of_mut!(EARLY_HEAP) as *mut MaybeUninit<u8>,
            EARLY_HEAP_LEN,
        )
    };

    let mmio = unsafe { arch::mmio::Mmio::new(MMIO_BASE) };
    let mut uart = unsafe { drivers::serial::Uart::new(mmio) };

    writeln!(uart, "Hello World!").unwrap();

    loop {
        uart.write_byte(uart.read_byte() as u8)
    }
}
