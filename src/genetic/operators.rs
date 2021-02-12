use crate::genetic::definitions::{Individual, MUTATION_CHANCE};
use rand::Rng;

pub(crate) fn mutate(individual: &Individual) -> Individual {
    let mut rng = rand::thread_rng();
    let mut modified = [0; 256];

    for (i, byte) in individual.iter().enumerate() {
        let roll: f64 = rng.gen();

        modified[i] = if roll <= MUTATION_CHANCE {
            rng.gen_range(0..=255)
        } else {
            *byte
        }
    }

    modified
}
