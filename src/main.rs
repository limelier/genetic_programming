use std::cmp::Ordering;
use genetic_programming::vm::program::Program;
use genetic_programming::vm::structures::{
    Instruction, BinaryOperation, JumpCondition, Source
};

fn main() {
    /*
        r0 = 1;
        while (r0 < 100) {
            print(r0);
            r0 += 10;
        }
     */
    let mut program = Program::from_instructions(&vec!{
        Instruction::Binary(0, Source::Value(1), BinaryOperation::Set), // 0
        Instruction::Compare(0, Source::Value(100)), // 1
        Instruction::Jump(6, JumpCondition::NotCompare(Ordering::Less)), // 2
        Instruction::Print(Source::Register(0)), // 3
        Instruction::Binary(0, Source::Value(10), BinaryOperation::Add), // 4
        Instruction::Jump(1, JumpCondition::None), // 5
        Instruction::Pass, // 6
    });

    program.execute();
}