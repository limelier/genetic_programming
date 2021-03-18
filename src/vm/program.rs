use std::cmp::Ordering;
use std::time::Instant;

use crate::simulator::definitions::BlockSpace;
use crate::vm::definitions::*;
use crate::simulator::simulator::Simulator;
use std::collections::HashMap;

pub struct Program {
    instructions: Vec<Instruction>,
    labels: HashMap<Label, Ins>,
    registers: [Val; 256],
    instruction_pointer: Ins,
    halted: bool,
    simulator: Simulator,
}

impl Program {
    fn new() -> Self {
        Program {
            instructions: vec!(Instruction::Pass),
            labels: HashMap::new(),
            registers: [0; 256],
            instruction_pointer: 0,
            halted: false,
            simulator: Simulator::new(),
        }
    }

    pub fn from_instructions(instructions: &Vec<Instruction>) -> Self {
        let mut program = Program::new();

        program.instructions = instructions.clone();
        program.process_labels();

        program
    }

    fn process_labels(&mut self) {
        for (idx, instr) in self.instructions.iter().enumerate() {
            if let Instruction::Label(label) = instr {
                self.labels.insert(*label, idx);
            }
        }
    }

    pub fn execute(&mut self, max_millis: Option<u128>) -> Result<(), &'static str> {
        let start = Instant::now();
        while !self.halted {
            self.step()?;
            if let Some(max_millis) = max_millis {
                if start.elapsed().as_millis() > max_millis {
                    return Err("Timeout reached!")
                }
            }
        }

        Ok(())
    }

    fn get_reg(&self, reg: Reg) -> Val {
        self.registers[reg as usize]
    }

    fn set_reg(&mut self, reg: Reg, val: Val) {
        self.registers[reg as usize] = val;
    }

    fn step(&mut self) -> Result<(), &'static str> {
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
                    BinaryOperation::Divide => {
                        if other == 0 {
                            1
                        } else {
                            i8::overflowing_div(old, other).0
                        }
                    },
                    BinaryOperation::Modulo => {
                        if other == 0 {
                            1
                        } else {
                            old % other
                        }
                    },
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
            Instruction::Jump(label, condition) => {
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
                        let destination = self.labels.get(&label);
                        if destination == None {
                            return Err("Attempted to jump to missing label");
                        }
                        let destination = destination.unwrap();
                        self.instruction_pointer = *destination;
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
                    TurtleOperation::Dig(dir) => {
                        let res = self.simulator.try_dig(dir) as i8;
                        self.set_reg(RESULT_REGISTER, res);
                    },
                    TurtleOperation::Detect(dir) => {
                        let res = self.simulator.detect(dir) as i8;
                        self.set_reg(RESULT_REGISTER, res);
                    },
                }
            }
            Instruction::Print(src) => {
                println!("{}", self.get_source(src))
            },
            Instruction::Pass => { /* do nothing */ },
            Instruction::Label(_) => { /* do nothing */ }
        }
        if !jumped {
            if self.instruction_pointer >= self.instructions.len() - 1 {
                self.halted = true;
            } else {
                self.instruction_pointer += 1;
            }
        }

        Ok(())
    }

    fn get_source(&self, src: Source) -> i8 {
        match src {
            Source::Register(reg) => self.get_reg(reg),
            Source::Value(val) => val
        }
    }

    pub(crate) fn get_simulator_state(&self) -> BlockSpace {
        self.simulator.get_state_copy()
    }
}

fn ord_to_num(ord: Ordering) -> Val {
    match ord {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}