#![no_std]
#![no_main]

//cuse cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;

//use stm32f3xx_hal as hal;
use stm32f3::stm32f303;
/*
use hal::pac;
use hal::flash::FlashExt;
use hal::time::U32Ext;
use hal::pwm::tim3;
use hal::hal::PwmPin;
use hal::gpio::GpioExt;
use hal::rcc::RccExt;
*/

#[entry]
fn main() -> ! {
        
        
        //let dp = pac::Peripherals::take().unwrap();
        
        use rtt_target::{rprintln};
        
        let peripherals = stm32f303::Peripherals::take().unwrap();
        let mut flash = peripherals.FLASH;
        let mut rcc = peripherals.RCC;
        let clocks = &rcc.cfgr;
        
        
        let mut gpioa = &peripherals.GPIOA(&mut flash.acr);
        gpioa.odr.modify(|_, w| w.odr0().set_bit());
        let pa6 = gpioa;


        //clocks 
        //let mut flash = dp.FLASH.constrain();
        
    
        
        peripherals.TIM3.ccmr1_output();
        let tim3_ch1 = &peripherals.TIM3;
        tim3_ch1.ccmr1_output();
        
        let mut tim3_ch1 = 
        
        
        
        // Each channel can be used with a different duty cycle and have many pins
        //let mut tim3_ch1 = tim3.ccmr1_output(pa6);

        //tim3_ch1.get_max_duty() = 1280
        
        loop {
            /*
            tim3_ch1.set_duty(tim3_ch1.get_max_duty()/10); // 5% duty cyle
            tim3_ch1.enable();
            cortex_m::asm::delay(5_000_000);
            tim3_ch1.set_duty(tim3_ch1.get_max_duty()/20); // 10% duty cyle
            tim3_ch1.enable();
            cortex_m::asm::delay(5_000_000);
             */
            //rtt_init_print!();
            rprintln!("{:?}",tim3_ch1);
        };    
}   