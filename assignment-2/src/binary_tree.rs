use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Node<T>
where
    T: Clone,
{
    pub left: Box<Option<Node<T>>>,
    pub right: Box<Option<Node<T>>>,
    pub val: T,
}

impl<T> Node<T>
where
    T: Clone,
{
    fn new(val: T) -> Self {
        Self {
            left: Box::new(None),
            right: Box::new(None),
            val,
        }
    }

    fn mutate_right(val: &T, right: &mut Node<T>) -> bool {
        let mut new_right = Node::new(val.clone());
        new_right.right = Box::new(Some(right.clone()));
        *right = new_right;
        true
    }

    fn mutate_left(val: &T, left: &mut Node<T>) -> bool {
        let mut new_left = Node::new(val.clone());
        new_left.left = Box::new(Some(left.clone()));
        *left = new_left;
        true
    }

    /// Inserts val at the the correct spot in the tree. The comparator is used to determine where
    /// the value should be placed.
    pub fn insert<F>(&mut self, val: &T, compare: &F) -> bool
    where
        F: Fn(&T, &T) -> i32,
    {
        match ((*self.right).as_mut(), (*self.left).as_mut()) {
            (Some(right), None) if compare(val, &right.val) == 0 => {
                Node::<T>::mutate_right(val, right)
            }
            (Some(right), None) if compare(val, &right.val) > 0 => right.insert(val, compare),
            (Some(right), None) if compare(val, &right.val) < 0 => {
                self.left = Box::new(Some(Node::new(val.clone())));
                true
            }
            (None, Some(left)) if compare(val, &left.val) == 0 => Node::<T>::mutate_left(val, left),
            (None, Some(left)) if compare(val, &left.val) > 0 => {
                self.right = Box::new(Some(Node::new(val.clone())));
                true
            }
            (None, Some(left)) if compare(val, &left.val) < 0 => left.insert(val, compare),
            (None, Some(left)) => left.insert(val, compare),
            (Some(_), Some(left)) if compare(val, &left.val) == 0 => {
                Node::<T>::mutate_left(val, left)
            }
            (Some(right), Some(_)) if compare(val, &right.val) == 0 => {
                Node::<T>::mutate_right(val, right)
            }
            (Some(right), Some(left)) if compare(val, &left.val) > 0 => right.insert(val, compare),
            (Some(right), Some(left)) if compare(val, &right.val) < 0 => left.insert(val, compare),
            (None, None) => {
                self.right = Box::new(Some(Node::new(val.clone())));
                true
            }
            _ => false,
        }
    }

    // If:
    //  Left = None && Right = Some -> point val to Right and delete curr node
    //  Left = Some && Right = None -> point val to Left and delete curr node
    //  Left = Some && Right = Some -> move smallest from right to curr
    pub fn delete<F>(mut self, val: &T, compare: &F) -> Option<Node<T>>
    where
        F: Fn(&T, &T) -> i32,
    {
        let compared = compare(val, &self.val);
        if self.right.is_some() && self.left.is_none() {
            let right = (*self.right).as_mut().unwrap();
            match compared.cmp(&0) {
                Ordering::Equal => {
                    self.left = right.left.clone();
                    self.right = right.right.clone();
                    self.val = val.clone();
                    return Some(self);
                }
                Ordering::Greater => {
                    self.right = Box::new(self.right.unwrap().delete(val, compare));
                    return *self.right;
                }
                Ordering::Less => return None,
            }
        }
        if self.right.is_none() && self.left.is_some() {
            let left = (*self.left).as_mut().unwrap();
            match compared.cmp(&0) {
                Ordering::Equal => {
                    self.right = left.right.clone();
                    self.left = left.left.clone();
                    self.val = val.clone();
                    return Some(self);
                }
                Ordering::Less => {
                    self.left = Box::new(self.left.unwrap().delete(val, compare));
                    return *self.left;
                }
                Ordering::Greater => return None,
            }
        }
        if self.right.is_some() && self.left.is_some() {
            match compared.cmp(&0) {
                Ordering::Equal => {
                    // Move smallest from right to curr
                    let smallest: Node<T> =
                        Node::<T>::get_smallest((*self.right).as_ref().unwrap());
                    self.right.unwrap().delete(&smallest.val, compare);
                    self.val = smallest.val;
                    return None;
                }
                Ordering::Less => {
                    self.left = Box::new(self.left.unwrap().delete(val, compare));
                    return *self.left;
                }
                Ordering::Greater => {
                    self.right = Box::new(self.right.unwrap().delete(val, compare));
                    return *self.right;
                }
            }
        }
        None
    }

    fn get_smallest(node: &Node<T>) -> Node<T> {
        if node.left.is_some() {
            return Node::<T>::get_smallest((*node.left).as_ref().unwrap());
        }
        node.clone()
    }
}

pub fn create_tree<T>(items: Vec<T>) -> Option<Node<T>>
where
    T: Clone,
{
    create_node(&items, 0, (items.len() - 1) as isize)
}

fn create_node<T>(items: &Vec<T>, left: isize, right: isize) -> Option<Node<T>>
where
    T: Clone,
{
    if left > right {
        return None;
    }

    let middle = left + (right - left) / 2;
    let index = middle as usize;
    let mut node = Node::new(items.get(index).cloned().unwrap());
    node.left = Box::new(create_node(items, left, middle - 1));
    node.right = Box::new(create_node(items, middle + 1, right));
    Some(node)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_create_tree() {
        let root = create_tree(vec![1, 2, 3, 4, 5]).unwrap();
        assert_eq!(root.val, 3);
    }

    #[test]
    fn can_insert_into_tree() {
        let mut root = create_tree(vec![1, 2, 3, 4, 5]).unwrap();
        root.insert(&4, &|val1, val2| val1 - val2);
        let right = root.right.unwrap();
        assert_eq!(right.val, 4);
        assert_eq!(right.right.unwrap().val, 4);
    }
}
