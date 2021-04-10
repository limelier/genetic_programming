use crate::trees::definitions::Node;
use crate::genetic::generation::{Method, generate};
use crate::genetic::definitions::{Generation, MUTATION_CHANCE, MAX_DEPTH};
use rand::{thread_rng, Rng};

impl Generation {
    pub(crate) fn mutate(&mut self) {
        let mut rng = thread_rng();
        for individual in &mut self.population {
            if rng.gen_bool(MUTATION_CHANCE) {
                mutate(&mut individual.tree);
            }
        }
    }
}

/// Replace a random sub-tree with a different random one of about the same depth
fn mutate(tree: &mut Node) {
    let point = tree.get_weighted_node_mut();
    let depth = point.get_max_depth();
    let mut branch = generate(Method::Grow, depth);
    std::mem::swap(point, &mut branch);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test] #[ignore]
    fn test_mutate() {
        let mut gen = Generation::random();
        gen.mutate();
    }
}