#![no_std]
#![no_main]
use core::alloc::Layout;
use core::arch::global_asm;
use core::fmt::Write;
use core::mem::{align_of, size_of};
use core::ptr::addr_of;

pub mod arch;
pub mod drivers;
pub mod mem;
pub mod panic;
pub mod sync;

global_asm!(include_str!("boot.s"));

const HEAP_SIZE: usize = 0x100000;

const MMIO_BASE: *mut u32 = 0xffff_0000_0800_0000 as *mut u32;

#[no_mangle]
pub extern "C" fn kernel_main(x0: u64, x1: u64, x2: u64, x3: u64, x4: u64) -> ! {
    let mmio = unsafe { arch::mmio::Mmio::new(MMIO_BASE) };
    let mut uart = unsafe { drivers::serial::Uart::new(mmio) };

    writeln!(uart, "Welcome to TuoniOS!").unwrap();
    writeln!(uart, "Kernel begin at: {:#16x}", x0).unwrap();
    writeln!(uart, "Heap begin at: {:#16x}", x1).unwrap();
    writeln!(uart, "Heap end at: {:#16x}", x2).unwrap();
    writeln!(uart, "Stack begin at: {:#16x}", x3).unwrap();
    writeln!(uart, "Stack end at: {:#16x}", x4).unwrap();

    let heap_base = x1 as usize;
    mem::init(heap_base, HEAP_SIZE);

    let test1 = unsafe {
        mem::alloc(Layout::from_size_align_unchecked(
            size_of::<u32>(),
            align_of::<u32>(),
        ))
    };
    let test2 = unsafe {
        mem::alloc(Layout::from_size_align_unchecked(
            size_of::<sync::mutex::Mutex<u32>>(),
            align_of::<sync::mutex::Mutex<u32>>(),
        ))
    };

    writeln!(
        uart,
        "Address returned by alloc for u32: {:#16x}",
        test1 as usize
    )
    .unwrap();
    writeln!(
        uart,
        "Address returned by alloc for Mutex<u32>: {:#16x}",
        test2 as usize
    )
    .unwrap();

    loop {
        uart.write_byte(uart.read_byte() as u8)
    }
}
