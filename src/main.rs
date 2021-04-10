use genetic_programming::simulator::definitions::BlockSpace;
use genetic_programming::genetic::train;
use genetic_programming::vm::program::Program;
use genetic_programming::trees::translate::translate_tree;

fn main() {
    let mut target = BlockSpace::default();
    for i in 0..8 {
        target[i][1][0] = 1;
    }
    let target = target;  // remove mutability
    let individual = train(&target);
    let mut program = Program::from_instructions(&translate_tree(&individual.tree));
    program.execute(None);
}