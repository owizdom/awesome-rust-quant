# Algorithms

Common algorithm templates for quantitative finance.

## Usage

```rust
use rust_quant_algorithms::*;

// Binary search
let arr = vec![1, 3, 5, 7, 9];
let index = binary_search(&arr, &5)?;

// Priority queue
let mut pq = PriorityQueue::new();
pq.push(10);
pq.push(5);
let max = pq.pop();

// Sliding window maximum
let arr = vec![1.0, 3.0, 2.0, 5.0, 4.0];
let maxes = sliding_window_max(&arr, 3)?;

// Moving average
let ma = moving_average(&arr, 3)?;

// Maximum subarray sum
let max_sum = max_subarray_sum(&arr);
```

## Features

- Binary search
- Quick sort
- Priority queue (max heap)
- Sliding window algorithms
- Moving average calculation
- Maximum subarray sum (Kadane's algorithm)

