/**
This module manages the playback thread
*/
use kira::manager::{backend::cpal::CpalBackend, AudioManager, AudioManagerSettings};
use kira::ClockSpeed;
use std::{error::Error, sync::mpsc::Receiver};

use crate::{sequencer::SampleSequence, sequencer::Sequence};

pub struct PlayBack {
    audio_manager: AudioManager,
    mute: bool,
    playing_sequence: SampleSequence,
}

impl PlayBack {
    /// set up audio playback
    pub fn setup() -> Result<PlayBack, Box<dyn Error>> {
        let m = AudioManager::<CpalBackend>::new(AudioManagerSettings::default())?;
        Ok(PlayBack {
            audio_manager: m,
            mute: true,
            playing_sequence: SampleSequence::new(0, 0),
        })
    }
}

pub trait Player {
    /// this is the playback loop
    fn begin_playback(&mut self, control_channel: Receiver<SampleSequence>) -> ();
}

impl Player for PlayBack {
    fn begin_playback(&mut self, control_channel: Receiver<SampleSequence>) -> () {
        self.mute = false;
        let mut step = 0;
        let ticks_per_step = 1;

        let metronome = self
            .audio_manager
            .add_clock(ClockSpeed::TicksPerMinute(100.0))
            .unwrap();

        metronome.start().unwrap();

        let mut time_last_loop = metronome.time();
        let mut ticks_elapsed: u64 = 0;

        loop {
            if self.mute {
                continue;
            }

            // check for control messages and handle them
            match control_channel.try_recv() {
                Ok(seq) => {
                    self.playing_sequence = seq;
                    step = 0;
                    // TODO: if update is a change dont reset step count
                }
                Err(_) => (),
            };

            let time_now = metronome.time();
            ticks_elapsed += time_now.ticks - time_last_loop.ticks;
            time_last_loop.ticks = time_now.ticks;

            if ticks_elapsed >= ticks_per_step {
                for t in self.playing_sequence.tracks() {
                    if t[step] {
                        print!("+");
                    } else {
                        print!("_");
                    }
                }
                println!("");

                if step == self.playing_sequence.steps() - 1 {
                    println!("\n");
                }
                // increment index
                step = (step + 1) % self.playing_sequence.steps();

                ticks_elapsed = 0;
            }

            // if something fucks up return break and return an err
        }
    }
}
