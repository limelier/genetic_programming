pub(crate) const SCORE_PRESENT_AIR: f64 = 0.0;
pub(crate) const SCORE_ABSENT_AIR: f64 = -0.5;
pub(crate) const SCORE_PRESENT_BLOCK: f64 = 1.0;
pub(crate) const SCORE_ABSENT_BLOCK: f64 = 0.0;

pub(crate) const INDIVIDUAL_SIZE: usize = 256;
pub type Individual = [u8; INDIVIDUAL_SIZE];

pub(crate) const MUTATION_CHANCE: f64 = 0.05;
