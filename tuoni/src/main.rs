#![no_std]
#![no_main]
#![feature(const_mut_refs)]

mod allocator;
mod bsp;
mod cpu;
mod drivers;
mod exceptions;
mod log;
//pub mod mem;
mod panic;
mod sync;

const HEAP_SIZE: usize = 0x100000;

#[cfg(feature = "raspi4b")]
const MMIO_BASE: usize = 0xFE00_0000;

#[cfg(not(feature = "raspi4b"))]
const MMIO_BASE: usize = 0x0800_0000;

#[no_mangle]
pub extern "C" fn kernel_main(x0: u64, x1: u64, x2: u64, x3: u64, x4: u64) -> ! {
    bsp::mmio::init(MMIO_BASE);
    log_write!("mmio initialized\n");

    log_write!("Welcome to TuoniOS!\n");
    log_write!("Kernel begin at: {:#16x}\n", x0);
    log_write!("Heap begin at: {:#16x}\n", x1);
    log_write!("Heap end at: {:#16x}\n", x2);
    log_write!("Stack begin at: {:#16x}\n", x3);
    log_write!("Stack end at: {:#16x}\n", x4);

    let heap_base = x1 as usize;
    allocator::init(heap_base, HEAP_SIZE);
    log_write!("allocator initialized\n");

    loop {
        log_write!("{}", log::read_byte() as char);
    }
}
