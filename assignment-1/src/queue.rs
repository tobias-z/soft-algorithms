const SIZE: usize = 10;

pub struct CircularQueue<T>
where
    T: Copy,
{
    front: usize,
    rear: usize,
    items: [Option<T>; SIZE],
    size: usize,
}

impl<T> CircularQueue<T>
where
    T: Copy,
{
    pub fn new() -> Self {
        Self {
            items: [None; SIZE],
            size: 0,
            front: 0,
            rear: 0,
        }
    }

    pub fn enqueue(&mut self, item: T) -> bool {
        if self.size == SIZE {
            return false;
        }
        self.items[self.rear] = Some(item);
        if self.rear == SIZE - 1 {
            self.rear = 0;
        } else {
            self.rear += 1;
        }
        self.size += 1;
        true
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let item = self.items[self.front];
        self.items[self.front] = None;
        self.front += 1;
        self.size -= 1;
        item
    }

    pub fn peek(&self) -> Option<T> {
        self.items[self.front]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn queue_works() {
        let mut queue = CircularQueue::<i32>::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(7);
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(7);
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(7);
        queue.enqueue(9);
        println!("{:?}", queue.enqueue(10));
        println!("{:?}", queue.peek());
        println!("{:?}", queue.dequeue());
        println!("{:?}", queue.enqueue(10));
        println!("{:?}", queue.dequeue());
        println!("{:?}", queue.dequeue());
        println!("{:?}", queue.dequeue());
    }
}
