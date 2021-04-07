use crate::trees::definitions::{Node, Node::*};
use std::collections::VecDeque;

impl Node {
    pub fn is_leaf(&self) -> bool {
        match self {
            Null | Val(_) | Turtle(_) => true,
            _ => false,
        }
    }

    /// Get a vector of references to all of the node's children
    pub fn children(&self) -> Vec<&Node> {
        match self {
            Print(c1) |
            Unary(_, c1) |
            Store(_, c1)
                => vec!(c1),

            Binary(_, c1, c2) |
            While(c1, c2) |
            Compare(c1, c2) |
            Then(c1, c2)
                => vec!(c1, c2),

            If(c1, c2, c3)
                => vec!(c1, c2, c3),

            _ => vec!(),
        }
    }

    /// Iterate through all nodes of tree, in preorder, yielding references
    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        let mut queue: VecDeque<&Node> = VecDeque::new();
        queue.push_front(self);

        core::iter::from_fn(move || {
            let node = queue.pop_back()?;
            queue.extend(node.children().into_iter().rev());
            Some(node)
        })
    }

    /// Search for the nth node in a tree, in preorder.
    ///
    /// Return either:
    ///     - the count of the nodes in the tree, if the tree does not have an nth node
    ///     - a list of directions (which child next?) to get to the node from the root
    fn search_nth_node(&self, n: usize) -> SearchResult {
        if n == 0 {
            return SearchResult::Directions(vec!());
        }

        let mut checked_nodes = 1;
        for (idx, child) in self.children().iter().enumerate() {
            let res = child.search_nth_node(n - checked_nodes);

            match res {
                SearchResult::Count(count) => { checked_nodes += count; }
                SearchResult::Directions(dirs) => {
                    let mut full_dirs = vec!(idx as u8);
                    full_dirs.append(&mut dirs.clone());
                    return SearchResult::Directions(full_dirs);
                }
            }
        }

        SearchResult::Count(checked_nodes)
    }

    /// Get an immutable reference to the nth node in a tree, in preorder
    pub(crate) fn get_nth_node(&self, n: usize) -> Option<&Node> {
        let result = self.search_nth_node(n);

        match result {
            SearchResult::Directions(vec) => {
                let mut reference = self;
                for idx in vec {
                    reference = reference.children()[idx as usize];
                }
                Some(reference)
            }
            _ => None,
        }
    }

    /// Get a mutable reference to the nth child of a node, if it exists
    fn get_nth_child_mut(&mut self, n: u8) -> Option<&mut Node> {
        match self {
            Print(c1) |
            Unary(_, c1) |
            Store(_, c1) => {
                match n {
                    0 => Some(c1),
                    _ => None,
                }
            },

            Binary(_, c1, c2) |
            While(c1, c2) |
            Compare(c1, c2) |
            Then(c1, c2) => {
                match n {
                    0 => Some(c1),
                    1 => Some(c2),
                    _ => None,
                }
            },

            If(c1, c2, c3) => {
                match n {
                    0 => Some(c1),
                    1 => Some(c2),
                    2 => Some(c3),
                    _ => None,
                }
            },

            _ => None,
        }
    }

    /// Get an immutable reference to the nth node of a tree, in preorder
    pub(crate) fn get_nth_node_mut(&mut self, n: usize) -> Option<&mut Node> {
        let result = self.search_nth_node(n);

        match result {
            SearchResult::Directions(vec) => {
                let mut reference = self;
                for idx in vec {
                    reference = reference.get_nth_child_mut(idx).unwrap();
                }
                Some(reference)
            }
            _ => None,
        }
    }

    /// Get the number of nodes in the tree
    pub(crate) fn node_count(&self) -> usize {
        1 + self.children().iter().map(|child| child.node_count() ).sum::<usize>()
    }
}

#[derive(Debug)]
enum SearchResult {
    Count(usize),
    Directions(Vec<u8>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::definitions::{Source, BinaryOperation};

    fn big_tree() -> Node {
        Binary(
            BinaryOperation::Add,
            Box::from(Binary(
                BinaryOperation::Add,
                Box::from(Val(Source::Value(1))),
                Box::from(Val(Source::Value(2))),
            )),
            Box::from(Binary(
                BinaryOperation::Multiply,
                Box::from(Val(Source::Value(3))),
                Box::from(Val(Source::Value(4))),
            )),
        )
    }

    #[test]
    fn test_is_not_leaf() {
        let tree = Print(Box::from(Val(Source::Value(1))));

        assert!(!tree.is_leaf());
    }

    #[test]
    fn test_is_leaf() {
        let tree = Null;

        assert!(tree.is_leaf());
    }

    #[test]
    fn test_children() {
        let tree = Binary(
            BinaryOperation::Add,
            Box::from(Val(Source::Value(1))),
            Box::from(Val(Source::Value(2))),
        );

        let children = tree.children();

        if let Val(Source::Value(1)) = children[0] {
            return;
        }
        panic!("did not find expected child");
    }

    #[test]
    fn test_nodes() {
        let tree = big_tree();

        let nodes: Vec<&Node> = tree.nodes().collect();

        // check the presence and order of nodes
        if let Binary(BinaryOperation::Multiply, _, _) = nodes[4] {
            if let Val(Source::Value(2)) = nodes[3] {
                return;
            } else {
                panic!("did not find expected node (val 3)")
            }
        } else {
            panic!("did not find expected node (multiply)");
        }
    }

    #[test]
    fn test_search_nth_node_directions() {
        let tree = big_tree();

        let result = tree.search_nth_node(3);
        if let SearchResult::Directions(vec) = result {
            assert_eq!(vec, [0, 1]);
            return;
        }
        panic!("did not find node #3");
    }

    #[test]
    fn test_search_nth_node_count() {
        let tree = big_tree();

        let result = tree.search_nth_node(10);
        if let SearchResult::Count(n) = result {
            assert_eq!(n, 7);
            return;
        }
        panic!("returned directions instead");
    }

    #[test]
    fn test_get_nth_node_hit() {
        let tree = big_tree();

        let result = tree.get_nth_node(3);
        if let Some(&Val(Source::Value(2))) = result {
            return;
        }
        panic!("node not found");
    }

    #[test]
    fn test_get_nth_node_miss() {
        let tree = big_tree();

        let result = tree.get_nth_node(10);
        if let None = result {
            return;
        }
        panic!("some node found");
    }

    #[test]
    fn get_nth_child_mut_hit() {
        let mut tree = Print(Box::from(Val(Source::Value(1))));

        let child = tree.get_nth_child_mut(0);
        if let Some(&mut Val(Source::Value(1))) = child {
            return;
        }
        panic!("could not find child");
    }

    #[test]
    fn get_nth_child_mut_miss() {
        let mut tree = Print(Box::from(Val(Source::Value(1))));

        let child = tree.get_nth_child_mut(1);
        if let None = child {
            return;
        }
        panic!("found a child where none should have been found");
    }

    #[test]
    fn test_get_nth_node_mut_hit() {
        let mut tree = big_tree();

        let result = tree.get_nth_node_mut(3);
        if let Some(&mut Val(Source::Value(2))) = result {
            return;
        }
        panic!("node not found");
    }

    #[test]
    fn test_get_nth_node_mut_miss() {
        let mut tree = big_tree();

        let result = tree.get_nth_node_mut(10);
        if let None = result {
            return;
        }
        panic!("some node found");
    }

    #[test]
    fn test_node_count() {
        let tree = big_tree();
        assert_eq!(tree.node_count(), 7);
    }
}