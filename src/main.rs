use genetic_programming::vm::definitions::{TurtleOperation, UnaryOperation};
use genetic_programming::vm::program::Program;
use genetic_programming::trees::definitions::Node;
use genetic_programming::trees::translate::translate_tree;
use genetic_programming::simulator::definitions::Direction;

fn main() {
    /*

     */
    let tree = Node::Then(
        Box::from(Node::While(
            Box::from(Node::Turtle(
                TurtleOperation::Move(Direction::Forward),
            )),
            Box::from(Node::Unary(
                UnaryOperation::Increment,
                Box::from(Node::Null))
            ),
        )),
        Box::from(Node::Print(
            Box::from(Node::Null)
        )),
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