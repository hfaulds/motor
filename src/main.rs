use std::thread;
use std::time::Duration;

use rppal::gpio::Error;
use rppal::gpio::Gpio;
use rppal::gpio::Level;

const BCM_PINS: [u8; 4] = [4, 17, 27, 22];

const HALFSTEP_SEQ: [[Level; 4]; 8] = [
    [Level::High, Level::Low, Level::Low, Level::Low],
    [Level::High, Level::High, Level::Low, Level::Low],
    [Level::Low, Level::High, Level::Low, Level::Low],
    [Level::Low, Level::High, Level::High, Level::Low],
    [Level::Low, Level::Low, Level::High, Level::Low],
    [Level::Low, Level::Low, Level::High, Level::High],
    [Level::Low, Level::Low, Level::Low, Level::High],
    [Level::High, Level::Low, Level::Low, Level::High],
];

fn main() -> Result<(), Error> {
    let gpio = match Gpio::new() {
        Ok(g) => g,
        Err(e) => return Err(e),
    };

    let mut pins = Vec::new();
    for bcm_pin in BCM_PINS.into_iter() {
        match gpio.get(*bcm_pin) {
            Ok(p) => pins.push(p.into_output()),
            Err(e) => return Err(e),
        }
    }

    for _ in 0..512 {
        for halfstep in HALFSTEP_SEQ.iter() {
            for (pin_index, level) in halfstep.iter().enumerate() {
                pins[pin_index].write(*level)
            }
            thread::sleep(Duration::from_millis(1));
        }
    }

    return Ok(());
}
