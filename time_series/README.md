# Time Series

Time series data structures and operations for financial data.

## Usage

```rust
use rust_quant_time_series::{TimeSeries, ResampleMethod};
use chrono::{Duration, Utc};

let mut ts = TimeSeries::new();
ts.add_point(Utc::now(), 100.0)?;
ts.add_point(Utc::now() + Duration::days(1), 110.0)?;

// Calculate returns
let returns = ts.returns()?;

// Resample to daily frequency
let daily = ts.resample(Duration::days(1), ResampleMethod::Mean)?;

// Fill missing values
ts.forward_fill();
```

## Features

- Time-indexed data structure
- Resampling to different frequencies
- Forward/backward fill for missing values
- Returns calculation
- Sorted by timestamp

