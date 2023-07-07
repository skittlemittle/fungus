use fungus::test_ui::Display;
use fungus::ui::Ui;

fn main() {
    let disp = Display::new();

    match fungus::play(&disp) {
        Err(e) => println!("{}", e),
        Ok(()) => (),
    }
    disp.end();
}
