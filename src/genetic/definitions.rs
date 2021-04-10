use crate::trees::definitions::Node;

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub struct Result {
    pub(crate) score: f64,
    pub(crate) perfect: bool,
}

#[derive(Clone)]
pub struct Individual {
    pub tree: Node,
    pub result: Option<Result>,
}

pub struct Generation {
    pub population: Vec<Individual>,
    pub best_index: Option<usize>,
}

pub struct Parents {
    /// The index of the tree that'll act as the base during the crossover
    pub stock: usize,
    /// The index of the tree that'll provide the subtree during the crossover
    pub scion: usize,
}

pub(crate) const INDIVIDUALS_PER_METHOD_AND_DEPTH: usize = 10;
pub(crate) const MIN_GEN_DEPTH: usize = 2;
pub(crate) const MAX_GEN_DEPTH: usize = 6;
pub(crate) const POPULATION_SIZE: usize = (MAX_GEN_DEPTH - MIN_GEN_DEPTH + 1) * INDIVIDUALS_PER_METHOD_AND_DEPTH * 2;
pub(crate) const MAX_PROGRAM_RUNTIME_MS: u128 = 10;
pub(crate) const GEN_COUNT: usize = 1000;
pub(crate) const CROSSOVER_SIZE: usize = POPULATION_SIZE * 3 / 4;

pub(crate) const MAX_DEPTH: usize = 12;
pub(crate) const SCORE_PRESENT_AIR: f64 = 0.0;
pub(crate) const SCORE_ABSENT_AIR: f64 = -0.1;
pub(crate) const SCORE_PRESENT_BLOCK: f64 = 1.0;
pub(crate) const SCORE_ABSENT_BLOCK: f64 = 0.0;
pub(crate) const SCORE_PROGRAM_ERROR: f64 = f64::NEG_INFINITY;
pub(crate) const SCORE_DEPTH_LIMIT_EXCEEDED: f64 = f64::NEG_INFINITY;

pub(crate) const TOURNAMENT_SIZE: usize = 7;
pub(crate) const TOURNAMENT_P: f64 = 0.8;

pub(crate) const MUTATION_CHANCE: f64 = 0.05;