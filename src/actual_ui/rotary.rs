use rppal::gpio::{Gpio, InputPin};
use std::error::Error;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

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
            value: 0,
            last_encoded: 0,
        })
    }
}

/// rotary encoder reading logic
pub trait Rotary {
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

/// tx: sends a tuple: (index_of_encoder, value)
/// encoders: vec of encoders
pub fn poll(tx: Sender<(usize, i32)>, mut encoders: Vec<Encoder>) {
    loop {
        for i in 0..encoders.len() {
            let now = encoders[i].update();
            if now != 0 {
                tx.send((i, now));
            }
        }
        // TODO: adaptive wait? so it always takes ~1ms to check again?
        thread::sleep(Duration::from_millis(1));
    }
}
