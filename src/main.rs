#![no_std]
#![no_main]

//cuse cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;

use stm32f3xx_hal as hal;

use hal::pac;
use hal::flash::FlashExt;
use hal::time::U32Ext;
use hal::pwm::tim3;
use hal::hal::PwmPin;
use hal::gpio::GpioExt;
use hal::rcc::RccExt;

#[entry]
fn main() -> ! {
        let dp = pac::Peripherals::take().unwrap();
        use rtt_target::{rprintln, rtt_init_print};
        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
        //let pa4 = gpioa.pa4.into_af2(&mut gpioa.moder, &mut gpioa.afrl);
        let pa6 = gpioa.pa6.into_af2(&mut gpioa.moder, &mut gpioa.afrl);

        let tim3_channels = tim3(
            dp.TIM3,
            1280,    // resolution of duty cycle
            50.hz(), // frequency of period
            &clocks, // To get the timer's clock speed
        );

        // Each channel can be used with a different duty cycle and have many pins
        let mut tim3_ch1 = tim3_channels.0.output_to_pa6(pa6);

        //tim3_ch1.get_max_duty() = 1280
        
        loop {
            tim3_ch1.set_duty(tim3_ch1.get_max_duty()/10); // 5% duty cyle
            tim3_ch1.enable();
            cortex_m::asm::delay(5_000_000);
            tim3_ch1.set_duty(tim3_ch1.get_max_duty()/20); // 10% duty cyle
            tim3_ch1.enable();
            cortex_m::asm::delay(5_000_000);
            //rtt_init_print!();
            //rprintln!("{}",tim3_ch1.get_max_duty());
            
        }     
}