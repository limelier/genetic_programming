use crate::trees::definitions::Node;

pub struct Individual {
    pub tree: Node,
    pub(crate) result: Option<f64>
}

pub struct Generation {
    pub population: Vec<Individual>,
    pub(crate) best: Option<Individual>,
}

pub(crate) const INDIVIDUALS_PER_METHOD_AND_DEPTH: usize = 10;
pub(crate) const MIN_DEPTH: usize = 2;
pub(crate) const MAX_DEPTH: usize = 6;
pub(crate) const POPULATION_SIZE: usize = (MAX_DEPTH - MIN_DEPTH + 1) * INDIVIDUALS_PER_METHOD_AND_DEPTH;