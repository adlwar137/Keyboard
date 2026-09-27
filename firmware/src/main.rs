#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f1xx_hal::{
    pac,
    prelude::*,
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    let mut gpioc = dp.GPIOC.split(&mut rcc);

    let mut led = gpioc
        .pc13
        .into_push_pull_output(&mut gpioc.crh);

    loop {
        led.set_low();
        cortex_m::asm::delay(8_000_000);

        led.set_high();
        cortex_m::asm::delay(8_000_000);
    }
}
