use std::error::Error;
use std::sync::mpsc;
use std::thread;

pub mod playback;
pub mod samples;
pub mod sequencer;
pub mod test_ui;
pub mod ui;

use kira::sound::static_sound::StaticSoundData;
use playback::{Controls, PlayBack, Player};
use sequencer::{SampleSequence, Sequence};
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
pub fn play(display: &impl Ui) -> Result<(), Box<dyn Error>> {
    let samples = samples::load()?;

    let mut state = State {
        muted: false,
        tempo: 180,
        selected_track: 0,
        step: 0,
        sequence: SampleSequence::new(samples.len(), 8),
        samples,
    };

    let (seq_tx, seq_rx) = mpsc::channel();
    let (control_tx, control_rx) = mpsc::channel();
    let (step_tx, step_rx) = mpsc::channel();

    let mut player = PlayBack::setup(state.samples)?;

    let _playback_handle = thread::spawn(move || {
        // FIXME: unwrap lmao
        player.begin_playback(seq_rx, control_rx, step_tx).unwrap();
    });

    seq_tx.send(state.sequence.get_sequence())?;

    loop {
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
            'm' => {
                state.muted = !state.muted;
                send_control = true;
            }
            ' ' => {
                state
                    .sequence
                    .set_step(state.selected_track, state.step, true)?;
                send_control = true;
            }
            'c' => {
                state
                    .sequence
                    .set_step(state.selected_track, state.step, false)?;
                send_control = true;
            }
            'z' => {
                state.sequence.clear_track(state.selected_track);
                send_control = true;
            }
            _ => {}
        }

        let mut update_screen = false;
        match step_rx.try_recv() {
            Ok(s) => {
                state.step = s;
                update_screen = true
            }
            Err(_) => (),
        }

        if update_screen || command != '0' {
            display.update(UIContent {
                muted: state.muted,
                tempo: state.tempo,
                step: state.step,
                track: state.selected_track,
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
