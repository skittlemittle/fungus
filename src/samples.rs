use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use std::error::Error;

/// bank of active samples
pub type ActiveSamples = Vec<StaticSoundData>;

/// Load up the samples innit
pub fn load() -> Result<ActiveSamples, Box<dyn Error>> {
    let mut samples: Vec<StaticSoundData> = vec![];

    for i in 1..4 {
        let sample =
            StaticSoundData::from_file(format!("{i}.wav"), StaticSoundSettings::default())?;
        samples.push(sample);
    }
    Ok(samples as ActiveSamples)
}

