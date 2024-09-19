#![no_std]
#![no_main]
use core::arch::global_asm;
use core::fmt::Write;
use core::mem::MaybeUninit;
use core::ptr::addr_of;

pub mod arch;
pub mod drivers;
pub mod locking;
pub mod mem;
pub mod panic;

global_asm!(include_str!("boot.s"));

const HEAP_SIZE: usize = 0x100000;

const MMIO_BASE: *mut u32 = 0xffff_0000_0800_0000 as *mut u32;

#[global_allocator]
static ALLOCATOR: mem::heap::LockedHeap = mem::heap::LockedHeap::new(mem::heap::Heap::new());

#[no_mangle]
pub extern "C" fn kernel_main(x0: u64, x1: u64, x2: u64) -> ! {
    let mmio = unsafe { arch::mmio::Mmio::new(MMIO_BASE) };
    let mut uart = unsafe { drivers::serial::Uart::new(mmio) };

    writeln!(uart, "Welcome to TuoniOS!").unwrap();
    writeln!(uart, "Kernel begin at: {:#16x}", x0).unwrap();
    writeln!(uart, "Early heap begin at: {:#16x}", x1).unwrap();
    writeln!(uart, "Stack end at: {:#16x}", x2).unwrap();

    let heap_base = addr_of!(x1) as *mut u8;
    unsafe {
        ALLOCATOR.lock().init(heap_base, HEAP_SIZE);
    }

    loop {
        uart.write_byte(uart.read_byte() as u8)
    }
}
