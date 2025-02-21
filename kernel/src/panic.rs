use core::panic::PanicInfo;

use crate::log_write;

#[panic_handler]
fn on_panic(info: &PanicInfo) -> ! {
    log_write!("KERNEL PANIC: {}\n", info);
    loop {}
}
