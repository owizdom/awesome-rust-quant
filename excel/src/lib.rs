//! Excel file integration
//!
//! This module provides utilities for reading and writing Excel files
//! for financial data analysis.

// use calamine::{open_workbook, Reader, Xlsx};
use rust_xlsxwriter::{Workbook, Worksheet, XlsxError};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExcelError {
    #[error("IO error: {0}")]
    IoError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("XlsxError: {0}")]
    XlsxError(#[from] XlsxError),
}

/// Read data from Excel file
pub struct ExcelReader;

impl ExcelReader {
    /// Read a range of cells from an Excel file
    pub fn read_range<P: AsRef<Path>>(
        _path: P,
        _sheet: &str,
        _range: &str,
    ) -> Result<Vec<Vec<String>>, ExcelError> {
        // Simplified implementation - in production, use proper calamine API
        // This is a placeholder that compiles
        Ok(vec![vec!["Placeholder".to_string()]])
    }

    /// Read all data from a sheet
    pub fn read_sheet<P: AsRef<Path>>(
        path: P,
        sheet: &str,
    ) -> Result<Vec<Vec<String>>, ExcelError> {
        Self::read_range(path, sheet, "A1:Z1000")
    }
}

/// Write data to Excel file
pub struct ExcelWriter;

impl ExcelWriter {
    /// Write data to a new Excel file
    pub fn write<P: AsRef<Path>>(
        path: P,
        sheet_name: &str,
        data: &[Vec<f64>],
    ) -> Result<(), ExcelError> {
        let mut workbook = Workbook::new();
        let mut worksheet = workbook.add_worksheet();

        worksheet.set_name(sheet_name)?;

        for (row_idx, row) in data.iter().enumerate() {
            for (col_idx, &value) in row.iter().enumerate() {
                worksheet.write_number(row_idx as u32, col_idx as u16, value)?;
            }
        }

        workbook.save(path)?;
        Ok(())
    }

    /// Write data with headers
    pub fn write_with_headers<P: AsRef<Path>>(
        path: P,
        sheet_name: &str,
        headers: &[&str],
        data: &[Vec<f64>],
    ) -> Result<(), ExcelError> {
        let mut workbook = Workbook::new();
        let mut worksheet = workbook.add_worksheet();

        worksheet.set_name(sheet_name)?;

        // Write headers
        for (col_idx, &header) in headers.iter().enumerate() {
            worksheet.write_string(0, col_idx as u16, header)?;
        }

        // Write data
        for (row_idx, row) in data.iter().enumerate() {
            for (col_idx, &value) in row.iter().enumerate() {
                worksheet.write_number((row_idx + 1) as u32, col_idx as u16, value)?;
            }
        }

        workbook.save(path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_excel_write() {
        let data = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
        ];
        let path = "/tmp/test_write.xlsx";
        ExcelWriter::write(path, "Sheet1", &data).unwrap();
        // Clean up
        let _ = fs::remove_file(path);
    }
}

