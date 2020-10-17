use std::cmp::Ordering;

pub type Reg = u8;
pub type Ins = u8;
pub type Val = i8;

pub const RESULT_REGISTER: u8 = 200;
pub const COMPARE_REGISTER: u8 = 201;

#[derive(Copy, Clone)]
pub enum Source {
    Register(Reg),
    Value(Val),
}

#[derive(Copy, Clone)]
pub enum UnaryOperation {
    Not,
    ShiftLeft,
    ShiftRight,
    RotateLeft,
    RotateRight,
    Increment,
    Decrement
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub enum JumpCondition {
    None,
    Zero(Reg),
    NotZero(Reg),
    Compare(Ordering),
    NotCompare(Ordering)
}

#[derive(Copy, Clone)]
pub enum Instruction {
    /// Do operation on register alone
    Unary(Reg, UnaryOperation),
    /// Do operation on register using source
    Binary(Reg, Source, BinaryOperation),
    /// Jump to the given instruction if it exists, and if the condition is met
    Jump (Ins, JumpCondition),
    /// Compare register with source, store value in comparison register
    Compare (Reg, Source),
    /// Print the source value
    Print (Source),
    /// Do nothing
    Pass
}

