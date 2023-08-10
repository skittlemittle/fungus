use rppal::gpio::{Gpio, InputPin};
use std::error::Error;

/// the data tied to a rotary encoder
pub struct Encoder {
    a: InputPin,
    b: InputPin,
    last_encoded: u8,
}

impl Encoder {
    /// takes BCM pin numbers for the encoders pins and makes a new encoder instance.
    pub fn new(pina: u8, pinb: u8, gpio: &Gpio) -> Result<Encoder, Box<dyn Error>> {
        let a = gpio.get(pina)?.into_input_pullup();
        let b = gpio.get(pinb)?.into_input_pullup();

        Ok(Encoder {
            a,
            b,
            last_encoded: 0,
        })
    }
}

/// rotary encoder reading logic
pub trait Rotary {
    /// check the encoder and return back the direction of movement
    /// returns: 0 for nothing, 1 for clockwise, -1 for anticlockwise
    /// poll this regularly, like ~900Hz
    fn update(&mut self) -> i32;
}

impl Rotary for Encoder {
    fn update(&mut self) -> i32 {
        let msb = self.a.is_high() as u8;
        let lsb = self.b.is_high() as u8;

        let encoded = (msb << 1) | lsb;
        let sum = (self.last_encoded << 2) | encoded;

        let mut direction = 0; // each step triggers a send
        if sum == 0b1101 || sum == 0b0100 || sum == 0b0010 || sum == 0b1011 {
            direction = 1;
        }
        if sum == 0b1110 || sum == 0b0111 || sum == 0b0001 || sum == 0b1000 {
            direction = -1;
        }

        self.last_encoded = encoded;
        direction
    }
}
