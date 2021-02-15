use crate::vm::structures::BlockSpace;
use crate::genetic::definitions::{Individual, INDIVIDUAL_SIZE, TRAINING_EPOCHS};
use crate::genetic::operators::{mutate_population, crossover_population, select};
use crate::genetic::generation::generate_population;

pub fn train(target: &BlockSpace) -> Individual {
    let mut best_individual = [0; INDIVIDUAL_SIZE];
    let mut best_score = 0.0;

    let mut population = generate_population();

    for epoch in 0..TRAINING_EPOCHS {
        println!("Epoch {}...", epoch);
        mutate_population(&mut population);
        crossover_population(&mut population);
        let (new_population, generation_best_individual, generation_best_score) = select(population, &target);
        println!("  We have a best score of {}", generation_best_score);
        population = new_population;
        if generation_best_score > best_score {
            println!("  Replacing old best individual with:");
            println!("  {:?}", generation_best_individual);
            best_score = generation_best_score;
            best_individual = generation_best_individual;
        }
    }

    best_individual
}