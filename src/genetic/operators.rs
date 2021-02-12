use crate::genetic::definitions::{Individual, MUTATION_CHANCE, INDIVIDUAL_SIZE};
use rand::Rng;

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
