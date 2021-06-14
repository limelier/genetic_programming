use rand::{Rng, thread_rng};

use crate::trees::definitions::Node;
use rand::prelude::SliceRandom;
use crate::simulator::definitions::{Direction, Side};
use crate::vm::definitions::{Source, TurtleOperation};

impl Node {
    pub(crate) fn get_weighted_node(&self) -> &Node {
        let nodes: Vec<&Node> = self.nodes().collect();
        let wheel_size = nodes.iter().map(weight).sum();
        let mut needle = thread_rng().gen_range(0..wheel_size);
        let mut index = 0;
        loop {
            let w = weight(&nodes[index]);
            if w >= needle {
                break;
            }
            needle -= w;
            index += 1;
        }
        nodes[index]
    }

    /// Get a weighted-random node from the tree and its depth
    pub(crate) fn get_weighted_node_mut(&mut self) -> (&mut Node, usize) {
        let nodes: Vec<&Node> = self.nodes().collect();
        let wheel_size = nodes.iter().map(weight).sum();
        let mut needle = thread_rng().gen_range(0..wheel_size);
        let mut index = 0;
        loop {
            let w = weight(&nodes[index]);
            if w >= needle {
                break;
            }
            needle -= w;
            index += 1;
        }
        self.get_nth_node_mut(index).unwrap()
    }

    /// Move down randomly to a certain depth; stop if a leaf occurs before then
    pub(crate) fn randomly_descend(&self, depth: usize) -> &Node {
        if depth == 0 || self.is_leaf() {
            self
        } else {
            self.children().choose(&mut thread_rng()).unwrap().randomly_descend(depth - 1)
        }
    }
}

fn weight(node: &&Node) -> usize { if node.is_leaf() { 1 } else { 9 } }

pub(crate) fn random_useful_leaf() -> Node {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::trees::definitions::Node::*;
    use crate::vm::definitions::{Source, BinaryOperation};

    fn big_tree() -> Node {
        Binary(
            BinaryOperation::Add,
            Box::from(Binary(
                BinaryOperation::Add,
                Box::from(Val(Source::Value(0))),
                Box::from(Val(Source::Value(1))),
            )),
            Box::from(Binary(
                BinaryOperation::Multiply,
                Box::from(Val(Source::Value(2))),
                Box::from(Val(Source::Value(3))),
            )),
        )
    }

    #[test] #[ignore]
    fn get_weighted_node() {
        let tree = big_tree();
        dbg!(tree.get_weighted_node());
    }

    #[test] #[ignore]
    fn get_weighted_node_mut() {
        let mut tree = big_tree();
        dbg!(tree.get_weighted_node_mut());
    }
}