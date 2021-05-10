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
            Repeat(c1, c2) |
            Then(c1, c2)
                => vec!(c1, c2),

            If(c1, c2, c3)
                => vec!(c1, c2, c3),

            Null | Val(_) | Turtle(_) => vec!(),
        }
    }

    /// Get a vector of references to all of the node's children
    pub fn children_mut(&mut self) -> Vec<&mut Node> {
        match self {
            Print(c1) |
            Unary(_, c1) |
            Store(_, c1)
            => vec!(c1),

            Binary(_, c1, c2) |
            While(c1, c2) |
            Compare(c1, c2) |
            Repeat(c1, c2) |
            Then(c1, c2)
            => vec!(c1, c2),

            If(c1, c2, c3)
            => vec!(c1, c2, c3),

            Null | Val(_) | Turtle(_) => vec!(),
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

    /// Get a mutable reference to the nth node of a tree, in preorder
    pub(crate) fn get_nth_node_mut(&mut self, n: usize) -> Option<(&mut Node, usize)> {
        match self.search_nth_node_mut(n, 0) {
            SearchResult::Count(_) => None,
            SearchResult::Hit(reference, depth) => Some((reference, depth)),
        }
    }

    /// Search for the nth child (in preorder) of a tree, returning:
    /// - node (&mut ref, depth), if the node was found
    /// - the count of the tree's nodes, otherwise
    fn search_nth_node_mut(&mut self, n: usize, depth: usize) -> SearchResult {
        if n == 0 {
            return SearchResult::Hit(self, depth);
        }
        let mut checked_nodes = 1;
        for child in self.children_mut() {
            let res = (*child).search_nth_node_mut(n - checked_nodes, depth + 1);

            match res {
                SearchResult::Count(count) => { checked_nodes += count; }
                SearchResult::Hit(_, _) => { return res; }
            }
        }

        SearchResult::Count(checked_nodes)
    }

    pub fn get_max_depth(&self) -> usize {
        if self.is_leaf() {
            1
        } else {
            1 + self.children().iter().map(|child| child.get_max_depth() ).max().unwrap()
        }
    }
}

#[derive(Debug)]
enum SearchResult<'a> {
    Count(usize),
    Hit(&'a mut Node, usize)
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
    fn test_get_nth_node_mut_hit() {
        let mut tree = big_tree();

        let result = tree.get_nth_node_mut(3);
        if let Some((&mut Val(Source::Value(2)), 2)) = result {
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
}