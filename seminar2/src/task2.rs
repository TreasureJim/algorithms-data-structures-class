#![allow(unused)]

// https://stackoverflow.com/questions/69192/how-to-implement-a-queue-using-two-stacks

use std::collections::VecDeque;

// Write routines to implement a queue using two stacks.
pub struct QueueTwoStacks<T> {
    inbox: Vec<T>,
    outbox: Vec<T>,
}

impl<T> QueueTwoStacks<T> {
    pub fn new() -> Self {
        Self {
            inbox: Vec::new(),
            outbox: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, val: T) {
        self.inbox.push(val);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.outbox.is_empty() {
            self.inbox.reverse();
            self.outbox.append(&mut self.inbox);
        }

        self.outbox.pop()
    }
}

// Write routines to implement a queue using only one stacks.
pub struct QueueOneStack<T> {
    stack: Vec<Option<T>>,
    front: usize,
    back: usize,
}

impl<T> QueueOneStack<T> {
    pub fn new(size: usize) -> Self {
        Self {
            stack: (0..size).map(|_| None).collect(),
            front: 0,
            back: 0,
        }
    }

    pub fn enqueue(&mut self, val: T) {
        if self.back == self.front {
            eprintln!("Overflow");
            self.front += self.increment_with_wrap(self.front);
        }

        self.stack[self.back] = Some(val);
        self.back += self.increment_with_wrap(self.back);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        let x = self.stack[self.front].take();
        if x.is_some() {
            eprintln!("Underflow");
            self.front += self.increment_with_wrap(self.front);
        }
        x
    }

    fn increment_with_wrap(&self, index: usize) -> usize {
        (index + 1) % self.stack.len()
    }
}

// Write routines to implement a stack using two queues.
// https://www.geeksforgeeks.org/implement-stack-using-queue/
struct StackTwoQueues<T> {
    q1: VecDeque<T>,
    q2: VecDeque<T>,
}

impl<T> StackTwoQueues<T> {
    pub fn new() -> Self { Self { q1: VecDeque::new(), q2: VecDeque::new() } }

    pub fn push(&mut self, x: T) {
        self.q2.push_back(x);
        while let Some(temp) = self.q1.pop_front() {
            self.q2.push_back(temp);
        }
        std::mem::swap(&mut self.q1, &mut self.q2);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.q1.pop_front()
    }
}

// Write routines to implement a stack using a single queue
// Came up with it without help :)
struct StackOneQueues<T> {
    queue: VecDeque<T>,
}

impl<T> StackOneQueues<T> {
    pub fn new() -> Self { Self { queue: VecDeque::new() } }

    pub fn push(&mut self, x: T) {
        self.queue.push_back(x);
        while let Some(temp) = self.queue.pop_front() {
            self.queue.push_back(temp);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.queue.pop_front()
    }
}
