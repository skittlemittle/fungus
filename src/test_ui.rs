/// curses testing ui
extern crate pancurses;

use crate::sequencer::{SampleSequence, Sequence};
use crate::ui::{Command, UIContent, Ui};
use pancurses::{endwin, initscr, Input};

pub struct Display {
    window: pancurses::Window,
}

impl Display {
    /// starts curses, call this before doing any other ui stuff
    pub fn new() -> Display {
        let window = initscr();
        window.nodelay(true);
        pancurses::noecho();

        Display { window }
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
}

impl Ui for Display {
    fn end(&self) {
        endwin();
    }

    /// updates the ui
    fn update(&self, content: UIContent) {
        self.window.clear();
        self.window.refresh();
        self.window.printw(&format!(
            "BPM: {} \t {} \n",
            content.tempo,
            if content.muted { "M" } else { "" },
        ));

        for track in Display::seq_format(content.sequence) {
            self.window.printw(&format!("{track} \n"));
        }
        self.window
            .mv(content.track as i32 + 1, content.step as i32);
        self.window.refresh();
    }

    fn get_command(&self) -> Command {
        match self.window.getch() {
            Some(Input::Character(c)) => c,
            Some(_) => '0',
            None => '0',
        }
    }
}
