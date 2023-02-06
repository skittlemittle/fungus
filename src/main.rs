use std::error::Error;
use std::sync::mpsc;
use std::thread;

fn main() {
    match play() {
        Err(e) => println!("{}", e),
        Ok(()) => (),
    }
}

use fungus::playback::{PlayBack, Player};
use fungus::samples;
use fungus::sequencer::{SampleSequence, Sequence};

fn play() -> Result<(), Box<dyn Error>> {
    let mut seq = SampleSequence::new(2, 5);

    seq.set_step(0, 0, true)?;
    seq.set_step(0, 4, true)?;

    seq.set_step(1, 3, true)?;
    seq.set_step(1, 0, true)?;

    let samples = samples::load()?;

    let mut player = PlayBack::setup(samples)?;

    let (seq_tx, seq_rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        // TODO: show (and log) error and restart playback
        player.begin_playback(seq_rx).unwrap();
    });

    seq_tx.send(seq.get_sequence())?;

    std::thread::sleep(std::time::Duration::from_secs(4));

    seq.set_step(0, 0, false)?;
    seq_tx.send(seq.get_sequence())?;

    handle.join().unwrap();

    Ok(())
}
