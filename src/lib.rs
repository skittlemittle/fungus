use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use std::error::Error;

pub mod sequencer;
pub mod playback;

/// bank of active samples
pub struct ActiveSamples {
    // the loaded samples
    pub samples: Vec<StaticSoundData>,
}

impl ActiveSamples {
    /// Load up the samples innit
    pub fn load() -> Result<ActiveSamples, Box<dyn Error>> {
        let mut samples: Vec<StaticSoundData> = vec![];

        for i in 1..3 {
            let sample = StaticSoundData::from_file(format!("{i}.ogg"), StaticSoundSettings::default())?;
            samples.push(sample);
        }
        Ok(ActiveSamples { samples })
    }
}
