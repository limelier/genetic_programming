use genetic_programming::meta::train::train_many;
use genetic_programming::simulator::definitions::BlockSpace;

fn main() {
    let mut target = BlockSpace::default();
    for i in 0..8 {
        target[i][1][0] = 1;
    }
    let target = target;  // remove mutability
    let individuals = train_many(&target, 30);

    println!("dice_index depth");
    for individual in individuals {
        println!("{} {}",
                 individual.result.unwrap().dice_index,
                 individual.tree.get_max_depth()
        );
    }
}