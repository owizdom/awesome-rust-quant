//! Web scraping utilities for financial data
//!
//! This module provides utilities for scraping financial data
//! from websites.

use reqwest::Client;
use scraper::{Html, Selector};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScraperError {
    #[error("HTTP error: {0}")]
    HttpError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Selector error: {0}")]
    SelectorError(String),
}

/// Web scraper for financial data
pub struct FinancialScraper {
    client: Client,
}

impl FinancialScraper {
    /// Create a new scraper
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Fetch HTML from a URL
    pub async fn fetch_html(&self, url: &str) -> Result<String, ScraperError> {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| ScraperError::HttpError(e.to_string()))?;

        let text = response
            .text()
            .await
            .map_err(|e| ScraperError::HttpError(e.to_string()))?;

        Ok(text)
    }

    /// Extract text by CSS selector
    pub fn extract_text(&self, html: &str, selector: &str) -> Result<Vec<String>, ScraperError> {
        let document = Html::parse_document(html);
        let sel = Selector::parse(selector)
            .map_err(|e| ScraperError::SelectorError(e.to_string()))?;

        let mut results = Vec::new();
        for element in document.select(&sel) {
            results.push(element.text().collect::<String>().trim().to_string());
        }

        Ok(results)
    }

    /// Extract attribute values by CSS selector
    pub fn extract_attr(
        &self,
        html: &str,
        selector: &str,
        attr: &str,
    ) -> Result<Vec<String>, ScraperError> {
        let document = Html::parse_document(html);
        let sel = Selector::parse(selector)
            .map_err(|e| ScraperError::SelectorError(e.to_string()))?;

        let mut results = Vec::new();
        for element in document.select(&sel) {
            if let Some(value) = element.value().attr(attr) {
                results.push(value.to_string());
            }
        }

        Ok(results)
    }

    /// Scrape price from a simple HTML structure
    pub async fn scrape_price(&self, url: &str, price_selector: &str) -> Result<f64, ScraperError> {
        let html = self.fetch_html(url).await?;
        let prices = self.extract_text(&html, price_selector)?;

        if prices.is_empty() {
            return Err(ScraperError::ParseError("No price found".to_string()));
        }

        // Try to parse the first price
        let price_str = prices[0]
            .chars()
            .filter(|c| c.is_numeric() || *c == '.' || *c == '-')
            .collect::<String>();

        price_str
            .parse::<f64>()
            .map_err(|e| ScraperError::ParseError(e.to_string()))
    }

    /// Scrape table data
    pub fn scrape_table(&self, html: &str, table_selector: &str) -> Result<Vec<Vec<String>>, ScraperError> {
        let document = Html::parse_document(html);
        let table_sel = Selector::parse(table_selector)
            .map_err(|e| ScraperError::SelectorError(e.to_string()))?;
        let row_sel = Selector::parse("tr")
            .map_err(|e| ScraperError::SelectorError(e.to_string()))?;
        let cell_sel = Selector::parse("td, th")
            .map_err(|e| ScraperError::SelectorError(e.to_string()))?;

        let mut table_data = Vec::new();

        if let Some(table) = document.select(&table_sel).next() {
            for row in table.select(&row_sel) {
                let mut row_data = Vec::new();
                for cell in row.select(&cell_sel) {
                    row_data.push(cell.text().collect::<String>().trim().to_string());
                }
                if !row_data.is_empty() {
                    table_data.push(row_data);
                }
            }
        }

        Ok(table_data)
    }
}

impl Default for FinancialScraper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_text() {
        let html = r#"<div class="price">$100.50</div>"#;
        let scraper = FinancialScraper::new();
        let result = scraper.extract_text(html, ".price").unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_scrape_table() {
        let html = r#"
            <table>
                <tr><th>Symbol</th><th>Price</th></tr>
                <tr><td>AAPL</td><td>150.00</td></tr>
            </table>
        "#;
        let scraper = FinancialScraper::new();
        let table = scraper.scrape_table(html, "table").unwrap();
        assert_eq!(table.len(), 2);
    }
}

