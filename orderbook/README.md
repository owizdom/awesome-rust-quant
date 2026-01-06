# Order Book

High-frequency trading order book implementation with price-time priority.

## Usage

```rust
use rust_quant_orderbook::{OrderBook, Order, Side};

let mut book = OrderBook::new();

// Add buy order
let buy_order = Order::new("1".to_string(), Side::Buy, 100.0, 10.0);
book.add_order(buy_order)?;

// Add sell order
let sell_order = Order::new("2".to_string(), Side::Sell, 101.0, 10.0);
book.add_order(sell_order)?;

// Get best bid/ask
let best_bid = book.best_bid();
let best_ask = book.best_ask();
let spread = book.spread();
let mid_price = book.mid_price();

// Remove order
book.remove_order("1")?;
```

## Features

- Efficient price-time priority matching
- BTreeMap-based implementation for O(log n) operations
- Best bid/ask tracking
- Spread and mid-price calculation
- Order depth tracking

