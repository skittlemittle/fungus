use std::error::Error;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub mod playback;
pub mod samples;
pub mod sequencer;
pub mod ui;

#[cfg(not(pi))]
pub mod test_ui;

#[cfg(pi)]
pub mod actual_ui;

use kira::sound::static_sound::StaticSoundData;
use playback::{Controls, PlayBack, Player};
use sequencer::{AccentLevel, SampleSequence, Sequence};
use ui::{UIContent, Ui};

/// app? state :trollface:
struct State {
    pub tempo: u32,
    pub muted: bool,
    pub selected_track: usize,
    pub step: usize,
    pub sequence: SampleSequence,
    pub samples: Vec<StaticSoundData>,
}

/// The program control loop
///
/// display: handle to UI
pub fn play(display: &impl Ui, steps: usize, divisions: u32) -> Result<(), Box<dyn Error>> {
    let samples = samples::load()?;

    let mut state = State {
        muted: false,
        tempo: 180,
        selected_track: 0,
        step: 0,
        sequence: SampleSequence::new(samples.len(), steps),
        samples,
    };

    let (seq_tx, seq_rx) = mpsc::channel();
    let (control_tx, control_rx) = mpsc::channel();

    let mut player = PlayBack::setup(state.samples)?;

    let _playback_handle = thread::spawn(move || {
        // FIXME: unwrap lmao
        player
            .begin_playback(seq_rx, control_rx, divisions)
            .unwrap();
    });

    seq_tx.send(state.sequence.get_sequence())?;

    loop {
        thread::sleep(Duration::from_millis(10));
        let command = display.get_command();

        let mut send_control = false;
        match command {
            'k' => {
                if state.selected_track >= 1 {
                    state.selected_track -= 1;
                }
            }
            'j' => {
                if state.selected_track < state.sequence.num_tracks() - 1 {
                    state.selected_track += 1;
                }
            }
            'l' => {
                if state.step < state.sequence.steps() {
                    state.step += 1;
                }
            }
            'h' => {
                if state.step > 0 {
                    state.step -= 1;
                }
            }

            'm' => {
                state.muted = !state.muted;
                send_control = true;
            }
            // hits
            ' ' => {
                state
                    .sequence
                    .set_step(state.selected_track, state.step, AccentLevel::Regular)?;
                send_control = true;
            }
            'd' => {
                state
                    .sequence
                    .set_step(state.selected_track, state.step, AccentLevel::Loud)?;
                send_control = true;
            }
            's' => {
                state
                    .sequence
                    .set_step(state.selected_track, state.step, AccentLevel::Soft)?;
                send_control = true;
            }
            // clearing
            'c' => {
                state
                    .sequence
                    .set_step(state.selected_track, state.step, AccentLevel::Silent)?;
                send_control = true;
            }
            'C' => {
                state.sequence.clear_track(state.selected_track);
                send_control = true;
            }
            '+' => {
                state.tempo += 1;
                send_control = true;
            }
            '-' => {
                state.tempo -= 1;
                send_control = true;
            }
            _ => {}
        }

        if command != '0' {
            display.update(UIContent {
                muted: state.muted,
                tempo: state.tempo,
                step: state.step,
                track: state.selected_track,
                divisions,
                sequence: &state.sequence,
            })
        }

        if send_control {
            control_tx.send(Controls {
                tempo: state.tempo,
                mute: state.muted,
            })?;

            seq_tx.send(state.sequence.get_sequence())?;
        }
    }

    Ok(())
}
