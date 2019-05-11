use std::thread;
use std::time::Duration;

use rppal::gpio::Error;
use rppal::gpio::Gpio;
use rppal::gpio::Level;
use rppal::gpio::OutputPin;

const MOTOR_BCM_PINS: [u8; 4] = [4, 17, 27, 22];
const BUTTON_UP_BCM_PIN: u8 = 23;
const BUTTON_DOWN_BCM_PIN: u8 = 24;

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

enum Direction {
    NoDirection,
    Up,
    Down,
}

fn main() -> Result<(), Error> {
    let gpio = match Gpio::new() {
        Ok(g) => g,
        Err(e) => return Err(e),
    };

    let mut motor_pins = match create_motor_pins(&gpio) {
        Ok(p) => p,
        Err(e) => return Err(e),
    };
    let button_up_pin = match gpio.get(BUTTON_UP_BCM_PIN) {
        Ok(p) => p.into_input(),
        Err(e) => return Err(e),
    };
    let button_down_pin = match gpio.get(BUTTON_DOWN_BCM_PIN) {
        Ok(p) => p.into_input(),
        Err(e) => return Err(e),
    };

    loop {
        let mut direction = Direction::NoDirection;
        if button_up_pin.is_high() {
            direction = Direction::Up;
        }
        if button_down_pin.is_high() {
            direction = Direction::Down;
        }
        match direction {
            Direction::NoDirection => {
                thread::sleep(Duration::from_millis(1));
                continue;
            },
            Direction::Up => {
                for step in HALFSTEP_SEQ.iter() {
                    move_motor(&mut motor_pins, *step)
                }
            },
            Direction::Down => {
                for step in HALFSTEP_SEQ.iter().rev() {
                    move_motor(&mut motor_pins, *step)
                }
            },
        };
    }
}

fn create_motor_pins(gpio: &Gpio) -> Result<Vec<OutputPin>, Error> {
    let mut pins = Vec::new();
    for bcm_pin in MOTOR_BCM_PINS.into_iter() {
        match gpio.get(*bcm_pin) {
            Ok(p) => pins.push(p.into_output()),
            Err(e) => return Err(e),
        }
    }
    return Ok(pins);
}

fn move_motor(motor_pins: &mut Vec<OutputPin>, step: [Level; 4]) {
    for (pin_index, level) in step.iter().enumerate() {
        motor_pins[pin_index].write(*level);
    }
    thread::sleep(Duration::from_millis(1));
}
