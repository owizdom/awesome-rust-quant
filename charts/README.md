# Charts/Plot

Charting and plotting utilities for financial data visualization.

## Usage

```rust
use rust_quant_charts::*;
use ndarray::array;

let data = array![100.0, 101.0, 99.0, 102.0, 98.0];

// Plot line chart
plot_line("chart.png", "Price Chart", "Time", "Price", data.view())?;

// Plot multiple series
let series = vec![
    ("Price", data.view()),
    ("SMA", sma_data.view()),
];
plot_multiple("multi.png", "Comparison", "Time", "Price", &series)?;
```

## Features

- Line charts
- Multiple series plotting
- OHLC candlestick charts (framework)
- Customizable labels and titles

