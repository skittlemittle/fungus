use kira::manager::{backend::cpal::CpalBackend, AudioManager, AudioManagerSettings};
use std::{error::Error, time::Duration};

fn main() {
    match play() {
        Err(e) => println!("{}", e),
        Ok(()) => (),
    }
}

use fungus::sequencer::{SampleSequence,Sequence};
use fungus::playback::{PlayBack, Player};

fn play() -> Result<(), Box<dyn Error>> {
    let mut seq = SampleSequence::new(2, 5);
    
    seq.set_step(0, 0, true)?;
    seq.set_step(0, 4, true)?;

    seq.set_step(1, 3, true)?;
    seq.set_step(1, 0, true)?;

    let samples = fungus::ActiveSamples::load()?;

    let mut player = PlayBack::setup()?;

    player.play_with(seq, samples);

    Ok(())
}
