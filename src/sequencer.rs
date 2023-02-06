type Track = Vec<bool>;

/// Stores information about when sounds get triggered
#[derive(Default)]
pub struct SampleSequence {
    tracks: Vec<Track>,
    steps: usize,
    /// tempo of the sequence in beats per minute
    tempo: u32
}

impl SampleSequence {
    /// new empty sequence
    /// 
    /// num tracks: how many tracks it got
    /// 
    /// num_steps: how many steps it got
    /// 
    /// tempo: tempo of the sequence in BPM
    pub fn new(num_tracks: usize, num_steps: usize, tempo: u32) -> SampleSequence {
        let mut tracks: Vec<Track> = vec![];

        for _ in 0..num_tracks {
            tracks.push(vec![false; num_steps]);
        }
        SampleSequence {
            tracks,
            steps: num_steps,
            tempo
        }
    }
}

pub trait Sequence {
    /// clears all tracks
    fn clear_all(&mut self) -> ();

    /// clear a specific track
    fn clear_track(&mut self, track: usize) -> ();

    /// Set a step in a specific track.
    ///
    /// track: specifies the track, 0 indexed
    ///
    /// step: specifies the step, 0 indexed
    ///
    /// trigger: sets the trigger; true to play a sample, false to remain silent
    ///
    /// Returns an error if the track or step is out of bounds
    fn set_step(&mut self, track: usize, step: usize, trigger: bool) -> Result<(), &'static str>;

    /// returns the sequence
    fn get_sequence(&self) -> SampleSequence;

    /// returns the number of steps this sequence has
    fn steps(&self) -> usize;

    /// returns a copy of the tracks in this sequence
    fn tracks(&self) -> Vec<Track>;

    fn num_tracks(&self) -> usize;

    fn tempo(&self) -> u32;
}

impl Sequence for SampleSequence {
    fn clear_all(&mut self) {
        for i in 0..self.tracks.len() {
            self.tracks[i] = vec![false; self.steps];
        }
    }

    fn clear_track(&mut self, track: usize) {
        if track < self.tracks.len() {
            self.tracks[track] = vec![false; self.steps];
        }
    }

    fn set_step(&mut self, track: usize, step: usize, trigger: bool) -> Result<(), &'static str> {
        if track < self.tracks.len() && step < self.steps {
            self.tracks[track][step] = trigger;
            return Ok(());
        }
        Err("track or step index out of bounds")
    }

    fn get_sequence(&self) -> SampleSequence {
        SampleSequence {
            tracks: self.tracks.clone(),
            steps: self.steps,
            tempo: self.tempo
        }
    }

    fn steps(&self) -> usize {
        self.steps
    }

    fn tracks(&self) -> Vec<Track> {
        self.tracks.clone()
    }

    fn num_tracks(&self) -> usize {
        self.tracks.len()
    }

    fn tempo(&self) -> u32 {
        self.tempo
    }
}

#[allow(unused_must_use)]
#[cfg(test)]
mod tests {
    use super::{SampleSequence, Sequence};

    #[test]
    fn setting_steps() {
        let mut s = SampleSequence::new(2, 8, 60);
        let mut ret = s.set_step(0, 3, true);
        assert_eq!(ret.unwrap(), ());
        ret = s.set_step(0, 9, true);
        assert!(ret.unwrap_err().len() != 0, "should have thrown an error");
        ret = s.set_step(2, 1, true);
        assert!(ret.unwrap_err().len() != 0, "should have thrown an error");

        let check = s.get_sequence();
        assert_eq!(check.tracks()[0][3], true);
        assert_eq!(check.tracks()[1][7], false);
    }

    #[test]
    fn clearing() {
        let mut s = SampleSequence::new(3, 5, 180);
        s.set_step(0, 2, true);
        s.set_step(0, 4, true);
        s.set_step(1, 0, true);
        s.set_step(1, 4, true);

        s.clear_track(1);
        assert_eq!(s.get_sequence().tracks[1][4], false);
        assert_eq!(s.get_sequence().tracks[1][0], false);
        assert_eq!(s.get_sequence().tracks[0][2], true);
        s.clear_all();
        assert_eq!(s.get_sequence().tracks[0][2], false);
    }

    #[test]
    fn getting() {
        let mut s = SampleSequence::new(3, 5, 60);
        s.set_step(0, 2, true);
        s.set_step(0, 4, true);
        s.set_step(1, 0, true);
        s.set_step(1, 4, true);

        let t = s.tracks();
        assert_eq!(t[0][2], true);
        assert_eq!(t[0][4], true);
        assert_eq!(t[1][0], true);
        assert_eq!(t[1][4], true);

        for track in &t[2] {
            assert_eq!(*track, false);
        }

        assert_eq!(s.steps(), 5);

        assert_eq!(s.tempo(), 60);
    }
}
