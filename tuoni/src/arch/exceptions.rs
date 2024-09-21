#![no_std]
use core::fmt::Write;

use crate::log_write;

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

fn safe_exception(frame: &mut InterruptFrame) {
    log_write!("Exception occured!\n");
    log_write!(
        "Frame:\nx0={:<#16x}\tx1={:<#16x}\tx2={:<#16x}\tx3={:<#16x}\n\
        x4={:<#16x}\tx5={:<#16x}\tx6={:<#16x}\tx7={:<#16x}\n\
        x8={:<#16x}\tx9={:<#16x}\tx10={:<#16x}\tx11={:<#16x}\n\
        x12={:<#16x}\tx13={:<#16x}\tx14={:<#16x}\tx15={:<#16x}\n\
        x16={:<#16x}\tx17={:<#16x}\tx18={:<#16x}\nfp={:<#16x}\n\
        lr={:<#16x}\nesr={:<#16x}\nfar={:<#16x}\n",
        frame.x0,
        frame.x1,
        frame.x2,
        frame.x3,
        frame.x4,
        frame.x5,
        frame.x6,
        frame.x7,
        frame.x8,
        frame.x9,
        frame.x10,
        frame.x11,
        frame.x12,
        frame.x13,
        frame.x14,
        frame.x15,
        frame.x16,
        frame.x17,
        frame.x18,
        frame.fp,
        frame.lr,
        frame.esr,
        frame.far
    );
    loop {}
}

#[no_mangle]
pub extern "C" fn interrupt(frame: *mut InterruptFrame) {
    unsafe { safe_interrupt(&mut *frame) }
}

fn safe_interrupt(frame: &mut InterruptFrame) {
    log_write!("Interrupt occured!\n");
    log_write!(
        "Frame:\nx0={:<#16x}\tx1={:<#16x}\tx2={:<#16x}\tx3={:<#16x}\n\
        x4={:<#16x}\tx5={:<#16x}\tx6={:<#16x}\tx7={:<#16x}\n\
        x8={:<#16x}\tx9={:<#16x}\tx10={:<#16x}\tx11={:<#16x}\n\
        x12={:<#16x}\tx13={:<#16x}\tx14={:<#16x}\tx15={:<#16x}\n\
        x16={:<#16x}\tx17={:<#16x}\tx18={:<#16x}\nfp={:<#16x}\n\
        lr={:<#16x}\nesr={}\nfar={:<#16x}\n",
        frame.x0,
        frame.x1,
        frame.x2,
        frame.x3,
        frame.x4,
        frame.x5,
        frame.x6,
        frame.x7,
        frame.x8,
        frame.x9,
        frame.x10,
        frame.x11,
        frame.x12,
        frame.x13,
        frame.x14,
        frame.x15,
        frame.x16,
        frame.x17,
        frame.x18,
        frame.fp,
        frame.lr,
        frame.esr,
        frame.far
    );
    loop {}
}
