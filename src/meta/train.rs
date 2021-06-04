use rayon::prelude::*;
use indicatif::ProgressBar;
use crate::simulator::definitions::BlockSpace;
use crate::genetic::train;
use crate::genetic::definitions::Individual;

pub fn train_many(target: &BlockSpace, number: usize) -> Vec<Individual> {
    let mut results = vec!();

    println!("Executing {} runs...", number);
    let pb = ProgressBar::new(number as u64);
    pb.tick();
    results.par_extend(
        (1..=number)
            .into_par_iter()
            .map(|_| {
                let result = train(target);
                pb.inc(1);
                result
            })
    );
    pb.finish_with_message("Runs done!");

    results
}

pub fn train_many_silent(target: &BlockSpace, number: usize) -> Vec<Individual> {
    let mut results = vec!();

    results.par_extend(
        (1..=number)
            .into_par_iter()
            .map(|_| {
                let result = train(target);
                result
            })
    );

    results
}