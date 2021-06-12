use std::cmp::Ordering;

use rand::distributions::{Uniform, WeightedIndex};
use rand::{Rng, thread_rng};

use crate::genetic::definitions::*;

impl Generation {
    pub fn select_by_tournament(&self) -> (Vec<usize>, Vec<Parents>) {
        let mut parents = Vec::with_capacity(CROSSOVER_SIZE);
        let mut kept = Vec::with_capacity(POPULATION_SIZE - CROSSOVER_SIZE);
        for _ in 0..CROSSOVER_SIZE {
            let stock = self.tournament();
            let scion = self.tournament();
            parents.push(Parents { stock, scion });
        }
        for _ in CROSSOVER_SIZE..POPULATION_SIZE {
            kept.push(self.tournament())
        }

        (kept, parents)
    }

    pub fn select_weighted_by_fitness(&self) -> (Vec<usize>, Vec<Parents>) {
        let mut best_score = self.population[self.best_index.unwrap()].result.unwrap().score;
        let worst_score = self.population[self.worst_index.unwrap()].result.unwrap().score;

        if best_score == worst_score {
            best_score += 0.1;
        }

        // calculate fitness values
        let mut fits = Vec::with_capacity(POPULATION_SIZE);
        for i in 0..POPULATION_SIZE {
            let score = self.population[i].result.unwrap().score;
            fits.push(if score.is_finite() {
                ((score - worst_score)/(best_score - worst_score) + 1.0).powf(SELECTION_PRESSURE)
            } else {
                0.0
            });
            1;
        }

        let mut rng = thread_rng();
        // let dist = WeightedIndex::new(&fits).unwrap(); // keeps panicking, unwrap on Err, but only rarely
        let dist = WeightedIndex::new(&fits).unwrap_or_else(|_| {
            dbg!(fits); // why DO the weights keep failing?
            panic!() // todo remove debug code
        });
        let chosen: Vec<usize> = (&mut rng)
            .sample_iter(dist)
            .take(POPULATION_SIZE + CROSSOVER_SIZE)
            .collect();
        // chosen: stocks + scions + kept
        let mut parents = Vec::with_capacity(CROSSOVER_SIZE);
        for i in 0..CROSSOVER_SIZE {
            parents.push(Parents { stock: chosen[i], scion: chosen[CROSSOVER_SIZE + i]});
        }

        (Vec::from(&chosen[2*CROSSOVER_SIZE..]), parents)
    }

    fn tournament(&self) -> usize {
        let mut rng = rand::thread_rng();

        let mut members: Vec<usize> = (&mut rng)
            .sample_iter(Uniform::from(0..POPULATION_SIZE))
            .take(TOURNAMENT_SIZE)
            .collect();
        members.sort_unstable_by(|&a, &b| -> Ordering {
            self.population[a].result.partial_cmp(&self.population[b].result).unwrap()
        });

        let mut idx = members.len() - 1;
        while idx > 0 {
            let choose = rng.gen_bool(TOURNAMENT_P);

            if choose {
                return members[idx];
            } else {
                idx -= 1;
            }
        }

        members[idx]
    }
}