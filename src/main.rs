use genetic_programming::genetic::definitions::Generation;
use genetic_programming::trees::translate::translate_tree;
use genetic_programming::vm::program::Program;

fn main() {
    let generation = Generation::generate();
    let individual = &generation.population[0];
    // let instr = translate_tree(&individual.tree);
    // for ins in &instr {
    //     println!("{:?}", ins);
    // }
    // println!("========== v");
    // let mut program = Program::from_instructions(&instr);
    // let result = program.execute(Some(2000));
    // println!("========== ^");
    // println!("{:?}", result);
}