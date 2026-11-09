#![no_std]
#![no_main]
use cortex_m_rt::entry;                  // Marks our embedded entry function.
use stm32f4xx_hal::{pac, prelude::*};     // Hardware registers and HAL extension traits.
use core::fmt::Write; // Enables formatted text output through the transmitter.

#[entry]                                // Called by the runtime after initialization.
fn main() -> ! {                        // This function never returns.
    let peripherals = pac::Peripherals::take() // Obtain the device's peripheral handles once.
        .expect("Peripherals already taken"); // Panic if they were already taken.

    let mut rcc = peripherals.RCC.constrain(); // Access reset and clock control through the HAL.
    let gpiob = peripherals.GPIOB.split(&mut rcc); // Enable port B and obtain its pin handles.
    let mut led = gpiob.pb7.into_push_pull_output(); // Configure PB7 to drive HIGH or LOW.

    let gpiod = peripherals.GPIOD.split(&mut rcc); // Enable port D and obtain its pins.
    let mut tx = peripherals.USART3.tx(
        gpiod.pd8,               // PD8 carries transmitted data to ST-LINK.
        115_200.bps(),            // Configure the serial communication speed.
        &mut rcc,                // Borrow clock control to configure USART3.
    ).expect("USART3 configuration failed"); // Panic if configuration fails.

    let mut delay = peripherals.TIM5.delay_us(&mut rcc); // Configure TIM5 for timing.

    let mut cycle_count: u32 = 0; // Mutable 32-bit unsigned counter, initially zero.
    loop {
        cycle_count = cycle_count.wrapping_add(1); // Increment; wrap to zero on overflow.

        writeln!(tx, "Hello from STM32! Cycle: {cycle_count}\r") // Send text followed by CR and LF.
            .expect("Serial write failed"); // Panic if writing fails.
        led.set_high();              // Turn the blue LED on.
        delay.delay(5.secs());    // Wait 5 seconds.
        led.set_low();               // Turn the blue LED off.
        delay.delay(5.secs());    // Wait another 5 seconds.
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}