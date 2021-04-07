use genetic_programming::genetic::definitions::Generation;
use genetic_programming::simulator::definitions::BlockSpace;

fn main() {
    let mut target = BlockSpace::default();
    for i in 0..8 {
        target[i][1][0] = 1;
    }
    let target = target;  // remove mutability

    let mut generation = Generation::generate();
    generation.evaluate(&target);
    let parents = generation.select();

    let new_generation = generation.crossover(&parents);

    dbg!(&new_generation.population[0].tree);
}