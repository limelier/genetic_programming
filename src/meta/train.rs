use rayon::prelude::*;
use crate::simulator::definitions::BlockSpace;
use crate::genetic::train;
use crate::genetic::definitions::Individual;

pub fn train_many(target: &BlockSpace, number: usize) -> Vec<Individual> {
    let mut results = vec!();

    results.par_extend(
        (1..=number)
            .into_par_iter()
            .map(|i| {
                let result = train(target);
                println!("Run {} done", i);
                result
            })
    );

    results
}