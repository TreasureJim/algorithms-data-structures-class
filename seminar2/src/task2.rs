#![allow(unused)]

// https://stackoverflow.com/questions/69192/how-to-implement-a-queue-using-two-stacks

pub struct QueueTwoStacks<T> {
    inbox: Vec<T>,
    outbox: Vec<T>
}

impl<T> QueueTwoStacks<T> {
    pub fn new() -> Self { Self { inbox: Vec::new(), outbox: Vec::new() } }
    
    pub fn enqueue(&mut self, val: T) {
        self.inbox.push(val);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if !self.outbox.is_empty() {
            self.inbox.reverse();
            self.outbox.append(&mut self.inbox);
        }
        
        self.outbox.pop()
    }
}

pub struct QueueOneStack<T> {
    stack: Vec<Option<T>>,
    front: usize,
    back: usize
}

impl<T> QueueOneStack<T> {
    pub fn new(size: usize) -> Self { Self { stack: (0..size).map(|_| None).collect(), front: 0, back: 0 } }

    pub fn enqueue(&mut self, val: T) {
        if self.back == self.front {
            panic!("Values are being overwritten");
        }

        self.stack[self.back] = Some(val);
        self.back += self.increment_with_wrap(self.back);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        let x = self.stack[self.front].take();
        self.front += self.increment_with_wrap(self.front);
        x
    }

    fn increment_with_wrap(&self, index: usize) -> usize {
        (index + 1) % self.stack.len()
    }
}
