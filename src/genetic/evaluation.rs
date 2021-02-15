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

pub fn evaluate_individual(individual: &mut Individual, target: &BlockSpace) {
    let mut program = Program::from_instructions(&parse_bytes(&individual.chromosome));
    let result = program.execute();

    individual.score = Some(match result {
        Ok(()) => evaluate_state(target, &program.get_simulator_state()),
        Err(_) => SCORE_PROGRAM_ERROR
    });
}

pub fn evaluate_population(population: &mut Population, target: &BlockSpace) {
    for individual in population.iter_mut() {
        evaluate_individual(individual, target);
    }

    population.sort_unstable_by(|i1, i2| i1.score.unwrap().partial_cmp(&i2.score.unwrap()).unwrap())
}


