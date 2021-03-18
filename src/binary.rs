use std::cmp::Ordering;
use std::convert::TryFrom;

use num_enum::TryFromPrimitive;

use crate::simulator::definitions::{Direction, Side};
use crate::vm::definitions::{BinaryOperation, Instruction, JumpCondition, Source, TurtleOperation, UnaryOperation};

#[derive(TryFromPrimitive)]
#[repr(u8)]
pub enum Instr {
    Pass,
    PrintValue,
    PrintRegister,
    SetValue,
    SetRegister,
    AddValue,
    AddRegister,
    SubtractValue,
    SubtractRegister,
    MultiplyValue,
    MultiplyRegister,
    DivideValue,
    DivideRegister,
    ModuloValue,
    ModuloRegister,
    AndValue,
    AndRegister,
    OrValue,
    OrRegister,
    Not,
    ShiftLeft,
    ShiftRight,
    RotateLeft,
    RotateRight,
    Increment,
    Decrement,
    CompareValue,
    CompareRegister,
    Jump,
    JumpZero,
    JumpNotZero,
    JumpLess,
    JumpNotLess,
    JumpGreater,
    JumpNotGreater,
    JumpEqual,
    JumpNotEqual,
    // turtle-specific operations:
    Forward,
    Back,
    Up,
    Down,
    TurnLeft,
    TurnRight,
    Dig,
    DigUp,
    DigDown,
    Place,
    PlaceUp,
    PlaceDown,
    Detect,
    DetectUp,
    DetectDown
}

fn extract_instruction<'a, I>(i: &mut I) -> Option<Instruction>
    where I: Iterator<Item = &'a u8>
{
    let instr_byte = i.next();
    let instr = Instr::try_from(*instr_byte?);

    if let Ok(instr) = instr {
        Some(match instr {
            Instr::Pass => Instruction::Pass,
            Instr::PrintValue => Instruction::Print(Source::Value(*i.next()? as i8)),
            Instr::PrintRegister => Instruction::Print(Source::Register(*i.next()?)),
            Instr::SetValue => Instruction::Binary(
                *i.next()?, Source::Value(*i.next()? as i8),
                BinaryOperation::Set
            ),
            Instr::SetRegister => Instruction::Binary(
                *i.next()?, Source::Register(*i.next()?),
                BinaryOperation::Set
            ),
            Instr::AddValue => Instruction::Binary(
                *i.next()?, Source::Value(*i.next()? as i8),
                BinaryOperation::Add
            ),
            Instr::AddRegister => Instruction::Binary(
                *i.next()?, Source::Register(*i.next()?),
                BinaryOperation::Add
            ),
            Instr::SubtractValue => Instruction::Binary(
                *i.next()?, Source::Value(*i.next()? as i8),
                BinaryOperation::Subtract
            ),
            Instr::SubtractRegister => Instruction::Binary(
                *i.next()?, Source::Register(*i.next()?),
                BinaryOperation::Subtract
            ),
            Instr::MultiplyValue => Instruction::Binary(
                *i.next()?, Source::Value(*i.next()? as i8),
                BinaryOperation::Multiply
            ),
            Instr::MultiplyRegister => Instruction::Binary(
                *i.next()?, Source::Register(*i.next()?),
                BinaryOperation::Multiply
            ),
            Instr::DivideValue => Instruction::Binary(
                *i.next()?, Source::Value(*i.next()? as i8),
                BinaryOperation::Divide
            ),
            Instr::DivideRegister => Instruction::Binary(
                *i.next()?, Source::Register(*i.next()?),
                BinaryOperation::Divide
            ),
            Instr::ModuloValue => Instruction::Binary(
                *i.next()?, Source::Value(*i.next()? as i8),
                BinaryOperation::Modulo
            ),
            Instr::ModuloRegister => Instruction::Binary(
                *i.next()?, Source::Register(*i.next()?),
                BinaryOperation::Modulo
            ),
            Instr::AndValue => Instruction::Binary(
                *i.next()?, Source::Value(*i.next()? as i8),
                BinaryOperation::And
            ),
            Instr::AndRegister => Instruction::Binary(
                *i.next()?, Source::Register(*i.next()?),
                BinaryOperation::And
            ),
            Instr::OrValue => Instruction::Binary(
                *i.next()?, Source::Value(*i.next()? as i8),
                BinaryOperation::Or
            ),
            Instr::OrRegister => Instruction::Binary(
                *i.next()?, Source::Register(*i.next()?),
                BinaryOperation::Or
            ),
            Instr::Not => Instruction::Unary(*i.next()?, UnaryOperation::Not),
            Instr::ShiftLeft => Instruction::Unary(*i.next()?, UnaryOperation::ShiftLeft),
            Instr::ShiftRight => Instruction::Unary(*i.next()?, UnaryOperation::ShiftRight),
            Instr::RotateLeft => Instruction::Unary(*i.next()?, UnaryOperation::RotateLeft),
            Instr::RotateRight => Instruction::Unary(*i.next()?, UnaryOperation::RotateRight),
            Instr::Increment => Instruction::Unary(*i.next()?, UnaryOperation::Increment),
            Instr::Decrement => Instruction::Unary(*i.next()?, UnaryOperation::Decrement),
            Instr::CompareValue =>
                Instruction::Compare(*i.next()?, Source::Value(*i.next()? as i8)),
            Instr::CompareRegister =>
                Instruction::Compare(*i.next()?, Source::Register(*i.next()?)),
            Instr::Jump => Instruction::Jump(*i.next()?, JumpCondition::None),
            Instr::JumpZero => Instruction::Jump(*i.next()?, JumpCondition::Zero(*i.next()?)),
            Instr::JumpNotZero => Instruction::Jump(*i.next()?, JumpCondition::NotZero(*i.next()?)),
            Instr::JumpLess =>
                Instruction::Jump(*i.next()?, JumpCondition::Compare(Ordering::Less)),
            Instr::JumpNotLess =>
                Instruction::Jump(*i.next()?, JumpCondition::NotCompare(Ordering::Less)),
            Instr::JumpGreater =>
                Instruction::Jump(*i.next()?, JumpCondition::Compare(Ordering::Greater)),
            Instr::JumpNotGreater =>
                Instruction::Jump(*i.next()?, JumpCondition::NotCompare(Ordering::Greater)),
            Instr::JumpEqual =>
                Instruction::Jump(*i.next()?, JumpCondition::Compare(Ordering::Equal)),
            Instr::JumpNotEqual =>
                Instruction::Jump(*i.next()?, JumpCondition::NotCompare(Ordering::Equal)),
            Instr::Forward => Instruction::Turtle(TurtleOperation::Move(Direction::Forward)),
            Instr::Back => Instruction::Turtle(TurtleOperation::Move(Direction::Back)),
            Instr::Up => Instruction::Turtle(TurtleOperation::Move(Direction::Up)),
            Instr::Down => Instruction::Turtle(TurtleOperation::Move(Direction::Down)),
            Instr::TurnLeft => Instruction::Turtle(TurtleOperation::Turn(Side::Left)),
            Instr::TurnRight => Instruction::Turtle(TurtleOperation::Turn(Side::Right)),
            Instr::Dig => Instruction::Turtle(TurtleOperation::Dig(Direction::Forward)),
            Instr::DigUp => Instruction::Turtle(TurtleOperation::Dig(Direction::Up)),
            Instr::DigDown => Instruction::Turtle(TurtleOperation::Dig(Direction::Down)),
            Instr::Place => Instruction::Turtle(TurtleOperation::Place(Direction::Forward)),
            Instr::PlaceUp => Instruction::Turtle(TurtleOperation::Place(Direction::Up)),
            Instr::PlaceDown => Instruction::Turtle(TurtleOperation::Place(Direction::Down)),
            Instr::Detect => Instruction::Turtle(TurtleOperation::Detect(Direction::Forward)),
            Instr::DetectUp => Instruction::Turtle(TurtleOperation::Detect(Direction::Up)),
            Instr::DetectDown => Instruction::Turtle(TurtleOperation::Detect(Direction::Down)),
        })
    } else {
        None
    }
}

pub fn parse_bytes(bytes: &[u8]) -> Vec<Instruction> {
    let mut iter = bytes.iter();
    let mut instructions = vec!();
    loop {
        let result = extract_instruction(&mut iter);
        if let Some(instruction) = result {
            instructions.push(instruction);
        } else {
            break;
        }
    }
    instructions
}