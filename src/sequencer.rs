/// A trck is a vec of trigger levels: 0 for silence, a positive int for how loud
type Track = Vec<u8>;

/// Stores information about when sounds get triggered
#[derive(Default)]
pub struct SampleSequence {
    tracks: Vec<Track>,
    steps: usize,
}

impl SampleSequence {
    /// new empty sequence
    ///
    /// num tracks: how many tracks it got
    ///
    /// num_steps: how many steps it got
    pub fn new(num_tracks: usize, num_steps: usize) -> SampleSequence {
        let mut tracks: Vec<Track> = vec![];

        for _ in 0..num_tracks {
            tracks.push(vec![0; num_steps]);
        }
        SampleSequence {
            tracks,
            steps: num_steps,
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
    /// trigger: sets the trigger, 0 for silence, positive int for an accent level.
    ///
    /// Returns an error if the track or step is out of bounds
    fn set_step(&mut self, track: usize, step: usize, trigger: u8) -> Result<(), &'static str>;

    /// returns the sequence
    fn get_sequence(&self) -> SampleSequence;

    /// returns the number of steps this sequence has
    fn steps(&self) -> usize;

    /// returns a copy of the tracks in this sequence
    fn tracks(&self) -> Vec<Track>;

    fn num_tracks(&self) -> usize;
}

impl Sequence for SampleSequence {
    fn clear_all(&mut self) {
        for i in 0..self.tracks.len() {
            self.tracks[i] = vec![0; self.steps];
        }
    }

    fn clear_track(&mut self, track: usize) {
        if track < self.tracks.len() {
            self.tracks[track] = vec![0; self.steps];
        }
    }

    fn set_step(&mut self, track: usize, step: usize, trigger: u8) -> Result<(), &'static str> {
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
}

#[allow(unused_must_use)]
#[cfg(test)]
mod tests {
    use super::{SampleSequence, Sequence};

    #[test]
    fn setting_steps() {
        let mut s = SampleSequence::new(2, 8);
        let mut ret = s.set_step(0, 3, 1);
        assert_eq!(ret.unwrap(), ());
        ret = s.set_step(0, 9, 1);
        assert!(ret.unwrap_err().len() != 0, "should have thrown an error");
        ret = s.set_step(2, 1, 3);
        assert!(ret.unwrap_err().len() != 0, "should have thrown an error");

        let check = s.get_sequence();
        assert_eq!(check.tracks()[0][3], 1);
        assert_eq!(check.tracks()[1][7], 0);
    }

    #[test]
    fn clearing() {
        let mut s = SampleSequence::new(3, 5);
        s.set_step(0, 2, 2);
        s.set_step(0, 4, 2);
        s.set_step(1, 0, 1);
        s.set_step(1, 4, 3);

        s.clear_track(1);
        assert_eq!(s.get_sequence().tracks[1][4], 0);
        assert_eq!(s.get_sequence().tracks[1][0], 0);
        assert_eq!(s.get_sequence().tracks[0][2], 2);
        s.clear_all();
        assert_eq!(s.get_sequence().tracks[0][2], 0);
    }

    #[test]
    fn getting() {
        let mut s = SampleSequence::new(3, 5);
        s.set_step(0, 2, 1);
        s.set_step(0, 4, 1);
        s.set_step(1, 0, 1);
        s.set_step(1, 4, 1);

        let t = s.tracks();
        assert_eq!(t[0][2], 1);
        assert_eq!(t[0][4], 1);
        assert_eq!(t[1][0], 1);
        assert_eq!(t[1][4], 1);

        for track in &t[2] {
            assert_eq!(*track, 0);
        }

        assert_eq!(s.steps(), 5);
    }
}
