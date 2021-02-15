pub(crate) const SCORE_PRESENT_AIR: f64 = 0.0;
pub(crate) const SCORE_ABSENT_AIR: f64 = -0.5;
pub(crate) const SCORE_PRESENT_BLOCK: f64 = 1.0;
pub(crate) const SCORE_ABSENT_BLOCK: f64 = 0.0;
pub(crate) const SCORE_PROGRAM_ERROR: f64 = f64::NEG_INFINITY;

pub(crate) const INDIVIDUAL_SIZE: usize = 256;
pub type Individual = [u8; INDIVIDUAL_SIZE];

pub(crate) const POPULATION_SIZE: usize = 100;
pub(crate) type Population = [Individual; POPULATION_SIZE];

pub(crate) const MUTATION_CHANCE: f64 = 0.1;

pub(crate) const MIN_SEGMENT_LENGTH: f64 = 1.0;

pub(crate) const TRAINING_EPOCHS: usize = 100;

pub const MAX_PROGRAM_RUNTIME_MILLIS: u128 = 1000;

pub const DEBUG_DO_PRINTS: bool = false;