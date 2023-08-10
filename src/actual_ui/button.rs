use rppal::gpio::{Gpio, InputPin};
use std::error::Error;

/// pushbutton, active LOW
pub struct Button {
    bpin: InputPin,
    state: u16,
}

impl Button {
    /// takes a BCM pin number and returns a new Button, fails if the
    /// given pin is already being used
    pub fn new(pin: u8, gpio: &Gpio) -> Result<Button, Box<dyn Error>> {
        Ok(Button {
            bpin: gpio.get(pin)?.into_input_pullup(),
            state: 0,
        })
    }
}
pub trait Momentary {
    /// did the button get pressed, only reports the transition from off to on
    /// returns true if the button went from off to on, false otherwise
    fn clicked(&mut self) -> bool;
}

impl Momentary for Button {
    fn clicked(&mut self) -> bool {
        self.state = (self.state << 1) | self.bpin.is_high() as u16 | 0xfe00;
        self.state == 0xff00
    }
}
