/// curses testing ui
extern crate pancurses;

use crate::sequencer::{SampleSequence, Sequence};
use pancurses::{endwin, initscr};

pub struct Display {
    window: pancurses::Window,
}

pub struct ScrContent<'a> {
    pub muted: bool,
    pub tempo: u32,
    pub play: bool,
    pub step: usize,
    pub track: usize,
    pub sequence: &'a SampleSequence,
}

impl Display {
    /// starts curses, call this before doing any other ui stuff
    pub fn begin() -> Display {
        let window = initscr();
        window.nodelay(true);
        pancurses::noecho();

        Display { window }
    }
    pub fn end() {
        endwin();
    }

    fn seq_format(sequence: &SampleSequence) -> Vec<String> {
        sequence
            .tracks()
            .iter()
            .map(|track| {
                track
                    .iter()
                    .map(|&step| if step { "+" } else { "_" })
                    .collect()
            })
            .collect::<Vec<String>>()
    }

    /// updates the ui
    pub fn update(&self, content: ScrContent) {
        self.window.clear();
        self.window.refresh();
        self.window.printw(&format!(
            "BPM: {} \t {} \n {} \n",
            content.tempo,
            if content.muted { "M" } else { "" },
            if content.play { "Playing" } else { "Paused" }
        ));

        for track in Display::seq_format(content.sequence) {
            self.window.printw(&format!("{track} \n"));
        }
        self.window
            .mv(content.track as i32 + 2, content.step as i32);
        self.window.refresh();
    }

    pub fn getch(&self) -> Option<pancurses::Input> {
        self.window.getch()
    }
}
