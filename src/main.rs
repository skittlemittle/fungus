use fungus::test_ui::Display;

fn main() {
    let disp = Display::begin();

    match fungus::play(disp) {
        Err(e) => println!("{}", e),
        Ok(()) => (),
    }
    Display::end();
}
