/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::{min, Ord};
use std::default::Default;
use std::mem::swap;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], // emm...
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
        //TODO
        // self.adjust_items_len();
        self.items.push(value);
        self.count += 1;
        let mut idx = self.count;
        while idx != 1 {
            let fa = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[fa]) {
                // swap(&mut self.items[idx], &mut self.items[fa]);
                self.items.swap(idx, fa);
                idx = fa;
            } else { break; }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		0
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        // self.adjust_items_len();
        if self.is_empty() {
            return None;
        }
        // swap(&mut items[0], &mut items[items.len() - 1]);
        self.items.swap(1, self.count);
        self.count -= 1;
        let mut idx = 1;

        // head adjust
        while self.left_child_idx(idx) < self.count {
            let left = self.left_child_idx(idx);
            let right = self.right_child_idx(idx);
            let mut minmax = idx;
            if left <= self.count && (self.comparator)(&self.items[left], &self.items[minmax]) {
                minmax = left;
            }
            if right <= self.count && (self.comparator)(&self.items[left], &self.items[minmax]) {
                minmax = left;
            }
            if minmax == idx { break; }
            // swap(&mut items[idx], &mut items[largest]);
            self.items.swap(idx, minmax);
            idx = minmax;
        }

        self.items.pop()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
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