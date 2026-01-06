//! Trading and backtesting framework
//!
//! This module provides an event-driven backtesting framework
//! for testing trading strategies.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TradingError {
    #[error("Insufficient funds: need {required}, have {available}")]
    InsufficientFunds { required: f64, available: f64 },
    #[error("Invalid order: {0}")]
    InvalidOrder(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub symbol: String,
    pub side: Side,
    pub order_type: OrderType,
    pub quantity: f64,
    pub price: Option<f64>, // None for market orders
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub id: String,
    pub symbol: String,
    pub side: Side,
    pub quantity: f64,
    pub price: f64,
    pub timestamp: DateTime<Utc>,
    pub commission: f64,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub avg_price: f64,
}

#[derive(Debug, Clone)]
pub struct Account {
    pub cash: f64,
    pub positions: std::collections::HashMap<String, Position>,
    pub equity: f64,
}

impl Account {
    pub fn new(initial_cash: f64) -> Self {
        Self {
            cash: initial_cash,
            positions: std::collections::HashMap::new(),
            equity: initial_cash,
        }
    }

    pub fn update_equity(&mut self, prices: &std::collections::HashMap<String, f64>) {
        let mut total_value = self.cash;
        for (symbol, position) in &self.positions {
            if let Some(&price) = prices.get(symbol) {
                total_value += position.quantity * price;
            }
        }
        self.equity = total_value;
    }
}

/// Event-driven backtesting engine
pub struct BacktestEngine {
    account: Account,
    trades: Vec<Trade>,
    orders: VecDeque<Order>,
    commission_rate: f64,
}

impl BacktestEngine {
    /// Create a new backtesting engine
    pub fn new(initial_cash: f64, commission_rate: f64) -> Self {
        Self {
            account: Account::new(initial_cash),
            trades: Vec::new(),
            orders: VecDeque::new(),
            commission_rate,
        }
    }

    /// Submit an order
    pub fn submit_order(&mut self, order: Order) -> Result<(), TradingError> {
        // Validate order
        if order.quantity <= 0.0 {
            return Err(TradingError::InvalidOrder("Quantity must be positive".to_string()));
        }

        // Check if we have enough cash for buy orders
        if order.side == Side::Buy {
            let required = order.price.unwrap_or(0.0) * order.quantity;
            if required > self.account.cash {
                return Err(TradingError::InsufficientFunds {
                    required,
                    available: self.account.cash,
                });
            }
        }

        self.orders.push_back(order);
        Ok(())
    }

    /// Process orders at current market price
    pub fn process_orders(&mut self, prices: &std::collections::HashMap<String, f64>) {
        let mut orders_to_process = std::mem::take(&mut self.orders);
        
        while let Some(order) = orders_to_process.pop_front() {
            if let Some(&price) = prices.get(&order.symbol) {
                let execution_price = match order.order_type {
                    OrderType::Market => price,
                    OrderType::Limit => {
                        match order.side {
                            Side::Buy => {
                                if let Some(limit) = order.price {
                                    if price <= limit {
                                        price
                                    } else {
                                        orders_to_process.push_back(order);
                                        continue;
                                    }
                                } else {
                                    price
                                }
                            }
                            Side::Sell => {
                                if let Some(limit) = order.price {
                                    if price >= limit {
                                        price
                                    } else {
                                        orders_to_process.push_back(order);
                                        continue;
                                    }
                                } else {
                                    price
                                }
                            }
                        }
                    }
                    OrderType::Stop => {
                        // Simplified stop order logic
                        price
                    }
                };

                if let Ok(trade) = self.execute_trade(&order, execution_price) {
                    self.trades.push(trade);
                }
            }
        }

        self.orders = orders_to_process;
    }

    fn execute_trade(&mut self, order: &Order, price: f64) -> Result<Trade, TradingError> {
        let commission = price * order.quantity * self.commission_rate;
        let total_cost = price * order.quantity + commission;

        match order.side {
            Side::Buy => {
                if total_cost > self.account.cash {
                    return Err(TradingError::InsufficientFunds {
                        required: total_cost,
                        available: self.account.cash,
                    });
                }
                self.account.cash -= total_cost;

                let position = self.account.positions.entry(order.symbol.clone()).or_insert(Position {
                    symbol: order.symbol.clone(),
                    quantity: 0.0,
                    avg_price: 0.0,
                });

                let total_value = position.quantity * position.avg_price + price * order.quantity;
                position.quantity += order.quantity;
                position.avg_price = total_value / position.quantity;
            }
            Side::Sell => {
                let position = self.account.positions.get_mut(&order.symbol)
                    .ok_or_else(|| TradingError::InvalidOrder("No position to sell".to_string()))?;

                if position.quantity < order.quantity {
                    return Err(TradingError::InvalidOrder("Insufficient position".to_string()));
                }

                position.quantity -= order.quantity;
                if position.quantity == 0.0 {
                    self.account.positions.remove(&order.symbol);
                }

                self.account.cash += price * order.quantity - commission;
            }
        }

        Ok(Trade {
            id: format!("trade_{}", self.trades.len()),
            symbol: order.symbol.clone(),
            side: order.side,
            quantity: order.quantity,
            price,
            timestamp: Utc::now(),
            commission,
        })
    }

    /// Get account state
    pub fn account(&self) -> &Account {
        &self.account
    }

    /// Get all trades
    pub fn trades(&self) -> &[Trade] {
        &self.trades
    }

    /// Calculate returns
    pub fn returns(&self) -> f64 {
        (self.account.equity - self.account.cash) / self.account.cash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backtest_engine() {
        let mut engine = BacktestEngine::new(10000.0, 0.001);
        let mut prices = std::collections::HashMap::new();
        prices.insert("AAPL".to_string(), 100.0);

        let order = Order {
            id: "1".to_string(),
            symbol: "AAPL".to_string(),
            side: Side::Buy,
            order_type: OrderType::Market,
            quantity: 10.0,
            price: None,
            timestamp: Utc::now(),
        };

        engine.submit_order(order).unwrap();
        engine.process_orders(&prices);
        assert_eq!(engine.trades().len(), 1);
    }
}

