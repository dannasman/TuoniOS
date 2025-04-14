use core::fmt;

use crate::bsp::drivers::serial;
use crate::sync;

static mut UART: sync::mutex::Mutex<serial::Uart> = sync::mutex::Mutex::new(serial::Uart::new());

struct Log;

impl Log {
    fn write_string(&mut self, msg: &str) {
        for c in msg.bytes() {
            unsafe {
                UART.lock().write_byte(c);
            }
        }
    }

    fn write_byte(&mut self, b: u8) {
        unsafe { UART.lock().write_byte(b) };
    }

    fn read_byte() -> u8 {
        unsafe { UART.lock().read_byte() }
    }
}

impl fmt::Write for Log {
    fn write_str(&mut self, msg: &str) -> fmt::Result {
        self.write_string(msg);
        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.write_byte(c as u8);
        Ok(())
    }
}

fn log() -> impl fmt::Write {
    Log {}
}

#[cfg(feature = "raspi4b")]
pub fn init() {
    unsafe { UART.lock().init() }
}

pub fn write_fmt(args: fmt::Arguments) {
    use core::fmt::Write;
    log().write_fmt(args).unwrap();
}

pub fn write_char(c: char) {
    use core::fmt::Write;
    log().write_char(c).unwrap();
}

pub fn read_byte() -> u8 {
    Log::read_byte()
}

pub fn flush() {
    unsafe { UART.lock().flush() };
}

#[macro_export]
macro_rules! log_write {
    ($($arg:tt)*) => ($crate::log::write_fmt(format_args!($($arg)*)));
}
