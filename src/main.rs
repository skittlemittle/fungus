use std::error::Error;
use std::sync::mpsc;
use std::thread;

use fungus::test_ui::{Display, ScrContent};
fn main() {
    let disp = Display::begin();

    match play(disp) {
        Err(e) => println!("{}", e),
        Ok(()) => (),
    }
    Display::end();
}

use fungus::playback::{Controls, PlayBack, Player};
use fungus::samples;
use fungus::sequencer::{SampleSequence, Sequence};

fn play(display: Display) -> Result<(), Box<dyn Error>> {
    let mut seq = SampleSequence::new(3, 5);

    seq.set_step(0, 0, true)?;
    seq.set_step(0, 4, true)?;

    seq.set_step(1, 3, true)?;
    seq.set_step(1, 0, true)?;

    seq.set_step(2, 1, true)?;
    seq.set_step(2, 3, true)?;

    let samples = samples::load()?;

    let mut player = PlayBack::setup(samples)?;

    let (seq_tx, seq_rx) = mpsc::channel();
    let (control_tx, control_rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        // TODO: show (and log) error and restart playback
        player.begin_playback(seq_rx, control_rx).unwrap();
    });

    Display::update(
        &display,
        ScrContent {
            muted: false,
            tempo: 180,
            play: true,
            step: 3,
            sequence: &seq,
        },
    );

    seq_tx.send(seq.get_sequence())?;

    std::thread::sleep(std::time::Duration::from_secs(2));
    control_tx.send(Controls {
        tempo: 130,
        mute: true,
    })?;
    Display::update(
        &display,
        ScrContent {
            muted: true,
            tempo: 130,
            play: true,
            step: 3,
            sequence: &seq,
        },
    );

    std::thread::sleep(std::time::Duration::from_secs(4));
    control_tx.send(Controls {
        tempo: 130,
        mute: false,
    })?;

    seq.set_step(0, 0, false)?;
    seq_tx.send(seq.get_sequence())?;

    Display::update(
        &display,
        ScrContent {
            muted: false,
            tempo: 130,
            play: true,
            step: 3,
            sequence: &seq,
        },
    );

    handle.join().unwrap();

    Ok(())
}
