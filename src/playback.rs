/*
get the audiomanager handle,
accept a samplebank
accept a sequence
play them
*/

use kira::manager::{backend::cpal::CpalBackend, AudioManager, AudioManagerSettings};
use std::{error::Error, time::Duration};

use crate::{sequencer::SampleSequence, ActiveSamples};

pub struct PlayBack {
    audio_manager: AudioManager,
}

impl PlayBack {
    /// set up audio playback
    pub fn setup() -> Result<PlayBack, Box<dyn Error>> {
        let m = AudioManager::<CpalBackend>::new(AudioManagerSettings::default())?;
        Ok(PlayBack { audio_manager: m })
    }
}

// fn to update the sequence or a sample
// fn play_with: take sequence and sample bank then begin playing
// fn pause: stop
// fn play: start

pub trait Player {
    fn play_with(&mut self, sequence: SampleSequence, samples: ActiveSamples) -> ();
}

impl Player for PlayBack {
    fn play_with(&mut self, sequence: SampleSequence, sample_bank: ActiveSamples) -> () {
        self.audio_manager.play(sample_bank.samples[0].clone());
        std::thread::sleep(Duration::from_secs(2));
        self.audio_manager.play(sample_bank.samples[1].clone());
        std::thread::sleep(Duration::from_secs(2));
    }
}
