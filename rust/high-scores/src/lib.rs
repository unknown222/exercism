#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a[u32]
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a[u32]) -> Self {
        HighScores {
            scores
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        Some(*self.scores.last()? as u32)
    }

    pub fn personal_best(&self) -> Option<u32> {
        Some(*self.scores.iter().max()? as u32)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut cloned_scores =  self.scores.to_vec();
        cloned_scores.sort();
        cloned_scores.reverse();
        cloned_scores.truncate(3);
        cloned_scores
    }
}
