pub struct NodeId {
    index: usize,
}

struct Node<T> {
    pub data: T,
}

pub struct Tree<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree { nodes: vec![] }
    }

    pub fn new_node(&mut self, data: T) -> NodeId {
        let index = self.nodes.len();
        self.nodes.push(Node { data });

        NodeId { index }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_node() {
        let mut tree = Tree::new();
        tree.new_node("COM");
        assert_eq!(tree.nodes.len(), 1);
    }
}
