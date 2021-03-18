use genetic_programming::trees::definitions::Node;
use genetic_programming::vm::definitions::{BinaryOperation};
use genetic_programming::vm::definitions::Source;
use genetic_programming::trees::translate::translate_tree;
use genetic_programming::vm::program::Program;

fn main() {
    // r0 = 2+3; print r0
    // actually:
    // r100 = 2; r101 = 3; r100 += r101; r0 = r100; r100 = r0; print r100
    let tree = Node::Then(
        Box::from(Node::Store(
            0,
            Box::from(Node::Binary(
                BinaryOperation::Add,
                Box::from(Node::Val(Source::Value(2))),
                Box::from(Node::Val(Source::Value(3))),
            ))
        )),
        Box::from(Node::Print(
            Box::from(Node::Val(Source::Register(0))),
        ))
    );

    let instr = translate_tree(tree);
    for ins in &instr {
        println!("{:?}", ins);
    }
    println!("========== v");
    let mut program = Program::from_instructions(&instr);
    let result = program.execute();
    println!("========== ^");
    println!("{:?}", result);
}