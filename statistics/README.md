# Statistics

Statistical functions for quantitative finance.

## Usage

```rust
use rust_quant_statistics::*;
use ndarray::array;

let data = array![100.0, 101.0, 99.0, 102.0, 98.0];

// Basic statistics
let avg = mean(data.view())?;
let std = std_dev(data.view())?;
let med = median(data.view())?;

// Correlation
let x = array![1.0, 2.0, 3.0, 4.0, 5.0];
let y = array![2.0, 4.0, 6.0, 8.0, 10.0];
let corr = correlation(x.view(), y.view())?;

// Percentiles
let p95 = percentile(data.view(), 0.95)?;
```

## Features

- Mean, median, variance, standard deviation
- Covariance and correlation
- Skewness and kurtosis
- Percentiles and quantiles
- Min/max values

