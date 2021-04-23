use crate::genetic::definitions::*;
use crate::simulator::definitions::BlockSpace;
use crate::trees::translate::translate_tree;
use crate::vm::program::Program;

impl Generation {
    pub fn evaluate(&mut self, target: &BlockSpace) {
        for index in 0..self.population.len() {
            let evaluated = &mut self.population[index];
            evaluate(evaluated, target);
            let result = evaluated.result.unwrap();

            if self.best_index == None || result > self.population[self.best_index.unwrap()].result.unwrap() {
                self.best_index = Some(index);
            }

            // only store finite worst scores, not -inf
            if result.score.is_finite() && (self.worst_index == None || result < self.population[self.worst_index.unwrap()].result.unwrap()) {
                self.worst_index = Some(index)
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
        }
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


#[cfg(test)]
mod test {
    use crate::simulator::definitions::Direction;
    use crate::trees::definitions::Node;
    use crate::vm::definitions::{Source, TurtleOperation, UnaryOperation};

    use super::*;

    fn target_line_of_eight() -> BlockSpace {
        let mut target = BlockSpace::default();
        for i in 0..8 {
            target[i][1][0] = 1;
        }
        target
    }

    #[test]
    fn test_eval_simple() {
        let tree = Node::Turtle(TurtleOperation::Place(Direction::Up));
        let mut individual = Individual { tree, result: None };
        evaluate(&mut individual, &target_line_of_eight());

        let expected_block_score = 1. * SCORE_PRESENT_BLOCK
            + 7. * SCORE_ABSENT_BLOCK
            + (16. * 16. * 16. - 8.) * SCORE_PRESENT_AIR;

        let expected_score = expected_block_score
            * (DEPTH_SOFTENER + MAX_DEPTH as f64 - 1.)
            / (DEPTH_SOFTENER + MAX_DEPTH as f64);

        assert_eq!(individual.result.unwrap().score, expected_score);
    }

    #[test]
    fn test_eval_deep() {
        let tree = Node::Then(
            Box::from(Node::Null),
            Box::from(Node::Then(
                Box::from(Node::Null),
                Box::from(Node::Turtle(TurtleOperation::Place(Direction::Up))),
            )),
        );
        let mut individual = Individual { tree, result: None };
        evaluate(&mut individual, &target_line_of_eight());

        let expected_block_score = 1. * SCORE_PRESENT_BLOCK
            + 7. * SCORE_ABSENT_BLOCK
            + (16. * 16. * 16. - 8.) * SCORE_PRESENT_AIR;

        let expected_score = expected_block_score
            * (DEPTH_SOFTENER + MAX_DEPTH as f64 - 3.)
            / (DEPTH_SOFTENER + MAX_DEPTH as f64);

        assert_eq!(individual.result.unwrap().score, expected_score);
    }

    #[test]
    fn test_eval_perfect() {
        // r0 = 8; while (r0) { place(up); go(forward); r0 = r0 - 1; }
        let tree = Node::Then(
            Box::from(Node::Store(
                0,
                Box::from(Node::Val(
                    Source::Value(8)
                )),
            )),
            Box::from(Node::While(
                Box::from(Node::Val(
                    Source::Register(0)
                )),
                Box::from(Node::Then(
                    Box::from(Node::Turtle(TurtleOperation::Place(Direction::Up))),
                    Box::from(Node::Then(
                        Box::from(Node::Turtle(TurtleOperation::Move(Direction::Forward))),
                        Box::from(Node::Store(
                            0,
                            Box::from(Node::Unary(
                                UnaryOperation::Decrement,
                                Box::from(Node::Val(Source::Register(0))),
                            )),
                        )),
                    )),
                )),
            )),
        );

        let mut individual = Individual { tree, result: None };
        evaluate(&mut individual, &target_line_of_eight());

        assert!(individual.result.unwrap().perfect);
    }

    #[test]
    fn test_eval_perfect_simpler() {
        // repeat (8) { place.up(); move.forward(); }
        let tree = Node::Repeat(
            Box::from(Node::Val(Source::Value(8))),
            Box::from(Node::Then(
                Box::from(Node::Turtle(TurtleOperation::Place(Direction::Up))),
                Box::from(Node::Turtle(TurtleOperation::Move(Direction::Forward))),
            )),
        );

        let mut individual = Individual { tree, result: None };
        evaluate(&mut individual, &target_line_of_eight());

        assert!(individual.result.unwrap().perfect);
    }
}