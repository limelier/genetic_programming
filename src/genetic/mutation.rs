use crate::trees::definitions::Node;
use crate::genetic::generation::{Method, generate};
use crate::genetic::definitions::{Generation, MUTATION_CHANCE, MUTATION_SINGLE_CHANCE};
use rand::{thread_rng, Rng};
use crate::vm::definitions::*;
use crate::trees::random::random_useful_leaf;

impl Generation {
    pub(crate) fn mutate(&mut self) {
        let mut rng = thread_rng();
        for individual in &mut self.population {
            if rng.gen_bool(MUTATION_CHANCE) {
                mutate(&mut individual.tree);
            }
            if rng.gen_bool(MUTATION_SINGLE_CHANCE) {
                mutate_single(&mut individual.tree);
            }
        }
    }
}

/// Replace a weighted-random sub-tree with a different random one of about the same depth
fn mutate(tree: &mut Node) {
    let (point, _) = tree.get_weighted_node_mut();
    let depth = point.get_max_depth();
    let mut branch = generate(Method::Grow, depth);
    std::mem::swap(point, &mut branch);
}

// Replace a random node with one of the same arity, keeping children
fn mutate_single(tree: &mut Node) {
    let node = tree.get_random_node_mut();
    *node = random_with_children(node.children());
}

pub(crate) fn random_with_children(children: Vec<&Node>) -> Node {
    use Node::*;

    let mut rng = rand::thread_rng();

    match children.len() {
        0 => random_useful_leaf(),
        1 => {
            let i = rng.gen_range(0..=1);
            let c = Box::from(children[0].clone());
            match i {
                0 => Unary(rng.gen::<UnaryOperation>(), c),
                1 => Store(rng.gen::<u8>(), c),
                _ => unreachable!(),
            }
        },
        2 => {
            let i = rng.gen_range(0..=4);
            let c1 = Box::from(children[0].clone());
            let c2 = Box::from(children[1].clone());
            match i {
                0 => Binary(rng.gen::<BinaryOperation>(), c1, c2),
                1 => While(c1, c2),
                2 => Compare(c1, c2),
                3 => Repeat(c1, c2),
                4 => Then(c1, c2),
                _ => unreachable!(),
            }
        },
        3 => {
            let c1 = Box::from(children[0].clone());
            let c2 = Box::from(children[1].clone());
            let c3 = Box::from(children[2].clone());

            If(c1, c2, c3)
        },
        _ => unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::trees::definitions::Node::Null;

    #[test] #[ignore]
    fn test_mutate() {
        let mut gen = Generation::random();
        gen.mutate();
    }

    #[test] #[ignore]
    fn test_random_with_children() {
        let child = Null;
        let node = random_with_children(vec!(&child, &child, &child));

        dbg!(node);
    }
}