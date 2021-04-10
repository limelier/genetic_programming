use crate::genetic::definitions::*;
use crate::simulator::definitions::BlockSpace;
use crate::trees::translate::translate_tree;
use crate::vm::program::Program;

impl Generation {
    pub fn evaluate(&mut self, target: &BlockSpace) {
        for index in 0.. self.population.len() {
            let evaluated = &mut self.population[index];
            evaluate(evaluated, target);
            let result = evaluated.result.unwrap();

            if self.best_index == None || result > self.population[self.best_index.unwrap()].result.unwrap() {
                self.best_index = Some(index);
            }
        }
    }
}

fn evaluate(individual: &mut Individual, target: &BlockSpace) {
    let depth = individual.tree.get_max_depth();
    if depth > MAX_DEPTH {
        individual.result = Some(Result {
            score: SCORE_DEPTH_LIMIT_EXCEEDED,
            perfect: false,
        });
        return;
    }
    let instructions = translate_tree(&individual.tree);
    let mut program = Program::from_instructions(&instructions);
    let result = program.execute(Some(MAX_PROGRAM_RUNTIME_MS));

    individual.result = Some(match result {
        Ok(()) => {
            let state = program.get_simulator_state();
            let res = compare_state(target, &state);
            let score = res.air_absent as f64 * SCORE_ABSENT_AIR +
                res.air_present as f64 * SCORE_PRESENT_AIR +
                res.blocks_absent as f64 * SCORE_ABSENT_BLOCK +
                res.blocks_present as f64 * SCORE_PRESENT_BLOCK;

            Result {
                score: score * (DEPTH_SOFTENER + MAX_DEPTH as f64 - depth as f64) / (DEPTH_SOFTENER + MAX_DEPTH as f64),
                perfect: res.perfect,
            }
        },
        Err(_) => {
            Result {
                score: SCORE_PROGRAM_ERROR,
                perfect: false,
            }
        }
    })
}

#[derive(Default)]
struct ComparisonResult {
    air_present: usize,
    air_absent: usize,
    blocks_present: usize,
    blocks_absent: usize,
    perfect: bool,
}



fn compare_state(expected: &BlockSpace, actual: &BlockSpace) -> ComparisonResult {
    let mut res = ComparisonResult::default();

    for i in 0..expected.len() {
        for j in 0..expected[i].len() {
            for k in 0..expected[i][j].len() {
                let exp = &expected[i][j][k];
                let act = &actual[i][j][k];

                match exp {
                    0 => if act == exp { res.air_present += 1; } else { res.air_absent += 1; },
                    _ => if act == exp { res.blocks_present += 1; } else { res.blocks_absent += 1; },
                }
            }
        }
    }

    res.perfect = res.blocks_absent == 0 && res.air_absent == 0;

    res
}