use crate::vm::structures::*;
use std::intrinsics::add_with_overflow;
use std::cmp::Ordering;

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
        let mut jumped = false;

        match self.instructions[self.instruction_pointer] {
            Instruction::Unary(reg, op) => {
                let old = self.registers[reg];
                self.registers[reg] = match op {
                    UnaryOperation::Not => !old,
                    UnaryOperation::ShiftLeft => {
                        let lost_bit = old & (1 << 7);
                        self.registers[RESULT_REGISTER] = lost_bit;
                        old << 1
                    },
                    UnaryOperation::ShiftRight => {
                        let lost_bit = old & 1;
                        self.registers[RESULT_REGISTER] = lost_bit;
                        old >> 1
                    },
                    UnaryOperation::RotateLeft => i8::rotate_left(old, 1),
                    UnaryOperation::RotateRight => i8::rotate_right(old, 1),
                    UnaryOperation::Increment => i8::overflowing_add(old, 1),
                    UnaryOperation::Decrement => i8::overflowing_sub(old, 1),
                }
            },
            Instruction::Binary(reg, src, op) => {
                let other = self.get_source(src);
                let old = self.registers[reg];
                self.registers[reg] = match op {
                    BinaryOperation::Set => other,
                    BinaryOperation::Add => old + other,
                    BinaryOperation::Subtract => old - other,
                    BinaryOperation::Multiply => old * other,
                    BinaryOperation::Divide => old / other,
                    BinaryOperation::Modulo => old % other,
                    BinaryOperation::And => old & other,
                    BinaryOperation::Or => old | other,
                }
            },
            Instruction::Compare(reg, src) => {
                let other = self.get_source(src);
                let this = self.registers[reg];
                self.registers[COMPARE_REGISTER] = match i8::cmp(this, &other) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                }
            },
            Instruction::Jump(ins, condition) => {
                // todo
            },
            Instruction::Pass => { /* do nothing */ }
        }
    }
    fn get_source(&self, src: Source) -> i8 {
        match src {
            Source::Register(reg2) => self.registers[reg2],
            Source::Value(val) => val
        }
    }
}