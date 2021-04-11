use std::cmp::Ordering;

use rand::distributions::Uniform;
use rand::Rng;

use crate::genetic::definitions::*;

impl Generation {
    /// Select POPULATION_SIZE pairs of parents for crossover
    ///
    /// Each parent is selected through a tournament
    pub fn select(&self) -> (Vec<usize>, Vec<Parents>) {
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

    /// Pick a handful of individuals from the population, and pick one based on its result
    ///
    /// More specifically, pick out n = TOURNAMENT_SIZE members of the population, then:
    /// - choose the best with probability p = TOURNAMENT_P
    /// - choose the 2nd best with probability (1-p) * p
    /// - choose the 3rd best with probability (1-p)^2 * p
    /// - ...
    /// - choose the worst with probability (1-p)^n
    ///
    /// Return the index of the chosen individual in the population
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