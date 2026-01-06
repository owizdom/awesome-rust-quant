//! Charting and plotting utilities
//!
//! This module provides basic charting capabilities for visualizing
//! financial data.

use ndarray::ArrayView1;
use plotters::prelude::*;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChartError {
    #[error("IO error: {0}")]
    IoError(String),
    #[error("Empty data")]
    EmptyData,
}

/// Plot a line chart
pub fn plot_line<P: AsRef<Path>>(
    path: P,
    title: &str,
    x_label: &str,
    y_label: &str,
    data: ArrayView1<f64>,
) -> Result<(), ChartError> {
    if data.is_empty() {
        return Err(ChartError::EmptyData);
    }

    let root = BitMapBackend::new(path.as_ref(), (800, 600))
        .into_drawing_area();
    root.fill(&WHITE)
        .map_err(|e| ChartError::IoError(e.to_string()))?;

    let min_val = data.iter().copied().fold(f64::INFINITY, f64::min);
    let max_val = data.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 40).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0..data.len(), min_val..max_val)
        .map_err(|e| ChartError::IoError(e.to_string()))?;

    chart
        .configure_mesh()
        .x_desc(x_label)
        .y_desc(y_label)
        .draw()
        .map_err(|e| ChartError::IoError(e.to_string()))?;

    chart
        .draw_series(LineSeries::new(
            (0..).zip(data.iter()).map(|(x, &y)| (x, y)),
            &RED,
        ))
        .map_err(|e| ChartError::IoError(e.to_string()))?
        .label("Data")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .map_err(|e| ChartError::IoError(e.to_string()))?;

    root.present().map_err(|e| ChartError::IoError(e.to_string()))?;
    Ok(())
}

/// Plot OHLC candlestick chart (simplified)
pub fn plot_candlestick<P: AsRef<Path>>(
    path: P,
    title: &str,
    opens: ArrayView1<f64>,
    highs: ArrayView1<f64>,
    lows: ArrayView1<f64>,
    closes: ArrayView1<f64>,
) -> Result<(), ChartError> {
    if opens.is_empty() || opens.len() != highs.len() || opens.len() != lows.len() || opens.len() != closes.len() {
        return Err(ChartError::EmptyData);
    }

    // For simplicity, plot as line chart
    // In production, implement proper candlestick rendering
    plot_line(path, title, "Time", "Price", closes)
}

/// Plot multiple series
pub fn plot_multiple<P: AsRef<Path>>(
    path: P,
    title: &str,
    x_label: &str,
    y_label: &str,
    series: &[(&str, ArrayView1<f64>)],
) -> Result<(), ChartError> {
    if series.is_empty() {
        return Err(ChartError::EmptyData);
    }

    let all_data: Vec<f64> = series.iter().flat_map(|(_, data)| data.iter().copied()).collect();
    let min_val = all_data.iter().copied().fold(f64::INFINITY, f64::min);
    let max_val = all_data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let max_len = series.iter().map(|(_, data)| data.len()).max().unwrap_or(0);

    let root = BitMapBackend::new(path.as_ref(), (800, 600))
        .into_drawing_area();
    root.fill(&WHITE)
        .map_err(|e| ChartError::IoError(e.to_string()))?;

    let colors = vec![&RED, &BLUE, &GREEN, &MAGENTA, &CYAN];

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 40).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0..max_len, min_val..max_val)
        .map_err(|e| ChartError::IoError(e.to_string()))?;

    chart
        .configure_mesh()
        .x_desc(x_label)
        .y_desc(y_label)
        .draw()
        .map_err(|e| ChartError::IoError(e.to_string()))?;

    for (idx, (name, data)) in series.iter().enumerate() {
        let color = colors[idx % colors.len()];
        chart
            .draw_series(LineSeries::new(
                (0..).zip(data.iter()).map(|(x, &y)| (x, y)),
                color,
            ))
            .map_err(|e| ChartError::IoError(e.to_string()))?
            .label(*name)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .map_err(|e| ChartError::IoError(e.to_string()))?;

    root.present().map_err(|e| ChartError::IoError(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_plot_line() {
        let data = array![1.0, 2.0, 3.0, 4.0, 5.0];
        let _ = plot_line("/tmp/test.png", "Test", "X", "Y", data.view());
    }
}

