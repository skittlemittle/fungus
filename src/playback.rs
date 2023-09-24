/**
This module manages the playback thread
*/
use kira::manager::{backend::cpal::CpalBackend, AudioManager, AudioManagerSettings};
use kira::sound::static_sound::StaticSoundSettings;
use kira::track::{TrackBuilder, TrackHandle};
use kira::Volume;
use std::time::Duration;
use std::{error::Error, sync::mpsc::Receiver};

use spin_sleep;

use crate::samples::ActiveSamples;
use crate::sequencer::{AccentLevel, SampleSequence, Sequence};

/// controls for playback
pub struct Controls {
    pub tempo: u32,
    pub mute: bool,
}

static TEMPO_INIT: u64 = 180;

pub struct PlayBack {
    audio_manager: AudioManager,
    soft_hits: TrackHandle,
    accented_hits: TrackHandle,
    mute: bool,
    sequence: SampleSequence,
    samples: ActiveSamples,
}

impl PlayBack {
    /// set up the playback loop
    ///
    /// returns an error if the samples are empty, or if it cant spawn an "audiomanager"
    pub fn setup(samples: ActiveSamples) -> Result<PlayBack, Box<dyn Error>> {
        let mut m = AudioManager::<CpalBackend>::new(AudioManagerSettings::default())?;
        if samples.len() == 0 {
            return Err(Box::<dyn Error>::from("Empty sample bank"));
        }
        // soft and loud mixer tracks for accent levels
        let soft_hits = m.add_sub_track(TrackBuilder::new().volume(Volume::Decibels(-6.0)))?;
        let accented_hits = m.add_sub_track(TrackBuilder::new().volume(Volume::Decibels(6.0)))?;
        Ok(PlayBack {
            audio_manager: m,
            soft_hits,
            accented_hits,
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
    /// control_rx: channel to receive control commands
    ///
    /// division: how many steps to a beat, the usual music sense
    ///
    /// returns an error if kira cant play
    fn begin_playback(
        &mut self,
        sequence_rx: Receiver<SampleSequence>,
        control_rx: Receiver<Controls>,
        divisions: u32,
    ) -> Result<(), Box<dyn Error>>;
}

impl Player for PlayBack {
    fn begin_playback(
        &mut self,
        sequence_rx: Receiver<SampleSequence>,
        control_rx: Receiver<Controls>,
        division: u32,
    ) -> Result<(), Box<dyn Error>> {
        self.mute = false;
        let num_tracks = if self.sequence.num_tracks() > self.samples.len() {
            self.sequence.num_tracks()
        } else {
            self.samples.len()
        };

        let mut sequence_tracks = self.sequence.tracks();
        let mut step = 0;
        let mut bpm: u64 = TEMPO_INIT;

        loop {
            spin_sleep::sleep(Duration::from_millis(60_000 / (bpm * division as u64)));
            match control_rx.try_recv() {
                Ok(ctrl) => {
                    bpm = ctrl.tempo as u64;
                    self.mute = ctrl.mute;
                }
                Err(_) => (),
            }

            if self.mute {
                continue;
            }

            match sequence_rx.try_recv() {
                Ok(seq) => {
                    if self.sequence.num_tracks() != seq.num_tracks() {
                        step = 0;
                    }
                    self.sequence = seq;
                    sequence_tracks = self.sequence.tracks();
                }
                Err(_) => (),
            };

            /* === The actual playback logic === */

            for track in 0..num_tracks {
                let mixer_track = match sequence_tracks[track][step] {
                    AccentLevel::Silent => self.soft_hits.id(),
                    AccentLevel::Loud => self.accented_hits.id(),
                    _ => self.audio_manager.main_track().id(),
                };

                if sequence_tracks[track][step] != AccentLevel::Silent {
                    self.audio_manager.play(
                        self.samples[track]
                            .clone()
                            .with_settings(StaticSoundSettings::new().track(mixer_track)),
                    )?;
                }
            }

            step = (step + 1) % self.sequence.steps();
        }
    }
}
