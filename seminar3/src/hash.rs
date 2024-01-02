use std::collections::LinkedList;

fn hash(x: usize) -> usize {
    x % 10
}

#[derive(Debug)]
struct KeyValue<X, Y> {
    pub key: X,
    pub value: Y,
}

impl<X, Y> KeyValue<X, Y> {
    fn new(key: X, value: Y) -> Self {
        Self { key, value }
    }
}

#[derive(Debug)]
pub struct SeparateChainingMap<X, Y> {
    list: Vec<LinkedList<KeyValue<X, Y>>>,
}

impl<X, Y> SeparateChainingMap<X, Y> {
    pub fn new(size: usize) -> Self {
        Self {
            list: Vec::with_capacity(size),
        }
    }
}

impl<Y> SeparateChainingMap<usize, Y> {
    pub fn insert(&mut self, key: usize, value: Y) {
        let i = hash(key);
        self.list[i].push_front(KeyValue::new(key, value));
    }

    pub fn get(&self, key: usize) -> Option<&Y> {
        let i = hash(key);
        self.list[i].iter().find(|&x| x.key == key).map(|x| &x.value)
    }
}
