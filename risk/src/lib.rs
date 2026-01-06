//! Risk analysis tools
//!
//! This module provides risk metrics: VaR, CVaR, Sharpe ratio,
//! Sortino ratio, maximum drawdown, etc.

use ndarray::{Array1, ArrayView1};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RiskError {
    #[error("Insufficient data: need at least {required} points, got {actual}")]
    InsufficientData { required: usize, actual: usize },
    #[error("Invalid confidence level: {0} (must be between 0 and 1)")]
    InvalidConfidence(f64),
}

/// Calculate Value at Risk (VaR) using historical simulation
pub fn var_historical(
    returns: ArrayView1<f64>,
    confidence: f64,
) -> Result<f64, RiskError> {
    if confidence <= 0.0 || confidence >= 1.0 {
        return Err(RiskError::InvalidConfidence(confidence));
    }
    if returns.is_empty() {
        return Err(RiskError::InsufficientData {
            required: 1,
            actual: 0,
        });
    }

    let mut sorted_returns: Vec<f64> = returns.to_vec();
    sorted_returns.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let index = ((1.0 - confidence) * sorted_returns.len() as f64) as usize;
    let index = index.min(sorted_returns.len() - 1);

    Ok(-sorted_returns[index]) // Negative because VaR is typically reported as positive
}

/// Calculate Conditional Value at Risk (CVaR) / Expected Shortfall
pub fn cvar_historical(
    returns: ArrayView1<f64>,
    confidence: f64,
) -> Result<f64, RiskError> {
    if confidence <= 0.0 || confidence >= 1.0 {
        return Err(RiskError::InvalidConfidence(confidence));
    }
    if returns.is_empty() {
        return Err(RiskError::InsufficientData {
            required: 1,
            actual: 0,
        });
    }

    let var = var_historical(returns, confidence)?;
    let mut tail_returns = Vec::new();

    for &ret in returns.iter() {
        if ret <= -var {
            tail_returns.push(-ret);
        }
    }

    if tail_returns.is_empty() {
        return Ok(var);
    }

    let avg_tail_loss: f64 = tail_returns.iter().sum::<f64>() / tail_returns.len() as f64;
    Ok(avg_tail_loss)
}

/// Calculate Sharpe ratio
pub fn sharpe_ratio(
    returns: ArrayView1<f64>,
    risk_free_rate: f64,
) -> Result<f64, RiskError> {
    if returns.is_empty() {
        return Err(RiskError::InsufficientData {
            required: 1,
            actual: 0,
        });
    }

    let mean_return: f64 = returns.iter().sum::<f64>() / returns.len() as f64;
    let excess_return = mean_return - risk_free_rate;

    let variance: f64 = returns
        .iter()
        .map(|&r| (r - mean_return).powi(2))
        .sum::<f64>()
        / returns.len() as f64;
    let std_dev = variance.sqrt();

    if std_dev == 0.0 {
        return Ok(0.0);
    }

    Ok(excess_return / std_dev)
}

/// Calculate Sortino ratio (uses downside deviation)
pub fn sortino_ratio(
    returns: ArrayView1<f64>,
    risk_free_rate: f64,
) -> Result<f64, RiskError> {
    if returns.is_empty() {
        return Err(RiskError::InsufficientData {
            required: 1,
            actual: 0,
        });
    }

    let mean_return: f64 = returns.iter().sum::<f64>() / returns.len() as f64;
    let excess_return = mean_return - risk_free_rate;

    // Calculate downside deviation (only negative returns)
    let downside_variance: f64 = returns
        .iter()
        .map(|&r| {
            let diff = r - risk_free_rate;
            if diff < 0.0 {
                diff.powi(2)
            } else {
                0.0
            }
        })
        .sum::<f64>()
        / returns.len() as f64;
    let downside_dev = downside_variance.sqrt();

    if downside_dev == 0.0 {
        return Ok(0.0);
    }

    Ok(excess_return / downside_dev)
}

/// Calculate maximum drawdown
pub fn max_drawdown(prices: ArrayView1<f64>) -> Result<f64, RiskError> {
    if prices.is_empty() {
        return Err(RiskError::InsufficientData {
            required: 1,
            actual: 0,
        });
    }

    let mut max_price = prices[0];
    let mut max_dd = 0.0;

    for &price in prices.iter() {
        if price > max_price {
            max_price = price;
        }
        let drawdown = (max_price - price) / max_price;
        if drawdown > max_dd {
            max_dd = drawdown;
        }
    }

    Ok(max_dd)
}

/// Calculate beta (relative to market)
pub fn beta(
    asset_returns: ArrayView1<f64>,
    market_returns: ArrayView1<f64>,
) -> Result<f64, RiskError> {
    if asset_returns.len() != market_returns.len() {
        return Err(RiskError::InsufficientData {
            required: asset_returns.len(),
            actual: market_returns.len(),
        });
    }

    let asset_mean: f64 = asset_returns.iter().sum::<f64>() / asset_returns.len() as f64;
    let market_mean: f64 = market_returns.iter().sum::<f64>() / market_returns.len() as f64;

    let covariance: f64 = asset_returns
        .iter()
        .zip(market_returns.iter())
        .map(|(&a, &m)| (a - asset_mean) * (m - market_mean))
        .sum::<f64>()
        / asset_returns.len() as f64;

    let market_variance: f64 = market_returns
        .iter()
        .map(|&m| (m - market_mean).powi(2))
        .sum::<f64>()
        / market_returns.len() as f64;

    if market_variance == 0.0 {
        return Ok(0.0);
    }

    Ok(covariance / market_variance)
}

/// Calculate alpha (risk-adjusted excess return)
pub fn alpha(
    asset_returns: ArrayView1<f64>,
    market_returns: ArrayView1<f64>,
    risk_free_rate: f64,
) -> Result<f64, RiskError> {
    let beta_val = beta(asset_returns, market_returns)?;
    let asset_mean: f64 = asset_returns.iter().sum::<f64>() / asset_returns.len() as f64;
    let market_mean: f64 = market_returns.iter().sum::<f64>() / market_returns.len() as f64;

    Ok(asset_mean - (risk_free_rate + beta_val * (market_mean - risk_free_rate)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_var() {
        let returns = array![-0.01, -0.02, 0.01, 0.02, -0.01];
        let var = var_historical(returns.view(), 0.95).unwrap();
        assert!(var > 0.0);
    }

    #[test]
    fn test_sharpe_ratio() {
        let returns = array![0.01, 0.02, -0.01, 0.01, 0.02];
        let sharpe = sharpe_ratio(returns.view(), 0.0).unwrap();
        assert!(sharpe.is_finite());
    }

    #[test]
    fn test_max_drawdown() {
        let prices = array![100.0, 110.0, 105.0, 120.0, 100.0];
        let dd = max_drawdown(prices.view()).unwrap();
        assert!(dd > 0.0 && dd <= 1.0);
    }
}

