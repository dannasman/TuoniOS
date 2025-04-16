#![no_std]
#![no_main]
#![feature(const_mut_refs)]

mod allocator;
mod bsp;
mod cpu;
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

#[cfg(feature = "raspi4b")]
const LOAD_ADDR: usize = 0x0400_0000;

// FIXME: test the address properly
#[cfg(not(feature = "raspi4b"))]
const LOAD_ADDR: usize = 0x4400_0000;

#[no_mangle]
pub extern "C" fn kernel_main(x0: u64, x1: u64, x2: u64, x3: u64, x4: u64) -> ! {
    bsp::drivers::mmio::init(MMIO_BASE);

    #[cfg(feature = "raspi4b")]
    log::init();

    log_write!("mmio initialized\r\n");

    log_write!("Welcome to TuoniOS!\r\n");
    log_write!("Kernel begin at: {:#16x}\r\n", x0);
    log_write!("Heap begin at: {:#16x}\r\n", x1);
    log_write!("Heap end at: {:#16x}\r\n", x2);
    log_write!("Stack begin at: {:#16x}\r\n", x3);
    log_write!("Stack end at: {:#16x}\r\n", x4);

    let heap_base = x1 as usize;
    allocator::init(heap_base, HEAP_SIZE);
    log_write!("allocator initialized\r\n");

    log::flush();

    for _ in 0..3 {
        log::write_char(3 as char);
    }

    let mut size: u32 = u32::from(log::read_byte());
    size |= u32::from(log::read_byte()) << 8;
    size |= u32::from(log::read_byte()) << 16;
    size |= u32::from(log::read_byte()) << 24;

    log::write_char('O');
    log::write_char('K');

    let kernel_addr: *mut u8 = LOAD_ADDR as *mut u8;
    unsafe {
        for i in 0..size {
            kernel_addr
                .offset(i as isize)
                .write_volatile(log::read_byte());
        }
    }

    log_write!(
        "Image loaded ({:?} bytes)! Beginning the execution now.\r\n",
        size
    );

    log::flush();

    let kernel: fn() -> ! = unsafe { core::mem::transmute(kernel_addr) };

    kernel();
}
