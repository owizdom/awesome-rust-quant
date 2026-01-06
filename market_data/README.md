# Market Data

Utilities for downloading and parsing historical market data.

## Usage

```rust
use rust_quant_market_data::MarketDataDownloader;

let downloader = MarketDataDownloader::new();

// Download CSV data
let data = downloader.download_csv("https://example.com/data.csv").await?;

// Parse CSV string
let csv = "Date,Open,High,Low,Close,Volume\n2023-01-01,100.0,101.0,99.0,100.5,1000000";
let ohlcv_data = MarketDataDownloader::parse_csv(csv)?;

// Generate sample data for testing
let sample_data = MarketDataDownloader::generate_sample_data("AAPL", 100);
```

## Features

- CSV parsing for OHLCV data
- Historical data downloader (framework)
- Sample data generation
- Support for multiple data formats

