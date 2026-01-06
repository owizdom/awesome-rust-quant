//! Market data utilities
//!
//! This module provides utilities for downloading and parsing
//! historical market data from various sources.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MarketDataError {
    #[error("HTTP error: {0}")]
    HttpError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Invalid date range")]
    InvalidDateRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OHLCV {
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub symbol: String,
    pub price: f64,
    pub volume: f64,
    pub timestamp: DateTime<Utc>,
}

/// Historical data downloader
pub struct MarketDataDownloader {
    client: reqwest::Client,
}

impl MarketDataDownloader {
    /// Create a new market data downloader
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Download historical data from a CSV file (simplified)
    /// In production, this would connect to real APIs
    pub async fn download_csv(&self, url: &str) -> Result<Vec<OHLCV>, MarketDataError> {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| MarketDataError::HttpError(e.to_string()))?;

        let text = response
            .text()
            .await
            .map_err(|e| MarketDataError::HttpError(e.to_string()))?;

        Self::parse_csv(&text)
    }

    /// Parse CSV data into OHLCV format
    pub fn parse_csv(csv_data: &str) -> Result<Vec<OHLCV>, MarketDataError> {
        let mut records = Vec::new();
        let lines: Vec<&str> = csv_data.lines().collect();

        if lines.is_empty() {
            return Ok(records);
        }

        // Skip header if present
        let start = if lines[0].contains("Date") || lines[0].contains("date") {
            1
        } else {
            0
        };

        for line in lines.iter().skip(start) {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 5 {
                // Try to parse as: Date,Open,High,Low,Close,Volume
                let _date_str = parts[0].trim();
                let open: f64 = parts[1].trim().parse::<f64>()
                    .map_err(|e: std::num::ParseFloatError| MarketDataError::ParseError(e.to_string()))?;
                let high: f64 = parts[2].trim().parse::<f64>()
                    .map_err(|e: std::num::ParseFloatError| MarketDataError::ParseError(e.to_string()))?;
                let low: f64 = parts[3].trim().parse::<f64>()
                    .map_err(|e: std::num::ParseFloatError| MarketDataError::ParseError(e.to_string()))?;
                let close: f64 = parts[4].trim().parse::<f64>()
                    .map_err(|e: std::num::ParseFloatError| MarketDataError::ParseError(e.to_string()))?;
                let volume: f64 = if parts.len() > 5 {
                    parts[5].trim().parse().unwrap_or(0.0)
                } else {
                    0.0
                };

                // Try to parse date (simplified - in production use proper date parsing)
                let timestamp = Utc::now(); // Placeholder

                records.push(OHLCV {
                    timestamp,
                    open,
                    high,
                    low,
                    close,
                    volume,
                });
            }
        }

        Ok(records)
    }

    /// Generate sample data for testing
    pub fn generate_sample_data(_symbol: &str, days: usize) -> Vec<OHLCV> {
        let mut data = Vec::new();
        let mut price = 100.0;

        for i in 0..days {
            let change = (i as f64 * 0.1).sin() * 2.0;
            price += change;

            let open = price;
            let high = price + (change.abs() * 0.5);
            let low = price - (change.abs() * 0.5);
            let close = price + change * 0.3;
            let volume = 1000000.0 + (i as f64 * 1000.0);

            data.push(OHLCV {
                timestamp: Utc::now() - chrono::Duration::days((days - i) as i64),
                open,
                high,
                low,
                close,
                volume,
            });
        }

        data
    }
}

impl Default for MarketDataDownloader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv() {
        let csv = "Date,Open,High,Low,Close,Volume\n2023-01-01,100.0,101.0,99.0,100.5,1000000";
        let result = MarketDataDownloader::parse_csv(csv).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].open, 100.0);
    }

    #[test]
    fn test_generate_sample_data() {
        let data = MarketDataDownloader::generate_sample_data("AAPL", 10);
        assert_eq!(data.len(), 10);
    }
}

