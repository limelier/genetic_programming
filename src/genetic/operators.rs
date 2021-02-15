use crate::genetic::definitions::{Individual, MUTATION_CHANCE, INDIVIDUAL_SIZE, Population, POPULATION_SIZE, MIN_SEGMENT_LENGTH};
use rand::Rng;
use crate::genetic::evaluation::evaluate_individual;
use crate::vm::structures::BlockSpace;

pub(crate) fn mutate(individual: &mut Individual) {
    let mut rng = rand::thread_rng();

    for byte in individual {
        let roll: f64 = rng.gen();

        *byte = if roll <= MUTATION_CHANCE {
            rng.gen_range(0..=255)
        } else {
            *byte
        }
    }
}

pub(crate) fn mutate_population(population: &mut Population) {
    for individual in population {
        mutate(individual);
    }
}

pub(crate) fn crossover(first: &mut Individual, second: &mut Individual) {
    let mut rng = rand::thread_rng();
    let cut_point = rng.gen_range(1..INDIVIDUAL_SIZE - 1);
    let swap_before: bool = rng.gen();

    let swap_range = if swap_before {
        0..cut_point
    } else {
        cut_point..INDIVIDUAL_SIZE
    };

    for i in swap_range {
        let aux = first[i];
        first[i] = second[i];
        second[i] = aux;
    }
}

pub(crate) fn crossover_population(population: &mut Population) {
    for i in 0..POPULATION_SIZE / 2 {
        let mut first = population[i];
        let mut second = population[i + POPULATION_SIZE / 2];
        crossover(&mut first, &mut second);
        population[i] = first;
        population[i + POPULATION_SIZE / 2] = second;
    }
}

pub(crate) fn select(population: Population, target: &BlockSpace) -> (Population, Individual, f64) {
    // evaluate
    let mut score_lower_bound = evaluate_individual(&population[0], target);
    let mut score_upper_bound = score_lower_bound;
    let mut best_individual = &population[0];
    let mut scores = [score_lower_bound; POPULATION_SIZE];

    for (i, individual) in population[1..].iter().enumerate() {
        let score = evaluate_individual(individual, target);

        if score < score_lower_bound {
            score_lower_bound = score;
        }

        if score > score_upper_bound {
            score_upper_bound = score;
            best_individual = &population[i];
        }

        scores[i] = score;
    }

    // construct the "wheel of fortune"
    let mut segment_length = [0.0; POPULATION_SIZE];
    for (i, score) in scores.iter().enumerate() {
        segment_length[i] = MIN_SEGMENT_LENGTH + score;
    }
    let wheel_size = segment_length.iter().sum();

    // spin the wheel to build the new population
    let mut new_population = [[0; INDIVIDUAL_SIZE]; POPULATION_SIZE];
    let mut rng = rand::thread_rng();
    for i in 0..POPULATION_SIZE {
        let mut needle = rng.gen_range(0.0..wheel_size);
        let mut chosen = 0;

        while needle > segment_length[chosen] {
            needle -= segment_length[chosen];
            chosen += 1;
        }

        new_population[i] = population[chosen].clone();
    }

    (new_population, best_individual.clone(), score_upper_bound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crossover_test() {
        let mut first = [0; 256];
        let mut second = [1; 256];
        crossover(&mut first, &mut second);
        assert_ne!(first[0], first[255]);
    }
}
