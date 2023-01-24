use kira::manager::{backend::cpal::CpalBackend, AudioManager, AudioManagerSettings};
use std::{error::Error, time::Duration};

fn main() {
    match play() {
        Err(e) => println!("{}", e),
        Ok(()) => (),
    }
}

fn play() -> Result<(), Box<dyn Error>> {
    let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default())?;

    let sample_bank = fungus::ActiveSamples::load()?;
    manager.play(sample_bank.samples[0].clone())?;
    std::thread::sleep(Duration::from_secs(2));
    manager.play(sample_bank.samples[1].clone())?;
    std::thread::sleep(Duration::from_secs(2));

    Ok(())
}
