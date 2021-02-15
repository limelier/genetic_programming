use crate::genetic::definitions::{INDIVIDUAL_SIZE, Individual, Population, POPULATION_SIZE};
use rand::Rng;

pub(crate) fn generate_individual() -> Individual {
    let mut individual = [0; INDIVIDUAL_SIZE];
    let mut rng = rand::thread_rng();

    for byte in individual.iter_mut() {
        *byte = rng.gen_range(0..=255);
    }

    individual
}

pub(crate) fn generate_population() -> Population {
    let mut population = [[0; INDIVIDUAL_SIZE]; POPULATION_SIZE];
    // keep at least one starting individual the null program (all passes)
    for individual in population[1..].iter_mut() {
        *individual = generate_individual();
    }

    population
}

