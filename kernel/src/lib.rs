#![no_std]
#![feature(const_mut_refs)]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod allocator;
pub mod bsp;
pub mod cpu;
pub mod exceptions;
pub mod log;
//pub mod mem;
pub mod panic;
pub mod sync;

#[cfg(test)]
const HEAP_SIZE: usize = 0x100000;

#[cfg(test)]
#[cfg(feature = "raspi4b")]
const MMIO_BASE: usize = 0xFE00_0000;

#[cfg(test)]
#[cfg(not(feature = "raspi4b"))]
const MMIO_BASE: usize = 0x0800_0000;

#[cfg(test)]
#[no_mangle]
pub extern "C" fn kernel_main(x0: u64, x1: u64, x2: u64, x3: u64, x4: u64) -> ! {
    bsp::drivers::mmio::init(MMIO_BASE);

    #[cfg(feature = "raspi4b")]
    log::init();

    log_write!("mmio initialized\r\n");

    log_write!("Welcome to chainloaded TuoniOS!\r\n");
    log_write!("Kernel begin at: {:#16x}\r\n", x0);
    log_write!("Heap begin at: {:#16x}\r\n", x1);
    log_write!("Heap end at: {:#16x}\r\n", x2);
    log_write!("Stack begin at: {:#16x}\r\n", x3);
    log_write!("Stack end at: {:#16x}\r\n", x4);

    let heap_base = x1 as usize;
    allocator::init(heap_base, HEAP_SIZE);
    log_write!("allocator initialized\r\n");

    test_main();

    loop {
        log_write!("{}", log::read_byte() as char);
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    log_write!("Running {} tests\r\n", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    log_write!("trivial assertion... \r\n");
    assert_eq!(1, 1);
    log_write!("[ok]\r\n");
}
