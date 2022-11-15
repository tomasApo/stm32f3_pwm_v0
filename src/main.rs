#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use stm32f3xx_hal as hal;
use hal::pac;
use hal::prelude::*;
use hal::pwm::{tim3,tim16};
use hal::time::rate::*;




#[entry]
fn main() -> ! {
  



    //set the resolution of our duty cycle to 9000 
    //period to 50Hz

    // Set the resolution of our duty cycle to 9000 and our period to
    // 50Hz.

    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();

    let mut rcc = dp.RCC.constrain();
    
    let clock = rcc.cfgr.freeze(&mut flash.acr);
  
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
  
    // Set the resolution of our duty cycle to 9000 and our period to
    // 50Hz.
    let mut c1_no_pins = tim16(device.TIM3, 9000, 50.Hz(), clocks);
    loop {
          asm::delay(8_000_000);
    }
}   
 