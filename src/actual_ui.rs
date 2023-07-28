/// Actual rel (real) ui that runs on the actual (real) hardware.
/// button, encoders, and such.
use crate::ui::{Command, UIContent, Ui};

use rppal::gpio::{Gpio, InputPin};
use std::error::Error;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

struct Encoder {
    a: InputPin,
    b: InputPin,
    pub value: i32,
    last_encoded: u8,
}

impl Encoder {
    /// takes BCM pin numbers for the encoders pins and makes a new encoder instance.
    pub fn new(pina: u8, pinb: u8, gpio: Gpio) -> Result<Encoder, Box<dyn Error>> {
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

trait Rotary {
    fn update(&mut self) -> ();
}

impl Rotary for Encoder {
    fn update(&mut self) {
        let msb = self.a.is_high() as u8;
        let lsb = self.b.is_high() as u8;

        let encoded = (msb << 1) | lsb;
        let sum = (self.last_encoded << 2) | encoded;

        if sum == 0b1101 || sum == 0b0100 || sum == 0b0010 || sum == 0b1011 {
            self.value += 1;
        }
        if sum == 0b1110 || sum == 0b0111 || sum == 0b0001 || sum == 0b1000 {
            self.value -= 1;
        }

        self.last_encoded = encoded;
    }
}

pub struct HardUi {
    tempo_knob: Receiver<i32>,
}

impl HardUi {
    pub fn new() -> Result<HardUi, Box<dyn Error>> {
        let gpio = Gpio::new()?;
        let mut t_enc = Encoder::new(17, 18, gpio)?;

        // make loop thread here
        // pass it the encoder
        // keep its channel recv here and we read from it ova here
        let (polled_inputs_tx, polled_inputs_rx) = mpsc::channel::<i32>();

        let _handle = thread::spawn(move || {
            poll(polled_inputs_tx, t_enc);
        });

        Ok(HardUi {
            tempo_knob: polled_inputs_rx,
        })
    }
}

impl Ui for HardUi {
    fn update(&self, content: UIContent) {}

    fn get_command(&self) -> Command {
        match self.tempo_knob.try_recv() {
            Ok(m) => println!("{}", m),
            Err(_) => (),
        };
        '0'
    }

    fn end(&self) {}
}

/// button and rotary polling
fn poll(tx: Sender<i32>, mut encoder: Encoder) {
    let mut last: i32 = 0;
    loop {
        encoder.update();
        let now = encoder.value;
        if last != now {
            tx.send(now);
        }
        last = now;

        thread::sleep(Duration::from_millis(1));
    }
}
