use std::{env, process};

use fungus::ui::Ui;

#[cfg(not(pi))]
use fungus::test_ui::Display;

#[cfg(pi)]
use fungus::actual_ui::HardUi;

fn main() {
    #[cfg(not(pi))]
    let disp = Display::new();
    #[cfg(pi)]
    let disp = HardUi::new().unwrap();

    let mut args = env::args();
    args.next();
    let steps = match args.next() {
        Some(a) => a.parse::<usize>().unwrap(),
        None => 0,
    };

    if steps <= 0 {
        println!("gotta have at least 1 step");
        process::exit(1);
    }

    match fungus::play(&disp, steps) {
        Err(e) => println!("{}", e),
        Ok(()) => (),
    }
    disp.end();
}
