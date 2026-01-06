//! Technical analysis indicators
//!
//! This module provides common technical analysis indicators:
//! SMA, EMA, RSI, MACD, Bollinger Bands, etc.

use ndarray::{s, Array1, ArrayView1};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndicatorError {
    #[error("Insufficient data: need at least {required} points, got {actual}")]
    InsufficientData { required: usize, actual: usize },
    #[error("Invalid period: {0}")]
    InvalidPeriod(usize),
}

/// Simple Moving Average (SMA)
pub fn sma(data: ArrayView1<f64>, period: usize) -> Result<Array1<f64>, IndicatorError> {
    if period == 0 {
        return Err(IndicatorError::InvalidPeriod(period));
    }
    if data.len() < period {
        return Err(IndicatorError::InsufficientData {
            required: period,
            actual: data.len(),
        });
    }

    let mut result = Array1::zeros(data.len() - period + 1);
    for i in 0..result.len() {
        let sum: f64 = data.slice(s![i..i + period]).iter().sum();
        result[i] = sum / period as f64;
    }
    Ok(result)
}

/// Exponential Moving Average (EMA)
pub fn ema(data: ArrayView1<f64>, period: usize) -> Result<Array1<f64>, IndicatorError> {
    if period == 0 {
        return Err(IndicatorError::InvalidPeriod(period));
    }
    if data.is_empty() {
        return Err(IndicatorError::InsufficientData {
            required: 1,
            actual: 0,
        });
    }

    let multiplier = 2.0 / (period + 1) as f64;
    let mut result = Array1::zeros(data.len());
    result[0] = data[0];

    for i in 1..data.len() {
        result[i] = (data[i] - result[i - 1]) * multiplier + result[i - 1];
    }
    Ok(result)
}

/// Relative Strength Index (RSI)
pub fn rsi(data: ArrayView1<f64>, period: usize) -> Result<Array1<f64>, IndicatorError> {
    if period == 0 {
        return Err(IndicatorError::InvalidPeriod(period));
    }
    if data.len() < period + 1 {
        return Err(IndicatorError::InsufficientData {
            required: period + 1,
            actual: data.len(),
        });
    }

    let mut gains = Vec::new();
    let mut losses = Vec::new();

    for i in 1..data.len() {
        let change = data[i] - data[i - 1];
        gains.push(if change > 0.0 { change } else { 0.0 });
        losses.push(if change < 0.0 { -change } else { 0.0 });
    }

    let gains_arr = Array1::from_vec(gains);
    let losses_arr = Array1::from_vec(losses);

    let avg_gain = sma(gains_arr.view(), period)?;
    let avg_loss = sma(losses_arr.view(), period)?;

    let mut result = Array1::zeros(avg_gain.len());
    for i in 0..result.len() {
        if avg_loss[i] == 0.0 {
            result[i] = 100.0;
        } else {
            let rs = avg_gain[i] / avg_loss[i];
            result[i] = 100.0 - (100.0 / (1.0 + rs));
        }
    }
    Ok(result)
}

/// Moving Average Convergence Divergence (MACD)
pub fn macd(
    data: ArrayView1<f64>,
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> Result<(Array1<f64>, Array1<f64>, Array1<f64>), IndicatorError> {
    if fast_period >= slow_period {
        return Err(IndicatorError::InvalidPeriod(fast_period));
    }

    let fast_ema = ema(data, fast_period)?;
    let slow_ema = ema(data, slow_period)?;

    let macd_line = &fast_ema - &slow_ema;
    let signal_line = ema(macd_line.view(), signal_period)?;

    let histogram = &macd_line.slice(s![signal_period - 1..]) - &signal_line;

    Ok((macd_line, signal_line, histogram))
}

/// Bollinger Bands
pub fn bollinger_bands(
    data: ArrayView1<f64>,
    period: usize,
    num_std: f64,
) -> Result<(Array1<f64>, Array1<f64>, Array1<f64>), IndicatorError> {
    let sma_values = sma(data, period)?;
    let mut upper = Array1::zeros(sma_values.len());
    let mut lower = Array1::zeros(sma_values.len());

    for i in 0..sma_values.len() {
        let slice = data.slice(s![i..i + period]);
        let mean = sma_values[i];
        let variance: f64 = slice.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / period as f64;
        let std_dev = variance.sqrt();
        let std_mult = num_std * std_dev;

        upper[i] = mean + std_mult;
        lower[i] = mean - std_mult;
    }

    Ok((upper, sma_values, lower))
}

/// Stochastic Oscillator
pub fn stochastic(
    high: ArrayView1<f64>,
    low: ArrayView1<f64>,
    close: ArrayView1<f64>,
    period: usize,
) -> Result<Array1<f64>, IndicatorError> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(IndicatorError::InsufficientData {
            required: high.len(),
            actual: low.len(),
        });
    }
    if high.len() < period {
        return Err(IndicatorError::InsufficientData {
            required: period,
            actual: high.len(),
        });
    }

    let mut result = Array1::zeros(high.len() - period + 1);
    for i in 0..result.len() {
        let high_slice = high.slice(s![i..i + period]);
        let low_slice = low.slice(s![i..i + period]);
        let highest = high_slice.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let lowest = low_slice.iter().copied().fold(f64::INFINITY, f64::min);
        let close_val = close[i + period - 1];

        if highest == lowest {
            result[i] = 50.0;
        } else {
            result[i] = 100.0 * (close_val - lowest) / (highest - lowest);
        }
    }
    Ok(result)
}

/// Average True Range (ATR)
pub fn atr(
    high: ArrayView1<f64>,
    low: ArrayView1<f64>,
    close: ArrayView1<f64>,
    period: usize,
) -> Result<Array1<f64>, IndicatorError> {
    if high.len() != low.len() || high.len() != close.len() {
        return Err(IndicatorError::InsufficientData {
            required: high.len(),
            actual: low.len(),
        });
    }
    if high.len() < period + 1 {
        return Err(IndicatorError::InsufficientData {
            required: period + 1,
            actual: high.len(),
        });
    }

    let mut true_ranges = Vec::new();
    for i in 1..high.len() {
        let tr1 = high[i] - low[i];
        let tr2 = (high[i] - close[i - 1]).abs();
        let tr3 = (low[i] - close[i - 1]).abs();
        true_ranges.push(tr1.max(tr2).max(tr3));
    }

    let tr_array = Array1::from_vec(true_ranges);
    ema(tr_array.view(), period)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_sma() {
        let data = array![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = sma(data.view(), 3).unwrap();
        assert_eq!(result.len(), 3);
        assert!((result[0] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_ema() {
        let data = array![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = ema(data.view(), 3).unwrap();
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_rsi() {
        let data = array![44.0, 44.34, 44.09, 44.15, 43.61, 44.33];
        let result = rsi(data.view(), 2).unwrap();
        assert!(result.len() > 0);
    }
}

