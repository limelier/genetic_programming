use crate::vm::structures::BlockSpace;
use crate::genetic::definitions::{TRAINING_GENERATIONS, POPULATION_SIZE, Individual};
use crate::genetic::operators::{mutate_population, crossover_population, select};

pub fn train(target: &BlockSpace) -> Individual {
    let mut best_individual = Individual::new();
    let mut population = [Individual::random(); POPULATION_SIZE];

    for generation in 0..TRAINING_GENERATIONS {
        println!("Generation {}...", generation);
        mutate_population(&mut population);
        crossover_population(&mut population);
        let (new_population, generation_best_individual) = select(&mut population, &target);
        let generation_best_score = generation_best_individual.score.unwrap();
        println!("  We have a best score of {}", generation_best_score);
        population = new_population;
        if best_individual.score == None || generation_best_score > best_individual.score.unwrap() {
            println!("  Replacing old best individual with:");
            println!("  {:?}", generation_best_individual);
            best_individual = generation_best_individual;
        }
    }

    best_individual
}