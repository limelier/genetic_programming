use crate::trees::definitions::{Node, STACK_START};
use crate::vm::definitions::{Instruction, BinaryOperation, Source, JumpCondition, RESULT_REGISTER};

pub fn translate_tree(tree: Node) -> Vec<Instruction> {
    let mut next_label = 0u8;
    translate_subtree(tree, STACK_START, &mut next_label)
}

fn translate_subtree(tree: Node, stack_ptr: u8, next_label: &mut u8) -> Vec<Instruction> {
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
            let mut instr = translate_subtree(*child, stack_ptr, next_label);
            // do op on r[stack_ptr]
            instr.push(Instruction::Unary(stack_ptr, op));

            instr
        }
        Node::Binary(op, left, right) => {
            // do left subtree, which stores result in r[stack_ptr]
            let mut instr = translate_subtree(*left, stack_ptr, next_label);
            // do right subtree, which stores result in r[stack_ptr + 1]
            instr.append(&mut translate_subtree(*right, stack_ptr + 1, next_label));
            // do op on r[stack_ptr] with r[stack_ptr + 1]
            instr.push(Instruction::Binary(stack_ptr, Source::Register(stack_ptr + 1), op));

            instr
        }
        Node::Then(left, right) => {
            // do left subtree, which stores result in r[stack_ptr]
            let mut instr = translate_subtree(*left, stack_ptr, next_label);
            // do right subtree, which stores result in r[stack_ptr]
            instr.append(&mut translate_subtree(*right, stack_ptr, next_label));

            instr
        }
        Node::Print(child) => {
            // do child, which stores result in r[stack_ptr]
            let mut instr = translate_subtree(*child, stack_ptr, next_label);
            // print r[stack_ptr]
            instr.push(Instruction::Print(Source::Register(stack_ptr)));

            instr
        }
        Node::Store(reg, child) => { // stores result of child in reg
            // execute child code, storing result in r[stack_ptr]
            let mut instr = translate_subtree(*child, stack_ptr, next_label);
            // r[reg] = r[stack_ptr]
            instr.push(Instruction::Binary(reg, Source::Register(stack_ptr), BinaryOperation::Set));

            instr
        }
        Node::If(cond, if_not_zero, if_zero) => {
            let label_else = *next_label;
            let label_after = label_else + 1;
            *next_label += 2;
            // process cond subtree, store result in r[stack_ptr]
            let mut instr = translate_subtree(*cond, stack_ptr, next_label);

            // if cond != 0, keep going to the if_not_zero block, then jump to label_after
            instr.push(Instruction::Jump(label_else, JumpCondition::Zero(stack_ptr)));
            instr.append(&mut translate_subtree(*if_not_zero, stack_ptr, next_label));
            instr.push(Instruction::Jump(label_after, JumpCondition::None));
            // else, jump to label_else, do the if_zero block, then continue to label_after
            instr.push(Instruction::Label(label_else));
            instr.append(&mut translate_subtree(*if_zero, stack_ptr, next_label));
            instr.push(Instruction::Label(label_after));

            instr
        }
        Node::While(condition, body) => {
            let label_before = *next_label;
            let label_after = label_before + 1;
            *next_label += 2;


            let mut instr = vec!(Instruction::Label(label_before));
            // calculate condition value into r[stack_ptr + 1] - don't overwrite last iteration result
            instr.append(&mut translate_subtree(*condition, stack_ptr + 1, next_label));
            // evaluate condition, skip loop body if null
            instr.push(Instruction::Jump(label_after, JumpCondition::Zero(stack_ptr + 1)));
            // execute loop body once, storing result in stack_ptr
            instr.append(&mut translate_subtree(*body, stack_ptr, next_label));
            // jump back to condition
            instr.push(Instruction::Jump(label_before, JumpCondition::None));
            instr.push(Instruction::Label(label_after));

            instr
        }
        Node::Turtle(operation) => {
            vec!(
                // do the operation, storing result in r[RESULT_REGISTER]
                Instruction::Turtle(operation),
                // copy r[RESULT_REGISTER] into r[stack_ptr] to move it up the tree
                Instruction::Binary(stack_ptr, Source::Register(RESULT_REGISTER), BinaryOperation::Set),
            )
        }
    }
}