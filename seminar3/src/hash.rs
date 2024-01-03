use std::collections::LinkedList;

fn hash(x: usize) -> usize {
    x
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
            list: (0..size).into_iter().map(|_| LinkedList::new()).collect(),
        }
    }
}

impl<Y> SeparateChainingMap<usize, Y> {
    fn hash(&self, key: usize) -> usize {
        hash(key) % self.list.len()
    }

    pub fn insert(&mut self, key: usize, value: Y) {
        let i = self.hash(key);
        self.list[i].push_front(KeyValue::new(key, value));
    }

    pub fn get(&self, key: usize) -> Option<&Y> {
        let i = self.hash(key);
        self.list[i]
            .iter()
            .find(|&x| x.key == key)
            .map(|x| &x.value)
    }
}

const INSERTION_ATTEMPTS: usize = 10;

#[derive(Debug)]
pub struct LinearProbingMap<X, Y> {
    list: Vec<Option<KeyValue<X, Y>>>,
}

impl<X, Y> LinearProbingMap<X, Y> {
    pub fn new(size: usize) -> Self {
        Self {
            list: (0..size).into_iter().map(|_| None).collect()
        }
    }
}

impl<Y> LinearProbingMap<usize, Y> {
    fn hash(&self, key: usize, attempt: usize) -> usize {
        (hash(key) + attempt) % self.list.len()
    }

    pub fn insert(&mut self, key: usize, value: Y) -> Option<&Y> {
        for attempt in 0..INSERTION_ATTEMPTS {
            let i = self.hash(key, attempt);
            if self.list[i].is_none() {
                self.list[i] = Some(KeyValue::new(key, value));
                return self.list[i].as_ref().map(|x| &x.value);
            }
        }
        None
    }

    pub fn get(&self, key: usize) -> Option<&Y> {
        for attempt in 0..INSERTION_ATTEMPTS {
            let i = self.hash(key, attempt);
            if let Some(elem) = &self.list[i]
                && elem.key == key
            {
                return self.list[i].as_ref().map(|x| &x.value);
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct QuadraticProbingMap<X, Y> {
    list: Vec<Option<KeyValue<X, Y>>>,
}

impl<X, Y> QuadraticProbingMap<X, Y> {
    pub fn new(size: usize) -> Self {
        Self {
            list: (0..size).into_iter().map(|_| None).collect()
        }
    }
}

impl<Y> QuadraticProbingMap<usize, Y> {
    fn hash(&self, key: usize, attempt: usize) -> usize {
        (hash(key) + attempt.pow(2)) % self.list.len()
    }

    pub fn insert(&mut self, key: usize, value: Y) -> Option<&Y> {
        for attempt in 0..INSERTION_ATTEMPTS {
            let i = self.hash(key, attempt);
            if self.list[i].is_none() {
                self.list[i] = Some(KeyValue::new(key, value));
                return self.list[i].as_ref().map(|x| &x.value);
            }
        }
        None
    }

    pub fn get(&self, key: usize) -> Option<&Y> {
        for attempt in 0..INSERTION_ATTEMPTS {
            let i = self.hash(key, attempt);
            if let Some(elem) = &self.list[i]
                && elem.key == key
            {
                return self.list[i].as_ref().map(|x| &x.value);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separate_chaining_map_insert_and_get() {
        let mut map: SeparateChainingMap<usize, &str> = SeparateChainingMap::new(5);
        map.insert(1, "value1");
        map.insert(6, "value2");

        dbg!(&map);
        assert_eq!(map.get(1), Some(&"value1"));
        assert_eq!(map.get(6), Some(&"value2"));
        assert_eq!(map.get(2), None);
    }

    #[test]
    fn linear_probing_map_insert_and_get() {
        let mut map: LinearProbingMap<usize, &str> = LinearProbingMap::new(5);
        map.insert(1, "value1");
        map.insert(6, "value2");

        dbg!(&map);
        assert_eq!(map.get(1), Some(&"value1"));
        assert_eq!(map.get(6), Some(&"value2"));
        assert_eq!(map.get(2), None);
    }

    #[test]
    fn quadratic_probing_map_insert_and_get() {
        let mut map: QuadraticProbingMap<usize, &str> = QuadraticProbingMap::new(5);
        map.insert(1, "value1");
        map.insert(6, "value2");

        dbg!(&map);
        assert_eq!(map.get(1), Some(&"value1"));
        assert_eq!(map.get(6), Some(&"value2"));
        assert_eq!(map.get(2), None);
    }
}
