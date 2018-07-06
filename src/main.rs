// Sample code from https://github.com/japaric/f3/blob/master/examples/roulette.rs with minimal modifications

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate cortex_m;
extern crate f3;
extern crate panic_semihosting;

use f3::hal::delay::Delay;
use f3::hal::prelude::*;
use f3::hal::stm32f30x;
use f3::led::Leds;
use rt::ExceptionFrame;
use cortex_m::asm::bkpt;

entry!(main);

fn main() -> ! {
    bkpt();

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut leds = Leds::new(gpioe);
    let mut delay = Delay::new(cp.SYST, clocks);

    let n = leds.len();
    loop {
        for curr in 0..n {
            let next = (curr + 1) % n;
            leds[curr].off();
            leds[next].on();

            delay.delay_ms(100_u8);
        }
    }
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
