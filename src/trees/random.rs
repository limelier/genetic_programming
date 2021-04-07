use rand::{Rng, thread_rng};

use crate::trees::definitions::Node;

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

    pub(crate) fn get_weighted_node_mut(&mut self) -> &mut Node {
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
}

fn weight(node: &&Node) -> usize { if node.is_leaf() { 1 } else { 9 } }


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