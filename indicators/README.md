# Technical Analysis Indicators

Common technical analysis indicators for trading strategies.

## Usage

```rust
use rust_quant_indicators::*;
use ndarray::array;

let prices = array![100.0, 101.0, 99.0, 102.0, 98.0, 103.0];

// Simple Moving Average
let sma_5 = sma(prices.view(), 5)?;

// Exponential Moving Average
let ema_5 = ema(prices.view(), 5)?;

// Relative Strength Index
let rsi_14 = rsi(prices.view(), 14)?;

// MACD
let (macd_line, signal_line, histogram) = macd(prices.view(), 12, 26, 9)?;

// Bollinger Bands
let (upper, middle, lower) = bollinger_bands(prices.view(), 20, 2.0)?;
```

## Features

- SMA, EMA
- RSI (Relative Strength Index)
- MACD (Moving Average Convergence Divergence)
- Bollinger Bands
- Stochastic Oscillator
- ATR (Average True Range)

