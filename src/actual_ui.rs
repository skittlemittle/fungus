/// Actual rel (real) ui that runs on the actual (real) hardware.
/// button, encoders, and such.
use crate::ui::{Command, UIContent, Ui};

use rppal::gpio::Gpio;
use std::error::Error;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

pub mod rotary;

use rotary::Encoder;

pub struct HardUi {
    /// the the rotary encoders
    encoders_rx: Receiver<(usize, i32)>,
}

/*
Okay look, its not pretty or elegant or smart but i dont care and it does not matter
all the encoders are dumped into a list and shipped off, NO READING THEM HERE.
the polling function sends you a (index, val) tuple so what your soy ass is gonna do
is remember this:

0: track_select
1: tempo
*/

impl HardUi {
    pub fn new() -> Result<HardUi, Box<dyn Error>> {
        let (encoders_tx, encoders_rx) = mpsc::channel::<(usize, i32)>();

        let _handle = thread::spawn(move || {
            //TODO: unwrap
            let gpio = Gpio::new().unwrap();
            let track_select = Encoder::new(17, 18, &gpio).expect("pins already in use");
            let tempo = Encoder::new(22, 23, &gpio).expect("pins already in use");
            rotary::poll(encoders_tx, vec![track_select, tempo]);
        });

        Ok(HardUi { encoders_rx })
    }
}

impl Ui for HardUi {
    fn update(&self, content: UIContent) {}

    fn get_command(&self) -> Command {
        match self.encoders_rx.try_recv() {
            Ok(m) => match m.0 {
                0 => {
                    if m.1 > 0 {
                        'j'
                    } else {
                        'k'
                    }
                }
                1 => {
                    if m.1 > 0 {
                        '+'
                    } else {
                        '-'
                    }
                }
                _ => '0',
            },
            Err(_) => '0',
        }
    }

    fn end(&self) {}
}
