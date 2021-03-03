use std::option::Option;
use rand::Rng;
use std::cmp::Ordering;

pub(crate) const SCORE_PRESENT_AIR: f64 = 0.0;
pub(crate) const SCORE_ABSENT_AIR: f64 = -0.5;
pub(crate) const SCORE_PRESENT_BLOCK: f64 = 1.0;
pub(crate) const SCORE_ABSENT_BLOCK: f64 = 0.0;
pub(crate) const SCORE_PROGRAM_ERROR: f64 = f64::NEG_INFINITY;

pub(crate) const CHROMOSOME_SIZE: usize = 256;
pub type Chromosome = [u8; CHROMOSOME_SIZE];

pub(crate) const POPULATION_SIZE: usize = 1000;
pub(crate) type Population = [Individual; POPULATION_SIZE];

#[derive(Debug, Copy, Clone)]
pub struct Individual {
    pub(crate) chromosome: Chromosome,
    pub(crate) score: Option<f64>,
}
impl Individual {
    pub(crate) fn new() -> Self {
        Self {
            chromosome: [0; CHROMOSOME_SIZE],
            score: None,
        }
    }

    pub(crate) fn random() -> Self {
        let mut chromosome = [0; CHROMOSOME_SIZE];
        let mut rng = rand::thread_rng();

        for byte in chromosome.iter_mut() {
            *byte = rng.gen_range(0..=255);
        }

        Self {
            chromosome,
            score: None,
        }
    }
}
impl PartialEq for Individual {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}
impl PartialOrd for Individual {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match other.score {
            None => Some(Ordering::Greater),
            Some(score) => self.score.unwrap().partial_cmp(&score),
        }
    }
}

pub(crate) const MUTATION_CHANCE: f64 = 0.05;

pub(crate) const MIN_SEGMENT_LENGTH: f64 = 1.0;
pub(crate) const ELITE_COUNT: usize = 100;
pub(crate) const RANDOM_COUNT: usize = 100;

pub(crate) const TRAINING_GENERATIONS: usize = 10000;

pub const MAX_PROGRAM_RUNTIME_MILLIS: u128 = 1000;

pub const DEBUG_DO_PRINTS: bool = false;