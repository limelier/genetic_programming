use crate::vm::program::Program;
use crate::vm::structures::BlockSpace;
use crate::genetic::definitions::*;
use crate::binary::parse_bytes;

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

pub fn evaluate_individual(individual: &Individual, target: &BlockSpace) -> f64 {
    let mut program = Program::from_instructions(&parse_bytes(individual));
    let result = program.execute();

    match result {
        Ok(()) => evaluate_state(target, &program.get_simulator_state()),
        Err(_) => SCORE_PROGRAM_ERROR
    }
}


