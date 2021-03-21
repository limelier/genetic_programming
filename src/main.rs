use genetic_programming::vm::definitions::{TurtleOperation, UnaryOperation, Source};
use genetic_programming::vm::program::Program;
use genetic_programming::trees::definitions::Node;
use genetic_programming::trees::translate::translate_tree;
use genetic_programming::simulator::definitions::Direction;

fn main() {
    /*

     */
    let tree = Node::Print(
        Box::from(Node::Compare(
            Box::from(Node::Val(Source::Value(10))),
            Box::from(Node::Val(Source::Value(20)))
        ))
    );
    let instr = translate_tree(tree);
    for ins in &instr {
        println!("{:?}", ins);
    }
    println!("========== v");
    let mut program = Program::from_instructions(&instr);
    let result = program.execute(None);
    println!("========== ^");
    println!("{:?}", result);
}