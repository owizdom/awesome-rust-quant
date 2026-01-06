//! Common algorithm templates for quantitative finance
//!
//! This module provides common algorithms and data structures
//! useful in quantitative finance.

use std::collections::BinaryHeap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AlgorithmError {
    #[error("Empty collection")]
    EmptyCollection,
    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(usize),
}

/// Binary search for sorted arrays
pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Result<usize, AlgorithmError> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        match arr[mid].cmp(target) {
            std::cmp::Ordering::Equal => return Ok(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }

    Err(AlgorithmError::IndexOutOfBounds(left))
}

/// Quick sort implementation
pub fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot = partition(arr);
    quicksort(&mut arr[..pivot]);
    quicksort(&mut arr[pivot + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot = arr.len() - 1;
    let mut i = 0;

    for j in 0..pivot {
        if arr[j] <= arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot);
    i
}

/// Priority queue (max heap)
pub struct PriorityQueue<T: Ord> {
    heap: BinaryHeap<T>,
}

impl<T: Ord> PriorityQueue<T> {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.heap.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

impl<T: Ord> Default for PriorityQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Sliding window maximum
pub fn sliding_window_max(arr: &[f64], window_size: usize) -> Result<Vec<f64>, AlgorithmError> {
    if window_size == 0 || window_size > arr.len() {
        return Err(AlgorithmError::IndexOutOfBounds(window_size));
    }

    let mut result = Vec::new();
    for i in 0..=arr.len() - window_size {
        let window = &arr[i..i + window_size];
        let max = window.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        result.push(max);
    }

    Ok(result)
}

/// Calculate moving average efficiently
pub fn moving_average(arr: &[f64], window_size: usize) -> Result<Vec<f64>, AlgorithmError> {
    if window_size == 0 || window_size > arr.len() {
        return Err(AlgorithmError::IndexOutOfBounds(window_size));
    }

    let mut result = Vec::new();
    let mut sum: f64 = arr[..window_size].iter().sum();

    result.push(sum / window_size as f64);

    for i in window_size..arr.len() {
        sum = sum - arr[i - window_size] + arr[i];
        result.push(sum / window_size as f64);
    }

    Ok(result)
}

/// Find maximum subarray sum (Kadane's algorithm)
pub fn max_subarray_sum(arr: &[f64]) -> f64 {
    if arr.is_empty() {
        return 0.0;
    }

    let mut max_so_far = arr[0];
    let mut max_ending_here = arr[0];

    for &num in arr.iter().skip(1) {
        max_ending_here = num.max(max_ending_here + num);
        max_so_far = max_so_far.max(max_ending_here);
    }

    max_so_far
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let arr = vec![1, 3, 5, 7, 9];
        assert_eq!(binary_search(&arr, &5).unwrap(), 2);
    }

    #[test]
    fn test_quicksort() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        quicksort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_priority_queue() {
        let mut pq = PriorityQueue::new();
        pq.push(3);
        pq.push(1);
        pq.push(4);
        assert_eq!(pq.pop(), Some(4));
        assert_eq!(pq.pop(), Some(3));
    }

    #[test]
    fn test_sliding_window_max() {
        let arr = vec![1.0, 3.0, 2.0, 5.0, 4.0];
        let result = sliding_window_max(&arr, 3).unwrap();
        assert_eq!(result, vec![3.0, 5.0, 5.0]);
    }

    #[test]
    fn test_moving_average() {
        let arr = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = moving_average(&arr, 3).unwrap();
        assert_eq!(result, vec![2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_max_subarray_sum() {
        let arr = vec![-2.0, 1.0, -3.0, 4.0, -1.0, 2.0, 1.0, -5.0, 4.0];
        assert_eq!(max_subarray_sum(&arr), 6.0);
    }
}

