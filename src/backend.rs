use std::thread;
use std::time::Duration;
use std::error::Error;

use rppal::gpio::{Gpio, OutputPin};

// Pulse width modulation
#[allow(dead_code)]
pub fn business_logic() -> Result<(), Box<dyn Error>> {
    println!("Running application");
    
    // Setup
    let gpio: Gpio = Gpio::new()?;
    let led_pin_nr: u8 = 21;

    // Init pins
    let mut led_pin: OutputPin = gpio.get(led_pin_nr)?.into_output();

    let mut x: u8 = 0;
    loop {

        led_pin.toggle();
        thread::sleep(Duration::from_millis(1000));

        if x >= 2 {
            led_pin.set_low();
            break;
        }
        x += 1;
    }

    Ok(())
}
