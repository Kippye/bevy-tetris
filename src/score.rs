use crate::consts::{LINE_SCORE_BASE, TETRIS_MULTIPLIER, PLACE_SCORE_BASE};

#[derive(Default)]
pub struct ScoreResource {
    score: usize,
    lines: usize
}
impl ScoreResource {
    /// Increase score depending on how many lines were cleared and the current level
    pub fn reward_line_score(&mut self, lines: usize, level: usize) {
        // TODO: come up with proper formula
        self.score += LINE_SCORE_BASE * if lines == 4 { TETRIS_MULTIPLIER } else { lines } * (level + 1);
        self.lines += lines;
    }
    
    /// Increase score depending on how many pixels a piece was dropped
    pub fn reward_drop_score(&mut self, dropped_pixels: usize, level: usize) {
        // TODO: come up with proper formula
        self.score += dropped_pixels * PLACE_SCORE_BASE * (level + 1);
    }

    /// Reset score to 0
    pub fn reset(&mut self) {
        self.score = 0;
        self.lines = 0;
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn lines(&self) -> usize {
        self.lines
    }
}