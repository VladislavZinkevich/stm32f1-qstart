#![no_std]
#![no_main]

use cortex_m_rt::entry;
use core::panic::PanicInfo;
use stm32f1::stm32f103 as pac;

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

