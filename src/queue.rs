use std::collections::VecDeque;

pub struct FifoQueue<T> {
    queue: VecDeque<T>,
}

impl<T> FifoQueue<T> {
    pub fn new() -> Self {
        FifoQueue {
            queue: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.queue.push_back(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}