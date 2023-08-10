/// Actual rel (real) ui that runs on the actual (real) hardware.
/// button, encoders, and such.
use crate::ui::{Command, UIContent, Ui};

use rppal::gpio::Gpio;
use std::error::Error;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

pub mod button;
pub mod rotary;

use button::{Button, Momentary};
use rotary::{Encoder, Rotary};

pub struct HardUi {
    /// the the rotary encoders
    encoders_rx: Receiver<(usize, i32)>,
    /// the buttons
    buttons_rx: Receiver<usize>,
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
        let (buttons_tx, buttons_rx) = mpsc::channel::<usize>();

        let _handle = thread::spawn(move || {
            //TODO: unwrap
            let gpio = Gpio::new().unwrap();
            let track_select = Encoder::new(17, 18, &gpio).expect("pins already in use");
            let tempo = Encoder::new(22, 23, &gpio).expect("pins already in use");
            let place_beat = Button::new(3, &gpio).expect("pin already in use");
            poll(
                encoders_tx,
                buttons_tx,
                vec![track_select, tempo],
                vec![place_beat],
            );
        });

        Ok(HardUi {
            encoders_rx,
            buttons_rx,
        })
    }
}

impl Ui for HardUi {
    fn update(&self, content: UIContent) {}

    fn get_command(&self) -> Command {
        let clicked_buttons = match self.buttons_rx.try_recv() {
            Ok(b) => match b {
                0 => ' ',
                _ => '0',
            },
            Err(_) => '0',
        };

        if clicked_buttons != '0' {
            return clicked_buttons;
        }

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

/// poll the inputs
/// encoder_tx: sends a tuple: (index_of_encoder, value)
/// button_tx: sends the index of the button what got pressed
/// encoders: vec of encoders
/// buttons: vec of buttons
pub fn poll(
    encoder_tx: Sender<(usize, i32)>,
    button_tx: Sender<usize>,
    mut encoders: Vec<Encoder>,
    mut buttons: Vec<Button>,
) {
    loop {
        for i in 0..encoders.len() {
            let now = encoders[i].update();
            if now != 0 {
                encoder_tx.send((i, now));
            }
        }

        for i in 0..buttons.len() {
            let now = buttons[i].clicked();
            if now {
                button_tx.send(i);
            }
        }
        // TODO: adaptive wait? so it always takes ~1ms to check again?
        thread::sleep(Duration::from_millis(1));
    }
}
