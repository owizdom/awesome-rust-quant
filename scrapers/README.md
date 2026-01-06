# Scrapers

Web scraping utilities for financial data.

## Usage

```rust
use rust_quant_scrapers::FinancialScraper;

let scraper = FinancialScraper::new();

// Fetch HTML
let html = scraper.fetch_html("https://example.com/stock").await?;

// Extract text by CSS selector
let prices = scraper.extract_text(&html, ".price")?;

// Scrape price
let price = scraper.scrape_price("https://example.com/stock", ".price").await?;

// Scrape table data
let table = scraper.scrape_table(&html, "table")?;
```

## Features

- HTML fetching
- CSS selector-based extraction
- Table scraping
- Price extraction
- Attribute extraction

