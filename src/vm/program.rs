use crate::vm::structures::*;

pub struct Program {
    instructions: [Instruction; 256],
    registers: [Val; 256],
    instruction_pointer: Ins,
    halted: bool,
}

impl Program {
    fn new() -> Self {
        Program {
            instructions: [Instruction::Pass; 256],
            registers: [0; 256],
            instruction_pointer: 0,
            halted: false,
        }
    }
    pub fn from_instructions(instructions: &Vec<Instruction>) -> Self {
        if instructions.len() > 256 {
            panic!("Instruction vector too big! Only 256 allowed.")
        }

        let mut program = Program::new();

        for (i, instruction) in instructions.iter().enumerate() {
            program.instructions[i] = instruction.clone();
        }

        program
    }
    pub fn execute(&mut self) {
        while !self.halted {
            self.step();
        }
    }
    fn step(&mut self) {
        // todo
        self.halted = true;
    }
}