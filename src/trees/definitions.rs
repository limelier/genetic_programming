use crate::vm::definitions::{Source, UnaryOperation, BinaryOperation, Reg};

pub enum Node {
    Null,
    Val(Source),
    Unary(UnaryOperation, Box<Node>),
    Binary(BinaryOperation, Box<Node>, Box<Node>),
    Then(Box<Node>, Box<Node>),
    Print(Box<Node>),
    Store(Reg, Box<Node>)
    // If(Source, Node, Node),
    // Loop(Source, Node, Node),
}

// usable registers up until STACK_START, then stack after
pub const STACK_START: u8 = 100;