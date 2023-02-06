/**
This module manages the playback thread
*/
use kira::manager::{backend::cpal::CpalBackend, AudioManager, AudioManagerSettings};
use kira::ClockSpeed;
use std::{error::Error, sync::mpsc::Receiver};

use crate::{samples::ActiveSamples, sequencer::SampleSequence, sequencer::Sequence};

pub struct PlayBack {
    audio_manager: AudioManager,
    mute: bool,
    sequence: SampleSequence,
    samples: ActiveSamples,
}

impl PlayBack {
    /// set up the playback loop
    ///
    /// returns an error is samples has no samples in it
    pub fn setup(samples: ActiveSamples) -> Result<PlayBack, Box<dyn Error>> {
        let m = AudioManager::<CpalBackend>::new(AudioManagerSettings::default())?;
        if samples.len() == 0 {
            return Err(Box::<dyn Error>::from("Empty sample bank"));
        }
        Ok(PlayBack {
            audio_manager: m,
            mute: true,
            sequence: Default::default(),
            samples,
        })
    }
}

pub trait Player {
    /// the playback loop
    ///
    /// sequence_rx: channel to receive sequence changes
    ///
    /// returns an error if kira cant play
    fn begin_playback(
        &mut self,
        sequence_rx: Receiver<SampleSequence>,
    ) -> Result<(), Box<dyn Error>>;
}

impl Player for PlayBack {
    fn begin_playback(
        &mut self,
        sequence_rx: Receiver<SampleSequence>,
    ) -> Result<(), Box<dyn Error>> {
        self.mute = false;
        let num_tracks = if self.sequence.num_tracks() > self.samples.len() {
            self.sequence.num_tracks()
        } else {
            self.samples.len()
        };

        let mut sequence_tracks = self.sequence.tracks();

        let mut step = 0;
        let ticks_per_step = 1;
        let metronome = self
            .audio_manager
            .add_clock(ClockSpeed::TicksPerMinute(100.0))?;

        metronome.start()?;

        let mut time_last_loop = metronome.time();
        let mut ticks_elapsed: u64 = 0;

        loop {
            if self.mute {
                continue;
            }

            // check for control messages and handle them
            match sequence_rx.try_recv() {
                Ok(seq) => {
                    self.sequence = seq;
                    sequence_tracks = self.sequence.tracks();
                    step = 0;
                    // TODO: if update is a change dont reset step count
                }
                Err(_) => (),
            };

            let time_now = metronome.time();
            ticks_elapsed += time_now.ticks - time_last_loop.ticks;
            time_last_loop.ticks = time_now.ticks;

            if ticks_elapsed >= ticks_per_step {
                for track in 0..num_tracks {
                    if sequence_tracks[track][step] {
                        print!("+");
                        self.audio_manager.play(self.samples[track].clone())?;
                    } else {
                        print!("_");
                    }
                }
                println!("");

                if step == self.sequence.steps() - 1 {
                    println!("\n");
                }

                step = (step + 1) % self.sequence.steps();

                ticks_elapsed = 0;
            }
        }
    }
}
