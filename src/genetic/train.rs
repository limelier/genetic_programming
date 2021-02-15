use crate::vm::structures::BlockSpace;
use crate::genetic::definitions::{TRAINING_GENERATIONS, POPULATION_SIZE, Individual};
use crate::genetic::operators::{mutate_population, crossover_population, select};
use crate::genetic::evaluation::evaluate_population;

pub fn train(target: &BlockSpace) -> Individual {
    let mut best_individual = Individual::new();
    let mut population = [Individual::random(); POPULATION_SIZE];

    for generation in 0..TRAINING_GENERATIONS {
        println!("Gen. {}:", generation);
        mutate_population(&mut population);
        crossover_population(&mut population);

        evaluate_population(&mut population, target);
        let generation_best = &population[POPULATION_SIZE - 1];
        println!("  best: {}", generation_best.score.unwrap());
        if generation_best > &best_individual {
            best_individual = generation_best.clone();
            println!("  this is better than current best, replacing with: \n   {:?}", generation_best);
        }

        population = select(&mut population);
    }

    best_individual
}