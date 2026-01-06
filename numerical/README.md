# Numerical Libraries & Data Structures

Basic DataFrame implementation and matrix operations for quantitative finance.

## Usage

```rust
use rust_quant_numerical::{DataFrame, matrix};

// Create a DataFrame
let columns = vec!["open".to_string(), "close".to_string(), "high".to_string()];
let data = vec![
    vec![100.0, 101.0, 102.0],
    vec![99.0, 102.0, 103.0],
    vec![98.0, 103.0, 104.0],
];
let mut df = DataFrame::from_columns(columns, data)?;

// Add a new column
df.add_column("volume".to_string(), vec![1000.0, 2000.0, 3000.0])?;

// Get a column
let close_prices = df.get_column("close")?;

// Convert to ndarray
let array = df.to_array2();
```

## Features

- DataFrame-like structure for financial data
- Matrix operations (multiplication, transpose)
- Integration with ndarray
- Column-based data access

