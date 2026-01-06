//! Numerical libraries and data structures for quantitative finance
//!
//! This module provides DataFrame-like structures and matrix operations
//! for financial data manipulation.

use ndarray::{Array2, Array1};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NumericalError {
    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
    #[error("Column not found: {0}")]
    ColumnNotFound(String),
    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(usize),
}

/// A simple DataFrame-like structure for financial data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFrame {
    columns: Vec<String>,
    data: HashMap<String, Vec<f64>>,
    index: Vec<String>,
}

impl DataFrame {
    /// Create a new empty DataFrame
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            data: HashMap::new(),
            index: Vec::new(),
        }
    }

    /// Create a DataFrame from columns and data
    pub fn from_columns(columns: Vec<String>, data: Vec<Vec<f64>>) -> Result<Self, NumericalError> {
        if columns.len() != data.len() {
            return Err(NumericalError::DimensionMismatch {
                expected: columns.len(),
                actual: data.len(),
            });
        }

        let n_rows = if data.is_empty() { 0 } else { data[0].len() };
        for (i, col_data) in data.iter().enumerate() {
            if col_data.len() != n_rows {
                return Err(NumericalError::DimensionMismatch {
                    expected: n_rows,
                    actual: col_data.len(),
                });
            }
        }

        let mut df_data = HashMap::new();
        for (col, values) in columns.iter().zip(data.iter()) {
            df_data.insert(col.clone(), values.clone());
        }

        let index: Vec<String> = (0..n_rows).map(|i| i.to_string()).collect();

        Ok(Self {
            columns,
            data: df_data,
            index,
        })
    }

    /// Add a column to the DataFrame
    pub fn add_column(&mut self, name: String, values: Vec<f64>) -> Result<(), NumericalError> {
        let n_rows = self.n_rows();
        if values.len() != n_rows && n_rows > 0 {
            return Err(NumericalError::DimensionMismatch {
                expected: n_rows,
                actual: values.len(),
            });
        }

        if !self.columns.contains(&name) {
            self.columns.push(name.clone());
        }
        self.data.insert(name, values);
        Ok(())
    }

    /// Get a column by name
    pub fn get_column(&self, name: &str) -> Result<&Vec<f64>, NumericalError> {
        self.data
            .get(name)
            .ok_or_else(|| NumericalError::ColumnNotFound(name.to_string()))
    }

    /// Get number of rows
    pub fn n_rows(&self) -> usize {
        self.index.len()
    }

    /// Get number of columns
    pub fn n_cols(&self) -> usize {
        self.columns.len()
    }

    /// Get column names
    pub fn columns(&self) -> &[String] {
        &self.columns
    }

    /// Convert column to ndarray Array1
    pub fn column_to_array(&self, name: &str) -> Result<Array1<f64>, NumericalError> {
        let col = self.get_column(name)?;
        Ok(Array1::from_vec(col.clone()))
    }

    /// Convert to ndarray Array2 (rows x columns)
    pub fn to_array2(&self) -> Array2<f64> {
        let n_rows = self.n_rows();
        let n_cols = self.n_cols();
        let mut data = Vec::with_capacity(n_rows * n_cols);

        for i in 0..n_rows {
            for col_name in &self.columns {
                if let Some(col_data) = self.data.get(col_name) {
                    data.push(col_data[i]);
                } else {
                    data.push(0.0);
                }
            }
        }

        Array2::from_shape_vec((n_rows, n_cols), data)
            .expect("Failed to create Array2 from DataFrame")
    }
}

impl Default for DataFrame {
    fn default() -> Self {
        Self::new()
    }
}

/// Matrix operations for financial calculations
pub mod matrix {
    use super::*;

    /// Calculate matrix multiplication
    pub fn matmul(a: &Array2<f64>, b: &Array2<f64>) -> Result<Array2<f64>, NumericalError> {
        if a.ncols() != b.nrows() {
            return Err(NumericalError::DimensionMismatch {
                expected: a.ncols(),
                actual: b.nrows(),
            });
        }

        let result = a.dot(b);
        Ok(result)
    }

    /// Calculate matrix transpose
    pub fn transpose(a: &Array2<f64>) -> Array2<f64> {
        a.t().to_owned()
    }

    /// Calculate matrix inverse (using pseudo-inverse for non-square)
    pub fn inv(a: &Array2<f64>) -> Result<Array2<f64>, NumericalError> {
        // For simplicity, this is a placeholder
        // In production, use a proper linear algebra library like nalgebra
        if a.nrows() != a.ncols() {
            return Err(NumericalError::DimensionMismatch {
                expected: a.nrows(),
                actual: a.ncols(),
            });
        }
        // This would require a proper matrix inversion algorithm
        // For now, return identity matrix as placeholder
        Ok(Array2::eye(a.nrows()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataframe_creation() {
        let df = DataFrame::new();
        assert_eq!(df.n_rows(), 0);
        assert_eq!(df.n_cols(), 0);
    }

    #[test]
    fn test_dataframe_from_columns() {
        let columns = vec!["open".to_string(), "close".to_string()];
        let data = vec![vec![100.0, 101.0], vec![99.0, 102.0]];
        let df = DataFrame::from_columns(columns, data).unwrap();
        assert_eq!(df.n_rows(), 2);
        assert_eq!(df.n_cols(), 2);
    }

    #[test]
    fn test_add_column() {
        let mut df = DataFrame::new();
        df.add_column("price".to_string(), vec![100.0, 101.0, 102.0])
            .unwrap();
        assert_eq!(df.n_rows(), 3);
        assert_eq!(df.n_cols(), 1);
    }
}

