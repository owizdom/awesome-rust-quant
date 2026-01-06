//! High-frequency trading order book implementation
//!
//! This module provides a limit order book (LOB) for high-frequency trading
//! with efficient price-time priority matching.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrderBookError {
    #[error("Order not found: {0}")]
    OrderNotFound(String),
    #[error("Invalid price: {0}")]
    InvalidPrice(f64),
    #[error("Invalid quantity: {0}")]
    InvalidQuantity(f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub side: Side,
    pub price: f64,
    pub quantity: f64,
    pub timestamp: DateTime<Utc>,
}

impl Order {
    pub fn new(id: String, side: Side, price: f64, quantity: f64) -> Self {
        Self {
            id,
            side,
            price,
            quantity,
            timestamp: Utc::now(),
        }
    }
}

/// Price level in the order book
#[derive(Debug, Clone)]
struct PriceLevel {
    price: f64,
    orders: Vec<Order>,
    total_quantity: f64,
}

impl PriceLevel {
    fn new(price: f64) -> Self {
        Self {
            price,
            orders: Vec::new(),
            total_quantity: 0.0,
        }
    }

    fn add_order(&mut self, order: Order) {
        self.total_quantity += order.quantity;
        self.orders.push(order);
    }

    fn remove_order(&mut self, order_id: &str) -> Option<Order> {
        if let Some(pos) = self.orders.iter().position(|o| o.id == order_id) {
            let order = self.orders.remove(pos);
            self.total_quantity -= order.quantity;
            Some(order)
        } else {
            None
        }
    }
}

/// Limit Order Book for high-frequency trading
pub struct OrderBook {
    bids: BTreeMap<i64, PriceLevel>, // Negative prices for reverse ordering
    asks: BTreeMap<i64, PriceLevel>, // Positive prices
    orders: std::collections::HashMap<String, (Side, i64)>, // order_id -> (side, price_key)
}

impl OrderBook {
    /// Create a new empty order book
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            orders: std::collections::HashMap::new(),
        }
    }

    /// Add an order to the book
    pub fn add_order(&mut self, order: Order) -> Result<(), OrderBookError> {
        if order.price <= 0.0 {
            return Err(OrderBookError::InvalidPrice(order.price));
        }
        if order.quantity <= 0.0 {
            return Err(OrderBookError::InvalidQuantity(order.quantity));
        }

        let price_key = Self::price_to_key(order.price);
        let level_map = match order.side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        level_map
            .entry(price_key)
            .or_insert_with(|| PriceLevel::new(order.price))
            .add_order(order.clone());

        self.orders.insert(order.id.clone(), (order.side, price_key));
        Ok(())
    }

    /// Remove an order from the book
    pub fn remove_order(&mut self, order_id: &str) -> Result<Order, OrderBookError> {
        let (side, price_key) = self
            .orders
            .remove(order_id)
            .ok_or_else(|| OrderBookError::OrderNotFound(order_id.to_string()))?;

        let level_map = match side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        let level = level_map
            .get_mut(&price_key)
            .ok_or_else(|| OrderBookError::OrderNotFound(order_id.to_string()))?;

        level
            .remove_order(order_id)
            .ok_or_else(|| OrderBookError::OrderNotFound(order_id.to_string()))
    }

    /// Get the best bid (highest buy price)
    pub fn best_bid(&self) -> Option<f64> {
        self.bids.keys().next_back().map(|&k| Self::key_to_price(k))
    }

    /// Get the best ask (lowest sell price)
    pub fn best_ask(&self) -> Option<f64> {
        self.asks.keys().next().map(|&k| Self::key_to_price(k))
    }

    /// Get the spread (best ask - best bid)
    pub fn spread(&self) -> Option<f64> {
        match (self.best_ask(), self.best_bid()) {
            (Some(ask), Some(bid)) => Some(ask - bid),
            _ => None,
        }
    }

    /// Get the mid price ((best bid + best ask) / 2)
    pub fn mid_price(&self) -> Option<f64> {
        match (self.best_ask(), self.best_bid()) {
            (Some(ask), Some(bid)) => Some((ask + bid) / 2.0),
            _ => None,
        }
    }

    /// Get total quantity at a price level
    pub fn quantity_at_price(&self, side: Side, price: f64) -> f64 {
        let price_key = Self::price_to_key(price);
        let level_map = match side {
            Side::Buy => &self.bids,
            Side::Sell => &self.asks,
        };

        level_map
            .get(&price_key)
            .map(|level| level.total_quantity)
            .unwrap_or(0.0)
    }

    /// Get depth (number of price levels) on a side
    pub fn depth(&self, side: Side) -> usize {
        match side {
            Side::Buy => self.bids.len(),
            Side::Sell => self.asks.len(),
        }
    }

    /// Convert price to integer key for BTreeMap ordering
    fn price_to_key(price: f64) -> i64 {
        // Use negative for bids to get reverse ordering (highest first)
        // Use positive for asks to get normal ordering (lowest first)
        (price * 1_000_000.0) as i64
    }

    /// Convert integer key back to price
    fn key_to_price(key: i64) -> f64 {
        (key.abs() as f64) / 1_000_000.0
    }
}

impl Default for OrderBook {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_book_add() {
        let mut book = OrderBook::new();
        let order = Order::new("1".to_string(), Side::Buy, 100.0, 10.0);
        book.add_order(order).unwrap();
        assert_eq!(book.best_bid(), Some(100.0));
    }

    #[test]
    fn test_order_book_spread() {
        let mut book = OrderBook::new();
        book.add_order(Order::new("1".to_string(), Side::Buy, 100.0, 10.0))
            .unwrap();
        book.add_order(Order::new("2".to_string(), Side::Sell, 101.0, 10.0))
            .unwrap();
        assert_eq!(book.spread(), Some(1.0));
    }

    #[test]
    fn test_order_book_remove() {
        let mut book = OrderBook::new();
        let order = Order::new("1".to_string(), Side::Buy, 100.0, 10.0);
        book.add_order(order).unwrap();
        book.remove_order("1").unwrap();
        assert_eq!(book.best_bid(), None);
    }
}

