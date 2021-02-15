use crate::vm::structures::BlockSpace;
use crate::genetic::definitions::{Individual, INDIVIDUAL_SIZE, TRAINING_EPOCHS, POPULATION_SIZE};
use crate::genetic::operators::{mutate_population, crossover_population, select};

pub fn train(target: &BlockSpace) -> Individual {
    let mut best_individual = [0; INDIVIDUAL_SIZE];
    let mut best_score = 0.0;
    let mut population = [[0; INDIVIDUAL_SIZE]; POPULATION_SIZE];

    for epoch in 0..TRAINING_EPOCHS {
        println!("Beginning epoch {}", epoch);
        mutate_population(&mut population);
        crossover_population(&mut population);
        let (new_population, generation_best_individual, generation_best_score) = select(population, &target);
        population = new_population;
        if generation_best_score > best_score {
            best_score = generation_best_score;
            best_individual = generation_best_individual;
        }
    }

    best_individual
}