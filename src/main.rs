use genetic_programming::vm::definitions::{BinaryOperation, Instruction, JumpCondition};
use genetic_programming::vm::definitions::Source;
use genetic_programming::vm::program::Program;

fn main() {
    let instr = vec!(
        Instruction::Binary(0, Source::Value(1), BinaryOperation::Set),
        Instruction::Jump(0, JumpCondition::Zero(1)),
        Instruction::Print(Source::Register(1)),
        Instruction::Label(0),
        Instruction::Print(Source::Register(0)),
    );
    println!("========== v");
    let mut program = Program::from_instructions(&instr);
    let result = program.execute(None);
    println!("========== ^");
    println!("{:?}", result);
}