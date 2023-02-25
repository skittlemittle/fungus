use std::error::Error;
use std::sync::mpsc;
use std::thread;

use pancurses::Input;

pub mod playback;
pub mod samples;
pub mod sequencer;
pub mod test_ui;

use kira::sound::static_sound::StaticSoundData;
use playback::{Controls, PlayBack, Player};
use sequencer::{SampleSequence, Sequence};
use test_ui::{Display, ScrContent};

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
/// display: handle to curses display
pub fn play(display: Display) -> Result<(), Box<dyn Error>> {
    let samples = samples::load()?;

    let mut state = State {
        muted: false,
        tempo: 100,
        selected_track: 0,
        step: 0,
        sequence: SampleSequence::new(samples.len(), 8),
        samples,
    };

    let (seq_tx, seq_rx) = mpsc::channel();
    let (control_tx, control_rx) = mpsc::channel();
    let (step_tx, step_rx) = mpsc::channel();

    let mut player = PlayBack::setup(state.samples)?;

    let playback_handle = thread::spawn(move || {
        // FIXME: unwrap lmao
        player.begin_playback(seq_rx, control_rx, step_tx).unwrap();
    });

    seq_tx.send(state.sequence.get_sequence())?;

    loop {
        let command = match Display::getch(&display) {
            Some(Input::Character(c)) => c,
            Some(_) => '0',
            None => '0',
        };

        let mut should_update = false;
        match command {
            'j' => {
                if state.selected_track >= 1 {
                    state.selected_track -= 1;
                }
            }
            'k' => {
                if state.selected_track < state.sequence.num_tracks() - 1 {
                    state.selected_track += 1;
                }
            }
            'm' => {
                state.muted = !state.muted;
                should_update = true;
            }
            ' ' => {
                state
                    .sequence
                    .set_step(state.selected_track, state.step, true)?;
                should_update = true;
            }
            'c' => {
                state
                    .sequence
                    .set_step(state.selected_track, state.step, false)?;
                should_update = true;
            }
            'z' => {
                state.sequence.clear_track(state.selected_track);
                should_update = true;
            }
            _ => {}
        }

        match step_rx.try_recv() {
            Ok(s) => state.step = s,
            Err(_) => (),
        }

        if !should_update {
            continue;
        }

        control_tx.send(Controls {
            tempo: state.tempo,
            mute: state.muted,
        })?;

        seq_tx.send(state.sequence.get_sequence())?;

        Display::update(
            &display,
            ScrContent {
                muted: state.muted,
                tempo: state.tempo,
                play: true,
                step: state.step,
                sequence: &state.sequence,
            },
        )
    }

    Ok(())
}
