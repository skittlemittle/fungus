use crate::sequencer::SampleSequence;

pub type Command = char;

pub struct UIContent<'a> {
    pub muted: bool,
    pub tempo: u32,
    pub step: usize,
    pub track: usize,
    pub sequence: &'a SampleSequence,
}

pub trait Ui {
    /// stop display
    fn end(&self) -> ();

    /// update UI
    fn update(&self, content: UIContent) -> ();

    /// returns char command for user inputs, '0' for no command
    fn get_command(&self) -> Command;
}
