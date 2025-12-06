#[derive(PartialEq, Clone, Copy)]
pub enum Direction { Negative, Positive }

impl Direction {
    pub fn get_multiplier(&self) -> f32 {
        match *self {
            Direction::Negative => -1.0,
            Direction::Positive => 1.0,
        }
    }
}