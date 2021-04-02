use genetic_programming::vm::program::Program;
use genetic_programming::genetic::generation::{generate, Method};
use genetic_programming::trees::translate::translate_tree;

fn main() {
    let tree = generate(Method::Grow, 6);
    let instr = translate_tree(tree);
    for ins in &instr {
        println!("{:?}", ins);
    }
    println!("========== v");
    let mut program = Program::from_instructions(&instr);
    let result = program.execute(Some(2000));
    println!("========== ^");
    println!("{:?}", result);
}