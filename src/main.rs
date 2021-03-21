use genetic_programming::vm::definitions::{Source, BinaryOperation};
use genetic_programming::vm::program::Program;
use genetic_programming::trees::definitions::Node;
use genetic_programming::trees::translate::translate_tree;

fn main() {
    /*

     */
    let tree = Node::Then(
        Box::from(Node::Then(
            Box::from(Node::Store(
                0,
                Box::from(Node::Val(Source::Value(5)))
            )),
            Box::from(Node::Store(
                1,
                Box::from(Node::Val(Source::Value(1)))
            ))
        )),
        Box::from(Node::Print(
            Box::from(Node::While(
                Box::from(Node::Val(Source::Register(0))),
                Box::from(Node::Then(
                    Box::from(Node::Print(
                        Box::from(Node::Val(Source::Register(0)))
                    )),
                    Box::from(Node::Then(
                        Box::from(Node::Then(
                            Box::from(Node::Store(
                                1,
                                Box::from(Node::Binary(
                                    BinaryOperation::Multiply,
                                    Box::new(Node::Val(Source::Register(1))),
                                    Box::new(Node::Val(Source::Value(2)))
                                )),
                            )),
                            Box::from(Node::Store(
                                0,
                                Box::from(Node::Binary(
                                    BinaryOperation::Subtract,
                                    Box::new(Node::Val(Source::Register(0))),
                                    Box::new(Node::Val(Source::Value(1)))
                                ))
                            ))
                        )),
                        Box::from(Node::Val(Source::Register(1)))
                    ))
                ))
            ))
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