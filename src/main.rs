use genetic_programming::simulator::definitions::BlockSpace;
use genetic_programming::meta::train::train_many;

fn main() {
    let mut target = BlockSpace::default();
    for i in 0..8 {
        target[i][1][0] = 1;
    }
    let target = target;  // remove mutability
    let individuals = train_many(&target, 30);
    dbg!(individuals);
}