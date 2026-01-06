//! Statistical functions for quantitative finance
//!
//! This module provides common statistical calculations used in finance:
//! mean, standard deviation, correlation, covariance, etc.

use ndarray::{Array1, ArrayView1};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StatisticsError {
    #[error("Empty data: cannot compute statistics on empty array")]
    EmptyData,
    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
}

/// Calculate the mean (average) of a data series
pub fn mean(data: ArrayView1<f64>) -> Result<f64, StatisticsError> {
    if data.is_empty() {
        return Err(StatisticsError::EmptyData);
    }
    Ok(data.sum() / data.len() as f64)
}

/// Calculate the variance of a data series
pub fn variance(data: ArrayView1<f64>) -> Result<f64, StatisticsError> {
    if data.is_empty() {
        return Err(StatisticsError::EmptyData);
    }
    let m = mean(data)?;
    let n = data.len() as f64;
    let sum_sq_diff: f64 = data.iter().map(|&x| (x - m).powi(2)).sum();
    Ok(sum_sq_diff / n)
}

/// Calculate the population variance (divide by n instead of n-1)
pub fn variance_pop(data: ArrayView1<f64>) -> Result<f64, StatisticsError> {
    variance(data)
}

/// Calculate the sample variance (divide by n-1)
pub fn variance_sample(data: ArrayView1<f64>) -> Result<f64, StatisticsError> {
    if data.is_empty() {
        return Err(StatisticsError::EmptyData);
    }
    if data.len() == 1 {
        return Ok(0.0);
    }
    let m = mean(data)?;
    let n = (data.len() - 1) as f64;
    let sum_sq_diff: f64 = data.iter().map(|&x| (x - m).powi(2)).sum();
    Ok(sum_sq_diff / n)
}

/// Calculate the standard deviation
pub fn std_dev(data: ArrayView1<f64>) -> Result<f64, StatisticsError> {
    variance(data).map(|v| v.sqrt())
}

/// Calculate the sample standard deviation
pub fn std_dev_sample(data: ArrayView1<f64>) -> Result<f64, StatisticsError> {
    variance_sample(data).map(|v| v.sqrt())
}

/// Calculate the covariance between two data series
pub fn covariance(x: ArrayView1<f64>, y: ArrayView1<f64>) -> Result<f64, StatisticsError> {
    if x.len() != y.len() {
        return Err(StatisticsError::DimensionMismatch {
            expected: x.len(),
            actual: y.len(),
        });
    }
    if x.is_empty() {
        return Err(StatisticsError::EmptyData);
    }

    let mean_x = mean(x)?;
    let mean_y = mean(y)?;
    let n = x.len() as f64;

    let sum: f64 = x
        .iter()
        .zip(y.iter())
        .map(|(&xi, &yi)| (xi - mean_x) * (yi - mean_y))
        .sum();

    Ok(sum / n)
}

/// Calculate the sample covariance
pub fn covariance_sample(x: ArrayView1<f64>, y: ArrayView1<f64>) -> Result<f64, StatisticsError> {
    if x.len() != y.len() {
        return Err(StatisticsError::DimensionMismatch {
            expected: x.len(),
            actual: y.len(),
        });
    }
    if x.len() <= 1 {
        return Err(StatisticsError::EmptyData);
    }

    let mean_x = mean(x)?;
    let mean_y = mean(y)?;
    let n = (x.len() - 1) as f64;

    let sum: f64 = x
        .iter()
        .zip(y.iter())
        .map(|(&xi, &yi)| (xi - mean_x) * (yi - mean_y))
        .sum();

    Ok(sum / n)
}

/// Calculate the correlation coefficient (Pearson)
pub fn correlation(x: ArrayView1<f64>, y: ArrayView1<f64>) -> Result<f64, StatisticsError> {
    let cov = covariance(x, y)?;
    let std_x = std_dev(x)?;
    let std_y = std_dev(y)?;

    if std_x == 0.0 || std_y == 0.0 {
        return Ok(0.0);
    }

    Ok(cov / (std_x * std_y))
}

/// Calculate the median of a data series
pub fn median(data: ArrayView1<f64>) -> Result<f64, StatisticsError> {
    if data.is_empty() {
        return Err(StatisticsError::EmptyData);
    }

    let mut sorted: Vec<f64> = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        Ok((sorted[mid - 1] + sorted[mid]) / 2.0)
    } else {
        Ok(sorted[mid])
    }
}

/// Calculate the minimum value
pub fn min(data: ArrayView1<f64>) -> Result<f64, StatisticsError> {
    data.iter()
        .copied()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .ok_or(StatisticsError::EmptyData)
}

/// Calculate the maximum value
pub fn max(data: ArrayView1<f64>) -> Result<f64, StatisticsError> {
    data.iter()
        .copied()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .ok_or(StatisticsError::EmptyData)
}

/// Calculate the skewness (third moment)
pub fn skewness(data: ArrayView1<f64>) -> Result<f64, StatisticsError> {
    if data.is_empty() {
        return Err(StatisticsError::EmptyData);
    }
    let m = mean(data)?;
    let std = std_dev(data)?;
    if std == 0.0 {
        return Ok(0.0);
    }

    let n = data.len() as f64;
    let sum: f64 = data.iter().map(|&x| ((x - m) / std).powi(3)).sum();
    Ok(sum / n)
}

/// Calculate the kurtosis (fourth moment)
pub fn kurtosis(data: ArrayView1<f64>) -> Result<f64, StatisticsError> {
    if data.is_empty() {
        return Err(StatisticsError::EmptyData);
    }
    let m = mean(data)?;
    let std = std_dev(data)?;
    if std == 0.0 {
        return Ok(0.0);
    }

    let n = data.len() as f64;
    let sum: f64 = data.iter().map(|&x| ((x - m) / std).powi(4)).sum();
    Ok(sum / n)
}

/// Calculate percentiles
pub fn percentile(data: ArrayView1<f64>, p: f64) -> Result<f64, StatisticsError> {
    if p < 0.0 || p > 1.0 {
        return Err(StatisticsError::EmptyData); // Invalid percentile
    }
    if data.is_empty() {
        return Err(StatisticsError::EmptyData);
    }

    let mut sorted: Vec<f64> = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let index = p * (sorted.len() - 1) as f64;
    let lower = index.floor() as usize;
    let upper = index.ceil() as usize;
    let weight = index - lower as f64;

    if lower == upper {
        Ok(sorted[lower])
    } else {
        Ok(sorted[lower] * (1.0 - weight) + sorted[upper] * weight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_mean() {
        let data = array![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(mean(data.view()).unwrap(), 3.0);
    }

    #[test]
    fn test_variance() {
        let data = array![1.0, 2.0, 3.0, 4.0, 5.0];
        let var = variance(data.view()).unwrap();
        assert!((var - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_correlation() {
        let x = array![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = array![2.0, 4.0, 6.0, 8.0, 10.0];
        let corr = correlation(x.view(), y.view()).unwrap();
        assert!((corr - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_median() {
        let data = array![1.0, 3.0, 2.0, 5.0, 4.0];
        assert_eq!(median(data.view()).unwrap(), 3.0);
    }
}

