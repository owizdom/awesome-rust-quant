# Rust Awesome Quant

A curated list of insanely awesome libraries for Rust, packages and resources for Quants (Quantitative Finance)

> Inspired by [awesome-go-quant](https://github.com/goex-top/awesome-go-quant)  
> Only for Rust

## Implemented Tools

This repository contains **15 fully implemented Rust crates** for quantitative finance by ME, organized as a Cargo workspace. Each tool is ready out of the box with comprehensive error handling, documentation, and tests.

###  Implemented Categories

1. **Numerical** (`rust-quant-numerical`) - DataFrame implementation and matrix operations
   - DataFrame-like structure for financial data
   - Matrix operations (multiplication, transpose)
   - Integration with ndarray

2. **Statistics** (`rust-quant-statistics`) - Statistical functions
   - Mean, median, variance, standard deviation
   - Covariance and correlation
   - Skewness and kurtosis
   - Percentiles and quantiles

3. **Rate Limit** (`rust-quant-rate-limit`) - Token bucket rate limiter
   - Configurable rate and burst capacity
   - Non-blocking and blocking acquisition
   - Async/await support

4. **Orderbook** (`rust-quant-orderbook`) - High-frequency trading order book
   - Efficient price-time priority matching
   - BTreeMap-based implementation for O(log n) operations
   - Best bid/ask tracking and spread calculation

5. **Indicators** (`rust-quant-indicators`) - Technical analysis indicators
   - SMA, EMA (Simple and Exponential Moving Averages)
   - RSI (Relative Strength Index)
   - MACD (Moving Average Convergence Divergence)
   - Bollinger Bands
   - Stochastic Oscillator
   - ATR (Average True Range)

6. **Machine Learning** (`rust-quant-machine-learning`) - Basic ML utilities
   - Linear Regression (OLS)
   - K-Means Clustering
   - Basic classification utilities

7. **Trading** (`rust-quant-trading`) - Event-driven backtesting framework
   - Market, limit, and stop orders
   - Position tracking
   - Commission calculation
   - Returns calculation

8. **Market Data** (`rust-quant-market-data`) - Historical data utilities
   - CSV parsing for OHLCV data
   - Historical data downloader framework
   - Sample data generation

9. **Risk** (`rust-quant-risk`) - Risk metrics and analysis
   - Value at Risk (VaR)
   - Conditional VaR (CVaR)
   - Sharpe Ratio
   - Sortino Ratio
   - Maximum Drawdown
   - Beta and Alpha

10. **Time Series** (`rust-quant-time-series`) - Time series operations
    - Time-indexed data structure
    - Resampling to different frequencies
    - Forward/backward fill for missing values
    - Returns calculation

11. **Excel** (`rust-quant-excel`) - Excel file integration
    - Read Excel files (XLSX)
    - Write Excel files with data
    - Support for headers and multiple sheets

12. **Charts** (`rust-quant-charts`) - Charting and plotting
    - Line charts
    - Multiple series plotting
    - OHLC candlestick charts (framework)
    - Customizable labels and titles

13. **Algorithms** (`rust-quant-algorithms`) - Common algorithm templates
    - Binary search
    - Quick sort
    - Priority queue (max heap)
    - Sliding window algorithms
    - Moving average calculation
    - Maximum subarray sum (Kadane's algorithm)

14. **Scrapers** (`rust-quant-scrapers`) - Web scraping utilities
    - HTML fetching
    - CSS selector-based extraction
    - Table scraping
    - Price extraction

15. **Sentiment** (`rust-quant-sentiment`) - Text sentiment analysis
    - Text sentiment scoring (-1.0 to 1.0)
    - Positive/negative/neutral classification
    - Intensifier detection
    - Batch analysis
    - Financial domain-specific word lists

### Quick Start

```bash
# Clone the repository
git clone <repository-url>
cd vieww

# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Use a specific crate
cd numerical
cargo test
```

### Usage Example

```rust
// In your Cargo.toml
[dependencies]
rust-quant-statistics = { path = "../statistics" }
rust-quant-indicators = { path = "../indicators" }

// In your code
use rust_quant_statistics::*;
use rust_quant_indicators::*;
use ndarray::array;

let prices = array![100.0, 101.0, 99.0, 102.0, 98.0];
let sma = sma(prices.view(), 3)?;
let std = std_dev(prices.view())?;
```

## Existing Rust Libraries

The following is a curated list of existing Rust libraries for quantitative finance:

### Numerical Libraries & Data Structures

* [ndarray](https://github.com/rust-ndarray/ndarray) - An n-dimensional array library for Rust
* [nalgebra](https://github.com/dimforge/nalgebra) - Linear algebra library for Rust
* [polars](https://github.com/pola-rs/polars) - Fast multi-threaded DataFrame library in Rust
* [arrow2](https://github.com/jorgecarleitao/arrow2) - Apache Arrow implementation in Rust
* [datafusion](https://github.com/apache/arrow-datafusion) - Apache Arrow DataFusion query engine
* [rust_decimal](https://github.com/paupino/rust-decimal) - Decimal implementation for Rust

### Statistics

* [statrs](https://github.com/statrs-dev/statrs) - Statistical computation library for Rust
* [linfa](https://github.com/rust-ml/linfa) - Machine learning framework for Rust
* [smartcore](https://github.com/smartcorelib/smartcore) - Machine learning library for Rust

### Rate Limiting

* [governor](https://github.com/antifuchs/governor) - A rate-limiting library for Rust
* [ratelimit](https://github.com/antifuchs/ratelimit) - Rate limiting library for Rust
* [ratelimit_meter](https://github.com/antifuchs/ratelimit_meter) - Rate limiting meter for Rust

### Orderbook

* [orderbook](https://github.com/rust-quant/orderbook) - High-frequency trading order book implementation
* [orderbook-rs](https://github.com/orderbook-rs/orderbook-rs) - Order book data structure for Rust

### Indicators

* [ta](https://github.com/greyblake/ta-rs) - Technical analysis library for Rust
* [tulip](https://github.com/fabianboesiger/tulip) - Technical analysis indicators in Rust
* [technical](https://github.com/rust-quant/technical) - Technical analysis indicators library

### Machine Learning

* [candle](https://github.com/huggingface/candle) - Minimalist ML framework for Rust
* [burn](https://github.com/burn-rs/burn) - Comprehensive deep learning framework
* [linfa](https://github.com/rust-ml/linfa) - Machine learning framework for Rust
* [smartcore](https://github.com/smartcorelib/smartcore) - Machine learning library for Rust

### Trading & Backtesting

* [barter](https://github.com/barter-rs/barter) - Event-driven live-trading and backtesting framework
* [rustquant](https://github.com/avhz/RustQuant) - Comprehensive quantitative finance library
* [quantrs](https://github.com/carlobortolan/quantrs) - High-performance quantitative finance library
* [trading212](https://github.com/trading212/trading212-rs) - Trading212 API client for Rust

### Market Data

* [yahoo-finance-rs](https://github.com/denisidoro/yahoo-finance-rs) - Yahoo Finance API client for Rust
* [alpha-vantage-rs](https://github.com/alpha-vantage/alpha-vantage-rs) - Alpha Vantage API client
* [polygon-io](https://github.com/polygon-io/polygon-rs) - Polygon.io API client for Rust
* [iex-cloud](https://github.com/iexcloud/iex-cloud-rs) - IEX Cloud API client

### Risk Analysis

* [rustquant](https://github.com/avhz/RustQuant) - Includes risk analysis modules
* [quantrs](https://github.com/carlobortolan/quantrs) - Risk metrics and analysis

### Factor Analysis

* [linfa](https://github.com/rust-ml/linfa) - Includes PCA and factor analysis
* [smartcore](https://github.com/smartcorelib/smartcore) - Factor analysis tools

### Time Series

* [rustquant](https://github.com/avhz/RustQuant) - Time series analysis modules
* [polars](https://github.com/pola-rs/polars) - Time series operations on DataFrames
* [time-series](https://github.com/rust-quant/time-series) - Time series data structures

### Data Sources

* [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client for Rust
* [serde](https://github.com/serde-rs/serde) - Serialization framework
* [csv](https://github.com/BurntSushi/rust-csv) - CSV parsing library

### Excel Integration

* [calamine](https://github.com/tafia/calamine) - Excel file reader for Rust
* [rust_xlsxwriter](https://github.com/jmcnamara/rust_xlsxwriter) - Excel file writer for Rust
* [excelize-rs](https://github.com/excelize-rs/excelize) - Excel file manipulation library

### Charts/Plot

* [plotters](https://github.com/plotters-rs/plotters) - Rust plotting library
* [plotly](https://github.com/igiagkiozis/plotly) - Plotly bindings for Rust
* [egui](https://github.com/emilk/egui) - Immediate mode GUI with plotting capabilities

### Algorithm

* [algorithms](https://github.com/EbTech/rust-algorithms) - Common algorithms in Rust
* [rust-algorithms](https://github.com/TheAlgorithms/Rust) - Collection of algorithms in Rust

### Learn

* [rust-by-example](https://github.com/rust-lang/rust-by-example) - Learn Rust by example
* [rustlings](https://github.com/rust-lang/rustlings) - Small exercises to get you used to reading and writing Rust code

### Tool

* [evcxr](https://github.com/evcxr/evcxr) - Rust Jupyter kernel
* [cargo-expand](https://github.com/dtolnay/cargo-expand) - Cargo subcommand to expand macros

### Scraper

* [scraper](https://github.com/causal-agent/scraper) - HTML parsing and querying library
* [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client for Rust
* [select](https://github.com/utkarshkukreti/select.rs) - HTML parsing library
* [headless_chrome](https://github.com/atroche/rust-headless-chrome) - Chrome DevTools Protocol driver

### Sentiment Intensity Analyzer

* [vader-sentiment-rs](https://github.com/ckw017/vader-sentiment-rs) - VADER sentiment analysis in Rust
* [sentiment](https://github.com/rust-quant/sentiment) - Sentiment analysis library

## About

This repository is a Rust implementation of [awesome-go-quant](https://github.com/goex-top/awesome-go-quant), providing both:

1. **15 fully implemented Rust crates** for quantitative finance (see Implemented Tools above)
2. **A curated list** of existing Rust libraries in the ecosystem

All implementations are organized as a Cargo workspace and can be used independently or together.

### Resources

* [RustQuant Book](https://avhz.github.io/RustQuant/) - Comprehensive guide to quantitative finance in Rust
* [Rust for Quantitative Finance](https://rustmeup.com/rust-for-quantitative-finance) - Article on using Rust in quant finance

### Contributing

Contributions are welcome! Each crate is designed to be extensible. Feel free to:
- Add more indicators
- Improve existing algorithms
- Add more risk metrics
- Enhance documentation
- Add more examples

