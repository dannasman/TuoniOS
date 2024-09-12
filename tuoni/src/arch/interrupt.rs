#![no_std]
use core::fmt::Write;

//use crate::drivers::*;
//use crate::mmio::*;

use crate::arch;
use crate::drivers;

const MMIO_BASE: *mut u32 = 0x0800_0000 as *mut u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct InterruptFrame {
    x0: u64,
    x1: u64,
    x2: u64,
    x3: u64,
    x4: u64,
    x5: u64,
    x6: u64,
    x7: u64,
    x8: u64,
    x9: u64,
    x10: u64,
    x11: u64,
    x12: u64,
    x13: u64,
    x14: u64,
    x15: u64,
    x16: u64,
    x17: u64,
    x18: u64,
    fp: u64,
    lr: u64,
    xzr: u64,
    esr: u64,
    far: u64,
}

#[no_mangle]
pub extern "C" fn exception(frame: *mut InterruptFrame) {
    unsafe { safe_exception(&mut *frame) }
}

fn safe_exception(_frame: &mut InterruptFrame) {
    let mut mmio = unsafe { arch::mmio::Mmio::new(MMIO_BASE) };
    let mut uart = unsafe { drivers::serial::Uart::new(mmio) };

    writeln!(uart, "exception occured").unwrap();
    loop {}
}

#[no_mangle]
pub extern "C" fn interrupt(frame: *mut InterruptFrame) {
    unsafe { safe_interrupt(&mut *frame) }
}

fn safe_interrupt(_frame: &mut InterruptFrame) {
    let mut mmio = unsafe { arch::mmio::Mmio::new(MMIO_BASE) };
    let mut uart = unsafe { drivers::serial::Uart::new(mmio) };

    writeln!(uart, "interrupt occured").unwrap();
    loop {}
}
