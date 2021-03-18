use crate::trees::definitions::{Node, STACK_START};
use crate::vm::definitions::{Instruction, BinaryOperation, Source};

pub fn translate_tree(tree: Node) -> Vec<Instruction> {
    translate_subtree(tree, STACK_START)
}

fn translate_subtree(tree: Node, stack_ptr: u8) -> Vec<Instruction> {
    match tree {
        Node::Null => {
            // do nothing
            vec!()
        }
        Node::Val(src) => {
            // r[stack_ptr] = src
            vec!(
                Instruction::Binary(stack_ptr, src, BinaryOperation::Set)
            )
        }
        Node::Unary(op, child) => {
            // do the subtree, which stores result in r[stack_tr]
            let mut instr = translate_subtree(*child, stack_ptr);
            // do op on r[stack_ptr]
            instr.push(Instruction::Unary(stack_ptr, op));

            instr
        }
        Node::Binary(op, left, right) => {
            // do left subtree, which stores result in r[stack_ptr]
            let mut instr = translate_subtree(*left, stack_ptr);
            // do right subtree, which stores result in r[stack_ptr + 1]
            instr.append(&mut translate_subtree(*right, stack_ptr + 1));
            // do op on r[stack_ptr] with r[stack_ptr + 1]
            instr.push(Instruction::Binary(stack_ptr, Source::Register(stack_ptr + 1), op));

            instr
        }
        Node::Then(left, right) => {
            // do left subtree, which stores result in r[stack_ptr]
            let mut instr = translate_subtree(*left, stack_ptr);
            // do right subtree, which stores result in r[stack_ptr]
            instr.append(&mut translate_subtree(*right, stack_ptr));

            instr
        }
        Node::Print(child) => {
            // do child, which stores result in r[stack_ptr]
            let mut instr = translate_subtree(*child, stack_ptr);
            // print r[stack_ptr]
            instr.push(Instruction::Print(Source::Register(stack_ptr)));

            instr
        }
        Node::Store(reg, child) => { // stores result of child in reg
            // execute child code, storing result in r[stack_ptr]
            let mut instr = translate_subtree(*child, stack_ptr);
            // r[reg] = r[stack_ptr]
            instr.push(Instruction::Binary(reg, Source::Register(stack_ptr), BinaryOperation::Set));

            instr
        }
    }
}