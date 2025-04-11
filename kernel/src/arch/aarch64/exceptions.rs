use core::arch::global_asm;

use crate::log_write;

global_asm!(include_str!("exceptions.s"));

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
pub extern "C" fn exception(frame: &mut InterruptFrame) {
    log_write!("Exception occured!\r\n");
    log_write!(
        "Frame:\r\n\
        x0={:<#16x}\r\n\
        x1={:<#16x}\r\n\
        x2={:<#16x}\r\n\
        x3={:<#16x}\r\n\
        x4={:<#16x}\r\n\
        x5={:<#16x}\r\n\
        x6={:<#16x}\r\n\
        x7={:<#16x}\r\n\
        x8={:<#16x}\r\n\
        x9={:<#16x}\r\n\
        x10={:<#16x}\r\n\
        x11={:<#16x}\r\n\
        x12={:<#16x}\r\n\
        x13={:<#16x}\r\n\
        x14={:<#16x}\r\n\
        x15={:<#16x}\r\n\
        x16={:<#16x}\r\n\
        x17={:<#16x}\r\n\
        x18={:<#16x}\r\n\
        fp={:<#16x}\r\n\
        lr={:<#16x}\r\n\
        esr={:<#16x}\r\n\
        far={:<#16x}\r\n",
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
pub extern "C" fn interrupt(frame: &mut InterruptFrame) {
    log_write!("Interrupt occured!\r\n");
    log_write!(
        "Frame:\r\n\
        x0={:<#16x}\r\n\
        x1={:<#16x}\r\n\
        x2={:<#16x}\r\n\
        x3={:<#16x}\r\n\
        x4={:<#16x}\r\n\
        5={:<#16x}\r\n\
        x6={:<#16x}\r\n\
        x7={:<#16x}\r\n\
        x8={:<#16x}\r\n\
        x9={:<#16x}\r\n\
        x10={:<#16x}\r\n\
        x11={:<#16x}\r\n\
        x12={:<#16x}\r\n\
        x13={:<#16x}\r\n\
        x14={:<#16x}\r\n\
        x15={:<#16x}\r\n\
        x16={:<#16x}\r\n\
        x17={:<#16x}\r\n\
        x18={:<#16x}\r\n\
        fp={:<#16x}\r\n\
        lr={:<#16x}\r\n\
        esr={}\r\n\
        far={:<#16x}\r\n",
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
