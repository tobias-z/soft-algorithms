const SIZE: usize = 500;

struct HashTable<T>
where
    T: Clone,
{
    vals: [Option<Node<T>>; SIZE],
}

impl<T> HashTable<T>
where
    T: Clone,
{
    fn new() -> Self {
        Self {
            vals: [const { None }; SIZE],
        }
    }

    pub fn put(&mut self, key: &str, value: T) {
        let index = simple_hash(key) % SIZE;
        match self.vals.get_mut(index).unwrap() {
            Some(node) => {
                if node.key == key {
                    node.val = value;
                    return;
                }
                let mut node = node;
                while node.next.is_some() {
                    let node = node.next.as_mut().unwrap();
                    if node.key == key {
                        node.val = value;
                        return;
                    }
                }
                node.next = Some(Box::new(Node::new(key, value)))
            }
            None => {
                self.vals[index] = Some(Node::new(key, value));
            }
        }
    }

    pub fn get(&self, key: &str) -> Option<T> {
        let index = simple_hash(key) % SIZE;
        let node = self.vals.get(index).unwrap();
        match node {
            Some(node) => {
                if node.key == key {
                    return Some(node.val.clone());
                }
                let mut node = node;
                while node.next.is_some() {
                    node = node.next.as_ref().unwrap();
                    if node.key == key {
                        return Some(node.val.clone());
                    }
                }
                None
            }
            None => None,
        }
    }
}

/// converts each char into its ascii code and plus it all together
pub fn simple_hash(key: &str) -> usize {
    key.chars().fold(0, |old, new| old + (new as usize))
}

struct Node<T> {
    pub val: T,
    pub key: String,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(key: &str, val: T) -> Self {
        Self {
            next: None,
            key: key.to_string(),
            val,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn table_works() {
        let mut table = HashTable::<i32>::new();
        table.put("one", 1);
        table.put("one", 5);
        table.put("two", 2);
        let val = table.get("one");
        assert_eq!(5, val.unwrap());
    }

    #[test]
    fn can_handle_collision() {
        let mut table = HashTable::<i32>::new();
        table.put("on", 1);
        table.put("no", 2);
        assert_eq!(1, table.get("on").unwrap());
        assert_eq!(2, table.get("no").unwrap());
    }
}
