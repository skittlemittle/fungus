/// curses testing ui
#[cfg(not(pi))]
extern crate pancurses;

use crate::sequencer::{AccentLevel, SampleSequence, Sequence};
use crate::ui::{Command, UIContent, Ui};
use pancurses::{endwin, init_pair, initscr, start_color, Input};
use pancurses::{COLOR_BLACK, COLOR_RED};

pub struct Display {
    window: pancurses::Window,
}

impl Display {
    /// starts curses, call this before doing any other ui stuff
    pub fn new() -> Display {
        let window = initscr();
        window.nodelay(true);
        pancurses::noecho();
        start_color();
        init_pair(1, COLOR_RED, COLOR_BLACK);

        Display { window }
    }
    fn seq_format(sequence: &SampleSequence) -> Vec<String> {
        sequence
            .tracks()
            .iter()
            .map(|track| {
                track
                    .iter()
                    .map(|&step| match step {
                        AccentLevel::Loud => "#",
                        AccentLevel::Regular => "+",
                        AccentLevel::Soft => "-",
                        AccentLevel::Silent => "_",
                    })
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

        let mut color: bool;
        for track in Display::seq_format(content.sequence) {
            color = true;
            for (i, step) in track.chars().enumerate() {
                if i % content.divisions as usize == 0 {
                    self.window.attrset(pancurses::COLOR_PAIR(color as u32));
                    color = !color;
                }
                self.window.addstr(&format!("{step}"));
            }
            self.window.printw("\n");
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
