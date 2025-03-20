use std::cmp::Ord;

pub struct Heap<T> {
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T> {
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: Vec::new(),
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        self.heapify_up(self.count - 1);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        2 * idx + 1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        2 * idx + 2
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        if right < self.count && (self.comparator)(&self.items[right], &self.items[left]) {
            right
        } else {
            left
        }
    }

    fn heapify_up(&mut self, idx: usize) {
        let mut current = idx;
        while current > 0 {
            let parent = self.parent_idx(current);
            if (self.comparator)(&self.items[current], &self.items[parent]) {
                self.items.swap(current, parent);
                current = parent;
            } else {
                break;
            }
        }
    }

    fn heapify_down(&mut self, idx: usize) {
        let mut current = idx;
        while self.left_child_idx(current) < self.count {
            let smallest_child = self.smallest_child_idx(current);
            if (self.comparator)(&self.items[smallest_child], &self.items[current]) {
                self.items.swap(smallest_child, current);
                current = smallest_child;
            } else {
                break;
            }
        }
    }
}

impl<T> Iterator for Heap<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let root = self.items.swap_remove(0);
        self.count -= 1;
        if !self.is_empty() {
            self.heapify_down(0);
        }

        Some(root)
    }
}

pub struct MinHeap;

impl MinHeap {
    pub fn new<T>() -> Heap<T>
    where
        T: Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    pub fn new<T>() -> Heap<T>
    where
        T: Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}