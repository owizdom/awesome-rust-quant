//! Time series data structures and operations
//!
//! This module provides time series data structures and common
//! operations like resampling, interpolation, and alignment.

use chrono::{DateTime, Duration, Utc};
use ndarray::Array1;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TimeSeriesError {
    #[error("Empty time series")]
    EmptySeries,
    #[error("Invalid time range")]
    InvalidTimeRange,
    #[error("Duplicate timestamp: {0:?}")]
    DuplicateTimestamp(DateTime<Utc>),
}

/// Time series data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeries {
    timestamps: Vec<DateTime<Utc>>,
    values: Vec<f64>,
}

impl TimeSeries {
    /// Create a new empty time series
    pub fn new() -> Self {
        Self {
            timestamps: Vec::new(),
            values: Vec::new(),
        }
    }

    /// Create from vectors
    pub fn from_vectors(
        timestamps: Vec<DateTime<Utc>>,
        values: Vec<f64>,
    ) -> Result<Self, TimeSeriesError> {
        if timestamps.len() != values.len() {
            return Err(TimeSeriesError::InvalidTimeRange);
        }

        // Check for duplicates
        let mut seen = std::collections::HashSet::new();
        for &ts in &timestamps {
            if !seen.insert(ts) {
                return Err(TimeSeriesError::DuplicateTimestamp(ts));
            }
        }

        // Sort by timestamp
        let mut pairs: Vec<(DateTime<Utc>, f64)> = timestamps.into_iter().zip(values).collect();
        pairs.sort_by_key(|(ts, _)| *ts);

        let (timestamps, values): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();

        Ok(Self { timestamps, values })
    }

    /// Add a data point
    pub fn add_point(&mut self, timestamp: DateTime<Utc>, value: f64) -> Result<(), TimeSeriesError> {
        // Check for duplicate
        if self.timestamps.contains(&timestamp) {
            return Err(TimeSeriesError::DuplicateTimestamp(timestamp));
        }

        // Insert in sorted order
        match self.timestamps.binary_search(&timestamp) {
            Ok(_) => Err(TimeSeriesError::DuplicateTimestamp(timestamp)),
            Err(pos) => {
                self.timestamps.insert(pos, timestamp);
                self.values.insert(pos, value);
                Ok(())
            }
        }
    }

    /// Get length
    pub fn len(&self) -> usize {
        self.timestamps.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.timestamps.is_empty()
    }

    /// Get value at index
    pub fn get(&self, index: usize) -> Option<(DateTime<Utc>, f64)> {
        if index < self.timestamps.len() {
            Some((self.timestamps[index], self.values[index]))
        } else {
            None
        }
    }

    /// Get values as array
    pub fn values(&self) -> Array1<f64> {
        Array1::from_vec(self.values.clone())
    }

    /// Resample to a different frequency
    pub fn resample(&self, interval: Duration, method: ResampleMethod) -> Result<Self, TimeSeriesError> {
        if self.is_empty() {
            return Err(TimeSeriesError::EmptySeries);
        }

        let start = self.timestamps[0];
        let end = *self.timestamps.last().unwrap();
        let mut new_timestamps = Vec::new();
        let mut new_values = Vec::new();

        let mut current = start;
        while current <= end {
            new_timestamps.push(current);
            
            // Find values in the interval
            let interval_start = current;
            let interval_end = current + interval;
            
            let values_in_range: Vec<f64> = self
                .timestamps
                .iter()
                .zip(self.values.iter())
                .filter(|(ts, _)| **ts >= interval_start && **ts < interval_end)
                .map(|(_, v)| *v)
                .collect();

            let value = match method {
                ResampleMethod::Mean => {
                    if values_in_range.is_empty() {
                        continue;
                    }
                    values_in_range.iter().sum::<f64>() / values_in_range.len() as f64
                }
                ResampleMethod::Sum => values_in_range.iter().sum(),
                ResampleMethod::Last => *values_in_range.last().unwrap_or(&0.0),
                ResampleMethod::First => *values_in_range.first().unwrap_or(&0.0),
            };

            new_values.push(value);
            current = current + interval;
        }

        Self::from_vectors(new_timestamps, new_values)
    }

    /// Fill missing values using forward fill
    pub fn forward_fill(&mut self) {
        if self.values.is_empty() {
            return;
        }

        let mut last_value = self.values[0];
        for value in &mut self.values {
            if value.is_nan() {
                *value = last_value;
            } else {
                last_value = *value;
            }
        }
    }

    /// Fill missing values using backward fill
    pub fn backward_fill(&mut self) {
        if self.values.is_empty() {
            return;
        }

        let mut last_value = *self.values.last().unwrap();
        for value in self.values.iter_mut().rev() {
            if value.is_nan() {
                *value = last_value;
            } else {
                last_value = *value;
            }
        }
    }

    /// Calculate returns
    pub fn returns(&self) -> Result<Self, TimeSeriesError> {
        if self.len() < 2 {
            return Err(TimeSeriesError::EmptySeries);
        }

        let mut new_timestamps = Vec::new();
        let mut new_values = Vec::new();

        for i in 1..self.timestamps.len() {
            new_timestamps.push(self.timestamps[i]);
            let ret = (self.values[i] - self.values[i - 1]) / self.values[i - 1];
            new_values.push(ret);
        }

        Self::from_vectors(new_timestamps, new_values)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ResampleMethod {
    Mean,
    Sum,
    Last,
    First,
}

impl Default for TimeSeries {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_series_creation() {
        let ts = TimeSeries::new();
        assert!(ts.is_empty());
    }

    #[test]
    fn test_time_series_add() {
        let mut ts = TimeSeries::new();
        let now = Utc::now();
        ts.add_point(now, 100.0).unwrap();
        assert_eq!(ts.len(), 1);
    }

    #[test]
    fn test_returns() {
        let mut ts = TimeSeries::new();
        let base = Utc::now();
        ts.add_point(base, 100.0).unwrap();
        ts.add_point(base + Duration::days(1), 110.0).unwrap();
        let returns = ts.returns().unwrap();
        assert_eq!(returns.len(), 1);
        assert!((returns.values()[0] - 0.1).abs() < 1e-10);
    }
}

