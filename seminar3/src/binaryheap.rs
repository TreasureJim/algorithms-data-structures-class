struct MinHeap<T> {
    data: Vec<T>
}

impl<T: PartialOrd> MinHeap<T> {
    pub fn new(mut vec: Vec<T>) -> Self {
        // find the parent of the last node
        let last_parent = MinHeap::<T>::parent_index(vec.len() - 1);
        let mut this = Self { data: vec };
        for i in last_parent..=0 {
            this.heapify(i);
        }

        this
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

    /// Heapify a subtree. Given a tree that has valid valid children but an invalid root. 
    /// Assumes the children are already heapified. 
    fn heapify(&mut self, mut index: usize) {
        let mut left = MinHeap::<T>::left_child_index(index);
        let mut right = MinHeap::<T>::right_child_index(index);

        while self.data[index] > self.data[left] || self.data[index] > self.data[right] {
            // TODO: If both are smaller then take the smallest of the 2
            if self.data[index] > self.data[left] {
                self.data.swap(index, left);
            } else {
                self.data.swap(index, right);
            }

            left = MinHeap::<T>::left_child_index(index);
            right = MinHeap::<T>::right_child_index(index);
        }
    }
}
