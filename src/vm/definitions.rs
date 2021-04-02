use std::cmp::Ordering;

use crate::simulator::definitions::{Direction, Side};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

pub type Reg = u8;
pub type Ins = usize;
pub type Label = u8;
pub type Val = i8;

pub const RESULT_REGISTER: u8 = 200;
pub const COMPARE_REGISTER: u8 = 201;

#[derive(Copy, Clone, Debug)]
pub enum Source {
    Register(Reg),
    Value(Val),
}

#[derive(Copy, Clone, Debug)]
pub enum UnaryOperation {
    Not,
    ShiftLeft,
    ShiftRight,
    RotateLeft,
    RotateRight,
    Increment,
    Decrement
}

impl Distribution<UnaryOperation> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UnaryOperation {
        match rng.gen_range(0..=6) {
            0 => UnaryOperation::Not,
            1 => UnaryOperation::ShiftLeft,
            2 => UnaryOperation::ShiftRight,
            3 => UnaryOperation::RotateLeft,
            4 => UnaryOperation::RotateRight,
            5 => UnaryOperation::Increment,
            _ => UnaryOperation::Decrement,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BinaryOperation {
    Set,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    And,
    Or,
}

impl Distribution<BinaryOperation> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BinaryOperation {
        match rng.gen_range(0..=7) {
            0 => BinaryOperation::Set,
            1 => BinaryOperation::Add,
            2 => BinaryOperation::Subtract,
            3 => BinaryOperation::Multiply,
            4 => BinaryOperation::Divide,
            5 => BinaryOperation::Modulo,
            6 => BinaryOperation::And,
            _ => BinaryOperation::Or,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum JumpCondition {
    None,
    Zero(Reg),
    NotZero(Reg),
    Compare(Ordering),
    NotCompare(Ordering)
}

#[derive(Copy, Clone, Debug)]
pub enum TurtleOperation {
    Move(Direction),
    Turn(Side),
    Place(Direction),
    Dig(Direction),
    Detect(Direction),
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    /// Do operation on register alone
    Unary(Reg, UnaryOperation),
    /// Do operation on register using source
    Binary(Reg, Source, BinaryOperation),
    /// Jump to the given instruction if it exists, and if the condition is met
    Jump (Label, JumpCondition),
    /// Compare register with source, store value in comparison register
    Compare (Reg, Source),
    /// Print the source value
    Print (Source),
    /// Execute turtle operations in the simulator
    Turtle (TurtleOperation),
    /// Label for jumping
    Label (Label),
    /// Do nothing
    Pass
}