use crate::vm::structures::*;
use crate::vm::simulator::Simulator;
use std::cmp::Ordering;

pub struct Program {
    instructions: [Instruction; 256],
    registers: [Val; 256],
    instruction_pointer: Ins,
    halted: bool,
    simulator: Simulator,
}

impl Program {
    fn new() -> Self {
        Program {
            instructions: [Instruction::Pass; 256],
            registers: [0; 256],
            instruction_pointer: 0,
            halted: false,
            simulator: Simulator::new(),
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
    fn get_reg(&self, reg: Reg) -> Val {
        self.registers[reg as usize]
    }
    fn set_reg(&mut self, reg: Reg, val: Val) {
        self.registers[reg as usize] = val;
    }
    fn step(&mut self) {
        let mut jumped = false;

        match self.instructions[self.instruction_pointer as usize] {
            Instruction::Unary(reg, op) => {
                let old = self.get_reg(reg);
                let new =match op {
                    UnaryOperation::Not => !old,
                    UnaryOperation::ShiftLeft => {
                        let lost_bit = old & (1 << 7);
                        self.set_reg(RESULT_REGISTER, lost_bit);
                        old << 1
                    },
                    UnaryOperation::ShiftRight => {
                        let lost_bit = old & 1;
                        self.set_reg(RESULT_REGISTER, lost_bit);
                        old >> 1
                    },
                    UnaryOperation::RotateLeft => i8::rotate_left(old, 1),
                    UnaryOperation::RotateRight => i8::rotate_right(old, 1),
                    UnaryOperation::Increment => i8::overflowing_add(old, 1).0,
                    UnaryOperation::Decrement => i8::overflowing_sub(old, 1).0,
                };
                self.set_reg(reg, new);
            },
            Instruction::Binary(reg, src, op) => {
                let other = self.get_source(src);
                let old = self.get_reg(reg);
                let new = match op {
                    BinaryOperation::Set => other,
                    BinaryOperation::Add => i8::overflowing_add(old, other).0,
                    BinaryOperation::Subtract => i8::overflowing_sub(old, other).0,
                    BinaryOperation::Multiply => i8::overflowing_mul(old, other).0,
                    BinaryOperation::Divide => i8::overflowing_div(old, other).0,
                    BinaryOperation::Modulo => old % other,
                    BinaryOperation::And => old & other,
                    BinaryOperation::Or => old | other,
                };
                self.set_reg(reg, new);
            },
            Instruction::Compare(reg, src) => {
                let other = self.get_source(src);
                let this = self.get_reg(reg);
                self.set_reg(COMPARE_REGISTER, match i8::cmp(&this, &other) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                });
            },
            Instruction::Jump(ins, condition) => {
                    let do_jump = match condition {
                        JumpCondition::None => true,
                        JumpCondition::Zero(reg) => self.get_reg(reg) == 0,
                        JumpCondition::NotZero(reg) => self.get_reg(reg) != 0,
                        JumpCondition::Compare(ord) =>
                            ord_to_num(ord) == self.get_reg(COMPARE_REGISTER),
                        JumpCondition::NotCompare(ord) =>
                            ord_to_num(ord) != self.get_reg(COMPARE_REGISTER),
                    };
                    if do_jump {
                        self.instruction_pointer = ins;
                        jumped = true;
                    }
            },
            Instruction::Turtle(op) => {
                match op {
                    TurtleOperation::Move(dir) => {
                        let res = self.simulator.try_move(dir) as i8;
                        self.set_reg(RESULT_REGISTER, res);
                    },
                    TurtleOperation::Turn(side) => self.simulator.turn(side),
                    TurtleOperation::Place(dir) => {
                        let res = self.simulator.try_place(dir) as i8;
                        self.set_reg(RESULT_REGISTER, res);
                    },
                    TurtleOperation::Break(dir) => {
                        let res = self.simulator.try_break(dir) as i8;
                        self.set_reg(RESULT_REGISTER, res);
                    },
                    TurtleOperation::Detect(dir) => {
                        let res = self.simulator.detect(dir) as i8;
                        self.set_reg(RESULT_REGISTER, res);
                    },
                }
            }
            Instruction::Print(src) => println!("{}", self.get_source(src)),
            Instruction::Pass => { /* do nothing */ },
        }
        if !jumped {
            if self.instruction_pointer == 0xFF {
                self.halted = true;
            } else {
                self.instruction_pointer += 1;
            }
        }
    }
    fn get_source(&self, src: Source) -> i8 {
        match src {
            Source::Register(reg) => self.get_reg(reg),
            Source::Value(val) => val
        }
    }
}

fn ord_to_num(ord: Ordering) -> Val {
    match ord {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}