use definitions::*;

use crate::simulator::definitions::BlockSpace;

pub mod generation;
pub mod definitions;
pub mod evaluation;
mod selection;
mod crossover;
mod mutation;

pub fn train(target: &BlockSpace) -> definitions::Individual {
    let mut generation = Generation::random();

    for gen in 0..GEN_COUNT {
        generation.evaluate(target);
        let best_individual = &generation.population[generation.best_index.unwrap()];
        let best_result = &best_individual.result.unwrap();
        println!(
            "Generation {}, with best individual: score {}, depth {}",
            gen, best_result.score, best_individual.tree.get_max_depth()
        );
        if best_result.perfect {
            break;
        }
        let (kept_over, parent_pairs) = generation.select();
        generation = Generation::from_old(&generation, &kept_over, &parent_pairs);
        generation.mutate();
    }

    generation.population[generation.best_index.unwrap()].clone()
}