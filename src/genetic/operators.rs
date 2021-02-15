use crate::genetic::definitions::{Chromosome, MUTATION_CHANCE, CHROMOSOME_SIZE, Population, POPULATION_SIZE, MIN_SEGMENT_LENGTH, RANDOM_COUNT, Individual};
use rand::Rng;
use crate::genetic::evaluation::evaluate_population;
use crate::vm::structures::BlockSpace;

pub(crate) fn mutate(individual: &mut Chromosome) {
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
        mutate(&mut individual.chromosome);
    }
}

pub(crate) fn crossover(first: &mut Chromosome, second: &mut Chromosome) {
    let mut rng = rand::thread_rng();
    let cut_point = rng.gen_range(1..CHROMOSOME_SIZE - 1);
    let swap_before: bool = rng.gen();

    let swap_range = if swap_before {
        0..cut_point
    } else {
        cut_point..CHROMOSOME_SIZE
    };

    for i in swap_range {
        let aux = first[i];
        first[i] = second[i];
        second[i] = aux;
    }
}

pub(crate) fn crossover_population(population: &mut Population) {
    for i in 0..POPULATION_SIZE / 2 {
        let mut first = population[i].chromosome;
        let mut second = population[i + POPULATION_SIZE / 2].chromosome;
        crossover(&mut first, &mut second);
        population[i].chromosome = first;
        population[i + POPULATION_SIZE / 2].chromosome = second;
    }
}

pub(crate) fn select(population: &mut Population, target: &BlockSpace) -> (Population, Individual) {
    // evaluate and sort by score
    evaluate_population(population, target);

    // construct the "wheel of fortune"
    let mut segment_length = [0.0; POPULATION_SIZE];
    let score_lower_bound = {
        let lowest_score = population[0].score.unwrap();

        if lowest_score.is_infinite() {
            0.0
        } else {
            lowest_score
        }
    };

    for (i, individual) in population.iter().enumerate() {
        let score = individual.score.unwrap();
        segment_length[i] = if score.is_finite() {
            MIN_SEGMENT_LENGTH + (score - score_lower_bound)
        } else {
            0.0
        };
    }
    let wheel_size: f64 = segment_length.iter().sum();

    // spin the wheel to build the new population
    let mut new_population = [Individual::new(); 256];
    let mut rng = rand::thread_rng();
    for i in 0..POPULATION_SIZE - RANDOM_COUNT {
        let mut needle = rng.gen_range(0.0..wheel_size);
        let mut chosen = 0;

        while needle > segment_length[chosen] {
            needle -= segment_length[chosen];
            chosen += 1;
        }

        new_population[i] = Individual {
            chromosome: population[chosen].chromosome,
            score: None
        };
    }

    for i in POPULATION_SIZE - RANDOM_COUNT..POPULATION_SIZE {
        new_population[i] = Individual::random();
    }

    (new_population, population[POPULATION_SIZE - 1].clone())
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
