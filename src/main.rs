#![no_std]
#![no_main]

use cortex_m_rt::entry;
use core::panic::PanicInfo;

#[entry]
fn main() -> ! {
    loop {
        
    }
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

