use std::{
    collections::VecDeque,
    fmt::{self, Display},
};

pub struct MinHeap<T> {
    data: Vec<T>,
}

impl<T> MinHeap<T> {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    fn parent_index(i: usize) -> usize {
        if i == 0 {
            return 0;
        }
        (i - 1) / 2
    }

    fn left_child_index(i: usize) -> usize {
        2 * i + 1
    }

    fn right_child_index(i: usize) -> usize {
        2 * i + 2
    }
}

impl<T: PartialOrd> From<Vec<T>> for MinHeap<T> {
    fn from(vec: Vec<T>) -> Self {
        // find the parent of the last node
        let last_parent = MinHeap::<T>::parent_index(vec.len() - 1);
        let mut this = Self { data: vec };
        for i in (0..=last_parent).into_iter().rev() {
            this.heapify(i);
        }

        this
    }
}

impl<T: PartialOrd> MinHeap<T> {
    pub fn insert(&mut self, x: T) {
        self.data.push(x);
        let mut i = self.data.len() - 1;

        let mut parent_index = MinHeap::<T>::parent_index(i);
        while self.data[i] < self.data[parent_index] {
            self.data.swap(i, parent_index);
            i = parent_index;
            parent_index = MinHeap::<T>::parent_index(i);
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        assert!(index < self.data.len());

        // if leaf node
        if MinHeap::<T>::left_child_index(index) >= self.data.len()
            && MinHeap::<T>::right_child_index(index) >= self.data.len()
        {
            return Some(self.data.remove(index));
        }

        // swap with last leaf node
        let last_index = self.data.len() - 1;
        self.data.swap(index, last_index);
        let removed = self.data.pop();

        self.heapify(index);

        removed
    }

    /// Heapify a subtree. Given a tree that has valid valid children but an invalid root.
    /// Assumes the children are already heapified.
    fn heapify(&mut self, mut index: usize) {
        assert!(index < self.data.len());

        let mut left = MinHeap::<T>::left_child_index(index);
        let mut right = MinHeap::<T>::right_child_index(index);

        loop {
            let mut smallest = index;

            if let Some(node) = self.data.get(left)
                && self.data[smallest] > *node
            {
                smallest = left;
            }
            if let Some(node) = self.data.get(right)
                && self.data[smallest] > *node
            {
                smallest = right;
            }

            if smallest == index {
                break;
            }

            self.data.swap(index, smallest);
            index = smallest;

            left = MinHeap::<T>::left_child_index(index);
            right = MinHeap::<T>::right_child_index(index);
        }
    }

    #[allow(unused)]
    fn is_valid(&self) -> bool {
        if self.data.is_empty() {
            return true;
        }

        for index in 0..=(MinHeap::<T>::parent_index(self.data.len() - 1)) {
            let left = MinHeap::<T>::left_child_index(index);
            let right = MinHeap::<T>::right_child_index(index);

            if let Some(left_node) = self.data.get(left)
                && *left_node < self.data[index]
            {
                return false;
            }
            if let Some(right_node) = self.data.get(right)
                && *right_node < self.data[index]
            {
                return false;
            }
        }

        true
    }
}

impl<T: std::fmt::Display> MinHeap<T> {
    pub fn in_order(&self, index: usize) {
        // Traverse the left subtree, i.e., call Inorder(left->subtree)
        // Visit the root.
        // Traverse the right subtree, i.e., call Inorder(right->subtree)

        if index >= self.data.len() {
            return;
        }

        self.in_order(MinHeap::<T>::left_child_index(index));
        println!("{}", self.data[index]);
        self.in_order(MinHeap::<T>::right_child_index(index));
    }

    pub fn pre_order(&self, index: usize) {
        // Visit the root.
        // Traverse the left subtree, i.e., call Preorder(left->subtree)
        // Traverse the right subtree, i.e., call Preorder(right->subtree)

        if index >= self.data.len() {
            return;
        }

        println!("{}", self.data[index]);
        self.in_order(MinHeap::<T>::left_child_index(index));
        self.in_order(MinHeap::<T>::right_child_index(index));
    }

    pub fn post_order(&self, index: usize) {
        // Traverse the left subtree, i.e., call Postorder(left->subtree)
        // Traverse the right subtree, i.e., call Postorder(right->subtree)
        // Visit the root

        if index >= self.data.len() {
            return;
        }

        self.in_order(MinHeap::<T>::left_child_index(index));
        self.in_order(MinHeap::<T>::right_child_index(index));
        println!("{}", self.data[index]);
    }

    pub fn level_order(&self, index: usize) {
        assert!(index < self.data.len());

        // For each node, first the node is visited and then it’s child nodes are put in a FIFO queue.
        // Then again the first node is popped out and then it’s child nodes are put in a FIFO queue and repeat until queue becomes empty.

        println!("{}", self.data[index]);
        let mut fifo = VecDeque::from(vec![
            MinHeap::<T>::left_child_index(index),
            MinHeap::<T>::right_child_index(index),
        ]);

        while let Some(child) = fifo.pop_front() {
            let Some(node) = self.data.get(child) else {
                continue;
            };
            println!("{}", node);
            // TODO: optimise by checking if children exist
            fifo.push_back(MinHeap::<T>::left_child_index(child));
            fifo.push_back(MinHeap::<T>::right_child_index(child));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MinHeap;

    #[test]
    fn iter_insert() {
        let vec = vec![10, 12, 1, 14, 6, 5, 8, 15, 3, 9, 7, 4, 11, 13, 2];
        let mut result = MinHeap::new();

        for x in vec {
            result.insert(x);
        }

        assert_eq!(
            result.data,
            [1, 3, 2, 6, 7, 5, 4, 15, 14, 12, 9, 10, 11, 13, 8]
        );
    }

    #[test]
    fn from_vec() {
        let vec = vec![10, 12, 1, 14, 6, 5, 8, 15, 3, 9, 7, 4, 11, 13, 2];
        let heap = MinHeap::from(vec);

        assert!(heap.is_valid());
        assert_eq!(
            heap.data,
            [1, 3, 2, 12, 6, 4, 8, 15, 14, 9, 7, 5, 11, 13, 10]
        );
    }

    #[test]
    fn test_heapify() {
        let mut min_heap = MinHeap::new();
        min_heap.data = vec![3, 1, 4, 6, 8, 5, 9];

        min_heap.heapify(0);
        assert!(min_heap.is_valid());
        assert_eq!(min_heap.data, vec![1, 3, 4, 6, 8, 5, 9]);
    }

    #[test]
    fn test_is_valid() {
        // valid
        let mut heap = MinHeap::new();
        heap.data = vec![1, 3, 5, 3, 9, 9, 9];
        assert!(heap.is_valid());

        // invalid
        let mut heap = MinHeap::new();
        heap.data = vec![1, 3, 5, 2, 9, 9, 9];
        assert!(!heap.is_valid());
    }
}
