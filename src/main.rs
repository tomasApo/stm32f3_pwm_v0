#![deny(unsafe_code)]
#![no_main]
#![no_std]


use cortex_m::asm;
use cortex_m_rt::entry;
use panic_semihosting as _;
use stm32f3xx_hal::{self as hal, pac, prelude::*, pwm::tim3, time::rate::*};


#[entry]
fn main() -> ! {
    // dp = pac::Peripherals::take().unwrap();
    let dp = stm32f303::Peripherals::take().unwrap();


    let mut rcc = dp.RCC.constrain();
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);

    let mut led = gpioe
          .pe13
          .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    loop {
          led.toggle().unwrap();
          asm::delay(8_000_000);
    }
}
