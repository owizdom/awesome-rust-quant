# Excel Integration

Read and write Excel files for financial data analysis.

## Usage

```rust
use rust_quant_excel::{ExcelReader, ExcelWriter};

// Read from Excel
let data = ExcelReader::read_sheet("data.xlsx", "Sheet1")?;

// Write to Excel
let data = vec![
    vec![100.0, 101.0, 99.0],
    vec![102.0, 103.0, 98.0],
];
ExcelWriter::write("output.xlsx", "Prices", &data)?;

// Write with headers
let headers = vec!["Open", "High", "Low", "Close"];
ExcelWriter::write_with_headers("output.xlsx", "OHLC", &headers, &data)?;
```

## Features

- Read Excel files (XLSX)
- Write Excel files with data
- Support for headers
- Multiple sheet support

