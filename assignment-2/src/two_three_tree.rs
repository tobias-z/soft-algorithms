use std::{cell::RefCell, rc::Rc, cmp::Ordering};

struct LeafNode {
    pub data: usize,
    parent: Option<Rc<RefCell<Node>>>,
}

struct TwoNode {
    pub data: usize,
    parent: Option<Rc<RefCell<Node>>>,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

struct ThreeNode {
    small: usize,
    large: usize,
    parent: Option<Rc<RefCell<Node>>>,
    left: Option<Rc<RefCell<Node>>>,
    middle: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

enum Node {
    LeafNode(LeafNode),
    TwoNode(TwoNode),
    ThreeNode(ThreeNode),
}

struct TwoThreeTree {
    root: Option<Rc<RefCell<Node>>>,
}

impl TwoThreeTree {
    fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, data: usize) {
        match self.root.to_owned() {
            Some(root) => {
                TwoThreeTree::insert_in_node(data, root);
            }
            None => {
                self.root = Some(Rc::new(RefCell::new(Node::LeafNode(LeafNode { data, parent: None }))));
            }
        }
    }

    fn insert_in_node(data: usize, node: Rc<RefCell<Node>>) {
        match *node.borrow() {
            Node::LeafNode(leaf) => {
                // If we hit a leaf, we should insert it here. Making the leaf a TwoNode
                match leaf.parent {
                    Some(parent) => {
                        match &mut *parent.borrow_mut() {
                            Node::LeafNode(_) => panic!("This should never be the case"),
                            Node::TwoNode(parent) if leaf.data > parent.data => {
                                // if larger we know that our leaf is the parents right node
                                let small = smallest(data, leaf.data);
                                let large = largest(data, leaf.data);
                                parent.right = Some(Rc::new(RefCell::new(Node::ThreeNode(ThreeNode { small, large, parent: leaf.parent, left: None, middle: None, right: None }))));
                            },
                            Node::TwoNode(parent) => {
                                // if less we know that our leaf is the parents left node
                            },
                            Node::ThreeNode(parent) => {

                            },
                        }
                    },
                    None => {
                        // if we do not have a parent that means we are dealing with the root node. therefor we modify self.root
                        let small = smallest(data, leaf.data);
                        let large = largest(data, leaf.data);
                        node = Rc::new(RefCell::new(Node::ThreeNode(ThreeNode { small, large, parent: None, left: None, middle: None, right: None })));
                    },
                }
            }
            Node::TwoNode(_) => todo!(),
            Node::ThreeNode(_) => todo!(),
        }
    }

    pub fn find(&self, data: usize) -> Option<Rc<RefCell<Node>>> {
        match self.root.as_ref() {
            Some(root) => TwoThreeTree::binary_search(data, root),
            None => None,
        }
    }

    fn binary_search(data: usize, node: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
        match &*node.borrow() {
            Node::LeafNode(leaf) => {
                if data == leaf.data {
                    return Some(Rc::clone(node));
                }
                None
            }
            Node::TwoNode(two) if data == two.data => Some(Rc::clone(node)),
            Node::TwoNode(two) if data > two.data => match &two.right {
                Some(right) => TwoThreeTree::binary_search(data, right),
                None => None,
            },
            Node::TwoNode(two) => match &two.left {
                Some(left) => TwoThreeTree::binary_search(data, left),
                None => None,
            },
            Node::ThreeNode(three) if data == three.small => Some(Rc::clone(node)),
            Node::ThreeNode(three) if data == three.large => Some(Rc::clone(node)),
            Node::ThreeNode(three) if data > three.large => match &three.right {
                Some(right) => TwoThreeTree::binary_search(data, right),
                None => None,
            },
            Node::ThreeNode(three) if data < three.small => match &three.left {
                Some(left) => TwoThreeTree::binary_search(data, left),
                None => None,
            },
            Node::ThreeNode(three) => match &three.middle {
                Some(middle) => TwoThreeTree::binary_search(data, middle),
                None => None,
            },
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
        let mut tree = TwoThreeTree::new();
        tree.insert(7);
        assert!(tree.find(7).is_some());
    }
}
