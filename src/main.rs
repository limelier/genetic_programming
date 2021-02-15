use genetic_programming::vm::structures::BlockSpace;
use genetic_programming::genetic::evaluation::evaluate_individual;
use genetic_programming::genetic::train::train;

fn main() {
    let mut target = BlockSpace::default();
    for i in 0..8 {
        target[i][1][0] = 1;
    }
    let target = target;  // remove mutability

    let individual = train(&target);

    println!("Final individual: \n{:?}", individual);
    println!("Evaluation score: {}", evaluate_individual(&individual, &target));
}