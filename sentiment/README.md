# Sentiment Analysis

Sentiment analysis tools for financial news and social media.

## Usage

```rust
use rust_quant_sentiment::SentimentAnalyzer;

let analyzer = SentimentAnalyzer::new();

// Analyze single text
let score = analyzer.analyze("This stock is performing very well")?;
let label = analyzer.label(score);

// Analyze batch of texts
let texts = vec![
    "Great earnings report",
    "Stock price is falling",
];
let avg_sentiment = analyzer.analyze_batch(&texts)?;
```

## Features

- Text sentiment scoring (-1.0 to 1.0)
- Positive/negative/neutral classification
- Intensifier detection (very, extremely, etc.)
- Batch analysis
- Financial domain-specific word lists

