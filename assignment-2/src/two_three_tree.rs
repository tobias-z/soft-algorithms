use std::{cmp::Ordering, ptr::addr_of_mut};

#[derive(Debug)]
struct LeafNode {
    pub data: usize,
    parent: Option<*mut Node>,
}

#[derive(Debug)]
struct TwoNode {
    pub data: usize,
    parent: Option<*mut Node>,
    left: Option<*mut Node>,
    right: Option<*mut Node>,
}

#[derive(Debug)]
struct ThreeNode {
    small: usize,
    large: usize,
    parent: Option<*mut Node>,
    left: Option<*mut Node>,
    middle: Option<*mut Node>,
    right: Option<*mut Node>,
}

#[derive(Debug)]
enum Node {
    Leaf(LeafNode),
    Two(TwoNode),
    Three(ThreeNode),
}

struct TwoThreeTree {
    root: Option<*mut Node>,
    // The nodes array is used for us to store each the nodes somewhere, this lets us create
    // pointers to them. In reality this should prob be a hashmap where the key is the value of the
    // data and the value is a linkedlist with all nodes holding that data. This would allow us to
    // easily handle deletion of nodes, without having to traverse the whole tree.
    nodes: Vec<Node>,
}

impl TwoThreeTree {
    fn new() -> TwoThreeTree {
        Self {
            root: None,
            nodes: vec![],
        }
    }

    pub fn insert(&mut self, data: usize) {
        match self.root {
            Some(root) => {
                TwoThreeTree::insert_in_node(data, root);
            }
            None => {
                let mut node: Node = Node::Leaf(LeafNode { data, parent: None });
                let node_ptr = addr_of_mut!(node);
                self.nodes.push(node);
                self.root = Some(node_ptr);
            }
        }
    }

    fn insert_in_node(data: usize, node: *mut Node) {}

    pub fn find(&self, data: usize) -> Option<*mut Node> {
        match self.root {
            Some(root) => TwoThreeTree::binary_search(data, root),
            None => None,
        }
    }

    fn binary_search(data: usize, node: *mut Node) -> Option<*mut Node> {
        unsafe {
            match node.as_ref() {
                Some(n) => match n {
                    Node::Leaf(leaf) => {
                        if data == leaf.data {
                            return Some(node);
                        }
                        None
                    }
                    Node::Two(two) if data == two.data => Some(node),
                    Node::Two(two) if data > two.data => match two.right {
                        Some(right) => TwoThreeTree::binary_search(data, right),
                        None => None,
                    },
                    Node::Two(two) => match two.left {
                        Some(left) => TwoThreeTree::binary_search(data, left),
                        None => None,
                    },
                    Node::Three(three) if data == three.small => Some(node),
                    Node::Three(three) if data == three.large => Some(node),
                    Node::Three(three) if data > three.large => match three.right {
                        Some(right) => TwoThreeTree::binary_search(data, right),
                        None => None,
                    },
                    Node::Three(three) if data < three.small => match three.left {
                        Some(left) => TwoThreeTree::binary_search(data, left),
                        None => None,
                    },
                    Node::Three(three) => match three.middle {
                        Some(middle) => TwoThreeTree::binary_search(data, middle),
                        None => None,
                    },
                },
                None => None,
            }
        }
    }
}

pub fn smallest(x: usize, y: usize) -> usize {
    match x.cmp(&y) {
        Ordering::Less => x,
        Ordering::Equal => x,
        Ordering::Greater => y,
    }
}

pub fn largest(x: usize, y: usize) -> usize {
    match x.cmp(&y) {
        Ordering::Less => y,
        Ordering::Equal => y,
        Ordering::Greater => x,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_insert_root() {
        // assert!(tree.find(7).is_some());
        unsafe {
            let mut tree = TwoThreeTree::new();
            tree.insert(7);
            let x = tree.find(7).unwrap();
            if let Node::Leaf(leaf) = &*x.as_mut().unwrap() {
                println!("{:?}", leaf.data);
            }
        }
    }
}
