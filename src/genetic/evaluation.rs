use crate::vm::program::Program;
use crate::vm::structures::{Instruction, BlockSpace};

const SCORE_PRESENT_AIR: f64 = 0.0;
const SCORE_ABSENT_AIR: f64 = -0.5;
const SCORE_PRESENT_BLOCK: f64 = 1.0;
const SCORE_ABSENT_BLOCK: f64 = 0.0;

fn evaluate_state(expected: &BlockSpace, actual: &BlockSpace) -> f64 {
    let mut score = 0.0;
    for i in 0..expected.len() {
        for j in 0..expected[i].len() {
            for k in 0..expected[i][j].len() {
                let exp = &expected[i][j][k];
                let act = &actual[i][j][k];

                score += match exp {
                    0 => if act == exp { SCORE_PRESENT_AIR } else { SCORE_ABSENT_AIR },
                    _ => if act == exp { SCORE_PRESENT_BLOCK } else { SCORE_ABSENT_BLOCK },
                }
            }
        }
    }

    score
}

pub fn evaluate_instructions(instructions: &Vec<Instruction>, target: &BlockSpace) -> f64 {
    let mut program = Program::from_instructions(&instructions);
    program.execute();

    evaluate_state(target, &program.get_simulator_state())
}


