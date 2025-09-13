#![no_std]
#![feature(const_mut_refs)]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(libtuoni::test_runner)]
#![reexport_test_harness_main = "test_main"]

use libtuoni::{allocator, bsp, log_write};

#[cfg(feature = "raspi4b")]
use libtuoni::log;

const HEAP_SIZE: usize = 0x100000;

#[cfg(feature = "raspi4b")]
const MMIO_BASE: usize = 0xFE00_0000;

#[cfg(not(feature = "raspi4b"))]
const MMIO_BASE: usize = 0x0800_0000;

#[no_mangle]
pub extern "C" fn kernel_main(x0: u64, x1: u64, x2: u64, x3: u64, x4: u64) -> ! {
    assert!(x0 > 0xffff000000000000);
    assert!(x1 > 0xffff000000000000);
    assert!(x2 > 0xffff000000000000);
    assert!(x3 > 0xffff000000000000);
    assert!(x4 > 0xffff000000000000);

    bsp::drivers::mmio::init(MMIO_BASE);

    #[cfg(feature = "raspi4b")]
    log::init();

    let heap_base = x1 as usize;
    allocator::init(heap_base, HEAP_SIZE);

    log_write!("[ok]\r\n");

    loop {}
}


