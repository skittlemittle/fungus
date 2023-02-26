use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use std::{error::Error, ffi::OsStr, fs};

/// bank of active samples
pub type ActiveSamples = Vec<StaticSoundData>;

/// Load up the samples innit
pub fn load() -> Result<ActiveSamples, Box<dyn Error>> {
    let mut samples: Vec<StaticSoundData> = vec![];

    let paths = fs::read_dir("./samples")?;

    for file in paths {
        let path = match file {
            Ok(f) => f.path(),
            Err(_) => continue,
        };

        if !path.is_dir() && path.extension() == Some(OsStr::new("wav")) {
            let sample = StaticSoundData::from_file(path, StaticSoundSettings::default())?;
            samples.push(sample);
        }
    }
    Ok(samples as ActiveSamples)
}
