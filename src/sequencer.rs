#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AccentLevel {
    /// no sound
    Silent,
    Soft,
    Regular,
    Loud,
}

/// A trck is a vec of trigger levels: 0 for silence, a positive int for how loud
type Track = Vec<AccentLevel>;

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
            tracks.push(vec![AccentLevel::Silent; num_steps]);
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
    /// trigger: sets a beat at an accent level for this step.
    ///
    /// Returns an error if the track or step is out of bounds
    fn set_step(
        &mut self,
        track: usize,
        step: usize,
        trigger: AccentLevel,
    ) -> Result<(), &'static str>;

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
            self.tracks[i] = vec![AccentLevel::Silent; self.steps];
        }
    }

    fn clear_track(&mut self, track: usize) {
        if track < self.tracks.len() {
            self.tracks[track] = vec![AccentLevel::Silent; self.steps];
        }
    }

    fn set_step(
        &mut self,
        track: usize,
        step: usize,
        trigger: AccentLevel,
    ) -> Result<(), &'static str> {
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
    use super::{AccentLevel, SampleSequence, Sequence};

    #[test]
    fn setting_steps() {
        let mut s = SampleSequence::new(2, 8);
        let mut ret = s.set_step(0, 3, AccentLevel::Soft);
        assert_eq!(ret.unwrap(), ());
        ret = s.set_step(1, 5, AccentLevel::Loud);
        ret = s.set_step(0, 9, AccentLevel::Soft);
        assert!(ret.unwrap_err().len() != 0, "should have thrown an error");
        ret = s.set_step(2, 1, AccentLevel::Loud);
        assert!(ret.unwrap_err().len() != 0, "should have thrown an error");

        let check = s.get_sequence();
        assert_eq!(check.tracks()[0][3], AccentLevel::Soft);
        assert_eq!(check.tracks()[1][5], AccentLevel::Loud);
        assert_eq!(check.tracks()[1][7], AccentLevel::Silent);
    }

    #[test]
    fn clearing() {
        let mut s = SampleSequence::new(3, 5);
        s.set_step(0, 2, AccentLevel::Regular);
        s.set_step(0, 4, AccentLevel::Regular);
        s.set_step(1, 0, AccentLevel::Soft);
        s.set_step(1, 4, AccentLevel::Loud);

        s.clear_track(1);
        assert_eq!(s.get_sequence().tracks[1][4], AccentLevel::Silent);
        assert_eq!(s.get_sequence().tracks[1][0], AccentLevel::Silent);
        assert_eq!(s.get_sequence().tracks[0][2], AccentLevel::Regular);
        s.clear_all();
        assert_eq!(s.get_sequence().tracks[0][2], AccentLevel::Silent);
    }

    #[test]
    fn getting() {
        let mut s = SampleSequence::new(3, 5);
        s.set_step(0, 2, AccentLevel::Regular);
        s.set_step(0, 4, AccentLevel::Regular);
        s.set_step(1, 0, AccentLevel::Regular);
        s.set_step(1, 4, AccentLevel::Regular);

        let t = s.tracks();
        assert_eq!(t[0][2], AccentLevel::Regular);
        assert_eq!(t[0][4], AccentLevel::Regular);
        assert_eq!(t[1][0], AccentLevel::Regular);
        assert_eq!(t[1][4], AccentLevel::Regular);

        for track in &t[2] {
            assert_eq!(*track, AccentLevel::Silent);
        }

        assert_eq!(s.steps(), 5);
    }
}
