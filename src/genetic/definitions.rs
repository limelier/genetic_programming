
use crate::trees::definitions::Node;

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub struct Result {
    pub(crate) score: f64,
    pub dice_index: f64,
    pub(crate) perfect: bool,
}

#[derive(Debug, Clone)]
pub struct Individual {
    pub tree: Node,
    pub result: Option<Result>,
}

pub struct Generation {
    pub population: Vec<Individual>,
    pub best_index: Option<usize>,
    pub worst_index: Option<usize>,
}

pub struct Parents {
    /// The index of the tree that'll act as the base during the crossover
    pub stock: usize,
    /// The index of the tree that'll provide the subtree during the crossover
    pub scion: usize,
}

#[allow(dead_code)]
pub(crate) enum SelectionMethod {
    FitnessWeighted,
    Tournament,
}


// Selection
pub(crate) const SELECTION_METHOD: SelectionMethod = SelectionMethod::Tournament;
// TOURNAMENT ONLY
<<<<<<< HEAD
<<<<<<< HEAD
pub(crate) const TOURNAMENT_SIZE: usize = 2;
pub(crate) const TOURNAMENT_P: f64 = 0.8;
=======
pub(crate) const TOURNAMENT_SIZE: usize = 7;
pub(crate) const TOURNAMENT_P: f64 = 1.0;
>>>>>>> master
=======
pub(crate) const TOURNAMENT_SIZE: usize = 2;
pub(crate) const TOURNAMENT_P: f64 = 0.8;
>>>>>>> 60c1416e45b16a60c4e33ca328542d8fc0183c44
// FITNESS-WEIGHTED ONLY
pub(crate) const SELECTION_PRESSURE: f64 = 1.0;

// Generation
pub(crate) const INDIVIDUALS_PER_METHOD_AND_DEPTH: usize = 5;
pub(crate) const MIN_GEN_DEPTH: usize = 2;
pub(crate) const MAX_GEN_DEPTH: usize = 6;
pub(crate) const POPULATION_SIZE: usize = (MAX_GEN_DEPTH - MIN_GEN_DEPTH + 1) * INDIVIDUALS_PER_METHOD_AND_DEPTH * 2;
pub(crate) const GEN_COUNT: usize = 1000;
pub(crate) const P_GROW_LEAF: f64 = 0.1;

// Evaluation
pub(crate) const MAX_DEPTH: usize = 12;
pub(crate) const MAX_PROGRAM_RUNTIME_MS: u128 = 10;
/// The bigger, the less important tree depth is in individual evaluation
pub(crate) const DEPTH_SOFTENER: f64 = 6.0;
pub(crate) const SCORE_PROGRAM_ERROR: f64 = f64::NEG_INFINITY;
pub(crate) const SCORE_DEPTH_LIMIT_EXCEEDED: f64 = f64::NEG_INFINITY;

// Mutation
<<<<<<< HEAD
<<<<<<< HEAD
pub(crate) const MUTATION_CHANCE: f64 = 0.05;
pub(crate) const MUTATION_SINGLE_CHANCE: f64 = 0.10;

// Crossover
pub(crate) const CROSSOVER_SIZE: usize = (POPULATION_SIZE as f64 * 0.5) as usize;
=======
pub(crate) const MUTATION_CHANCE: f64 = 0.02;
pub(crate) const MUTATION_SINGLE_CHANCE: f64 = 0.05;

// Crossover
pub(crate) const CROSSOVER_SIZE: usize = (POPULATION_SIZE as f64 * 0.75) as usize;
>>>>>>> master
=======
pub(crate) const MUTATION_CHANCE: f64 = 0.00;
pub(crate) const MUTATION_SINGLE_CHANCE: f64 = 0.05;

// Crossover
pub(crate) const CROSSOVER_SIZE: usize = (POPULATION_SIZE as f64 * 0.5) as usize;
>>>>>>> 60c1416e45b16a60c4e33ca328542d8fc0183c44

