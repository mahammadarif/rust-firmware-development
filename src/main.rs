#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32f4xx_hal::{pac, prelude::*};
use embedded_hal::{digital::v2::OutputPin, blocking::delay::{DelayUs, DelayMs}};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(_cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        let rcc = dp.RCC.constrain();
        // F401 max SYSCLK=84MHz with 25MHz HSE
        let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(84.MHz()).freeze();

        // Create a delay abstraction based on general-purpose 32-bit timer TIM5
        let mut delay = dp.TIM5.delay_us(&clocks);

        // Acquire the GPIOC peripheral
        let gpioc = dp.GPIOC.split();

        let mut led = gpioc.pc13.into_push_pull_output();
        led.set_high(); // Off (active low on Black Pill)

        let message = ".... . .-.. .-.. ---"; // Fixed to HELLO

        loop {
            transmit_message(&mut led, &mut delay, &message).unwrap();
        }
    } else {
        loop {}
    }
}

fn transmit_message<LED, D>(
    led: &mut LED,
    delay: &mut D,
    message: &str
) -> Result<(),()>
where LED: OutputPin, D: DelayUs<u16> + DelayMs<u16>, {
    for char in message.chars() {
        match char {
            ' ' => delay.delay_ms(500), // Letter space
            '/' => delay.delay_ms(1000), // Word space (7 units, ~1000ms at 20 WPM)
            '.' => { // Dot: 200ms on
                led.set_low().map_err(|_| ())?;
                delay.delay_ms(200);
                led.set_high().map_err(|_| ())?;
            },
            '-' => { // Dash: 600ms on (3x dot)
                led.set_low().map_err(|_| ())?;
                delay.delay_ms(600);
                led.set_high().map_err(|_| ())?;
            },
            _ => {},
        }
        delay.delay_ms(200); // Inter-element pause (1 unit, ~200ms)
    }
    Ok(())
}
