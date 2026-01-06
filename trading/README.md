# Trading & Backtesting

Event-driven backtesting framework for trading strategies.

## Usage

```rust
use rust_quant_trading::{BacktestEngine, Order, OrderType, Side};

// Create backtesting engine with $10,000 initial capital
let mut engine = BacktestEngine::new(10000.0, 0.001);

// Submit a buy order
let order = Order {
    id: "1".to_string(),
    symbol: "AAPL".to_string(),
    side: Side::Buy,
    order_type: OrderType::Market,
    quantity: 10.0,
    price: None,
    timestamp: Utc::now(),
};
engine.submit_order(order)?;

// Process orders with current market prices
let mut prices = HashMap::new();
prices.insert("AAPL".to_string(), 100.0);
engine.process_orders(&prices);

// Get account state and trades
let account = engine.account();
let trades = engine.trades();
let returns = engine.returns();
```

## Features

- Event-driven backtesting
- Market, limit, and stop orders
- Position tracking
- Commission calculation
- Returns calculation

