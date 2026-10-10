#![no_std]
#![no_main]

// stm32f103c6

//First value of the Interrupt Vector Table is the Stack Pointer address
//Second value of the Interrupt Vector Table is the Program Counter address

use core::panic::PanicInfo;


#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_handler() -> ! {
    //inline asm to copy memory

    let _x = 42;

    loop {}
}

#[unsafe(link_section = ".vector_table.reset_vector")]
#[unsafe(no_mangle)]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = reset_handler;


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}