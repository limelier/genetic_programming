use crate::trees::definitions::Node;
use crate::vm::definitions::{BinaryOperation, Source, TurtleOperation, UnaryOperation};
use rand::Rng;
use crate::simulator::definitions::{Direction, Side};
use crate::genetic::definitions::{Generation, Individual, MAX_DEPTH, MIN_DEPTH, INDIVIDUALS_PER_METHOD_AND_DEPTH, POPULATION_SIZE};

#[derive(Copy, Clone, PartialEq)]
pub enum Method {
    Full,
    Grow,
}

impl Generation {
    /// Generate a generation from scratch, using ramped half-and-half
    /// Creates individuals with a max depth from MIN_DEPTH to MAX_DEPTH,
    /// generating INDIVIDUALS_PER_METHOD_AND_DEPTH of them with each method per depth.
    pub fn generate() -> Self {
        let mut population = Vec::with_capacity(POPULATION_SIZE);
        for depth in MIN_DEPTH..=MAX_DEPTH {
            for _ in 0..INDIVIDUALS_PER_METHOD_AND_DEPTH {
                population.push(Individual {
                    tree: generate(Method::Grow, depth),
                    result: None,
                });
                population.push(Individual {
                    tree: generate(Method::Full, depth),
                    result: None,
                });
            }
        }

        Generation {
            population,
            best: None,
        }
    }
}

pub fn generate(method: Method, max_depth: usize) -> Node {
    recurse(method, max_depth, 0)
}

fn recurse(method: Method, max_depth: usize, current_depth: usize) -> Node {
    let mut rng = rand::thread_rng();

    if current_depth == max_depth || (method == Method::Grow && rng.gen_bool(0.1)) {
        random_terminal()
    } else {
        random_function(method, max_depth, current_depth)
    }
}

fn random_terminal() -> Node {
    let mut rng = rand::thread_rng();

    if rng.gen::<bool>() {
        Node::Val(
            if rng.gen::<bool>() {
                Source::Value(rng.gen::<i8>())
            } else {
                Source::Register(rng.gen::<u8>())
            }
        )
    } else {
        Node::Turtle(
            match rng.gen_range(0..=4) {
                0 => TurtleOperation::Move(rng.gen::<Direction>()),
                1 => TurtleOperation::Turn(rng.gen::<Side>()),
                2 => TurtleOperation::Place(rng.gen::<Direction>()),
                3 => TurtleOperation::Dig(rng.gen::<Direction>()),
                _ => TurtleOperation::Detect(rng.gen::<Direction>()),
            }
        )
    }
}

fn random_function(method: Method, max_depth: usize, current_depth: usize) -> Node {
    let mut rng = rand::thread_rng();

    match rng.gen_range(0..=20) {
        0..=6 => Node::Unary(
            rng.gen::<UnaryOperation>(),
            Box::from(recurse(method, max_depth, current_depth + 1)),
        ),
        7..=15 => Node::Binary(
            rng.gen::<BinaryOperation>(),
            Box::from(recurse(method, max_depth, current_depth + 1)),
            Box::from(recurse(method, max_depth, current_depth + 1)),
        ),
        16 => Node::Compare(
            Box::from(recurse(method, max_depth, current_depth + 1)),
            Box::from(recurse(method, max_depth, current_depth + 1)),
        ),
        17 => Node::Store(
            rng.gen::<u8>(),
            Box::from(recurse(method, max_depth, current_depth + 1)),
        ),
        18 => Node::If(
            Box::from(recurse(method, max_depth, current_depth + 1)),
            Box::from(recurse(method, max_depth, current_depth + 1)),
            Box::from(recurse(method, max_depth, current_depth + 1)),
        ),
        19 => Node::While(
            Box::from(recurse(method, max_depth, current_depth + 1)),
            Box::from(recurse(method, max_depth, current_depth + 1)),
        ),
        _ => Node::Then(
            Box::from(recurse(method, max_depth, current_depth + 1)),
            Box::from(recurse(method, max_depth, current_depth + 1)),
        ),
    }
}