use genetic_programming::vm::definitions::Source;
use genetic_programming::vm::program::Program;
use genetic_programming::trees::definitions::Node;
use genetic_programming::trees::translate::translate_tree;

fn main() {
    // if 1 { 10 } else { 20 }
    let tree = Node::Print(
        Box::from(Node::If(
            Box::from(Node::Val(Source::Value(1))),
            Box::from(Node::Val(Source::Value(10))),
            Box::from(Node::Val(Source::Value(20))),
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