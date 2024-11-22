#![no_std]
#![no_main]
#![feature(const_mut_refs)]
#![feature(alloc)]
extern crate alloc;
use alloc::{vec, vec::Vec};

use core::alloc::Layout;
use core::arch::global_asm;
use core::mem::{align_of, size_of};
use core::ptr::addr_of;

pub mod allocator;
pub mod arch;
pub mod drivers;
pub mod log;
//pub mod mem;
pub mod panic;
pub mod sync;

global_asm!(include_str!("boot.s"));

const HEAP_SIZE: usize = 0x100000;
const MMIO_BASE: usize = 0xffff_0000_0800_0000;

#[no_mangle]
pub extern "C" fn kernel_main(x0: u64, x1: u64, x2: u64, x3: u64, x4: u64) -> ! {
    arch::mmio::init(MMIO_BASE);
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
