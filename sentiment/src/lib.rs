//! Sentiment analysis for financial news and social media
//!
//! This module provides basic sentiment analysis tools for
//! analyzing financial news and social media sentiment.

use std::collections::HashMap;

#[allow(dead_code)]
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SentimentError {
    #[error("Empty text")]
    EmptyText,
}

/// Sentiment analyzer
pub struct SentimentAnalyzer {
    positive_words: Vec<String>,
    negative_words: Vec<String>,
    intensifiers: HashMap<String, f64>,
}

impl SentimentAnalyzer {
    /// Create a new sentiment analyzer
    pub fn new() -> Self {
        let positive_words = vec![
            "good", "great", "excellent", "positive", "bullish", "up", "rise", "gain",
            "profit", "success", "strong", "growth", "outperform", "buy", "outperform",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let negative_words = vec![
            "bad", "terrible", "poor", "negative", "bearish", "down", "fall", "loss",
            "decline", "weak", "underperform", "sell", "crash", "drop", "plunge",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let mut intensifiers = HashMap::new();
        intensifiers.insert("very".to_string(), 1.5);
        intensifiers.insert("extremely".to_string(), 2.0);
        intensifiers.insert("highly".to_string(), 1.3);
        intensifiers.insert("slightly".to_string(), 0.5);
        intensifiers.insert("somewhat".to_string(), 0.7);

        Self {
            positive_words,
            negative_words,
            intensifiers,
        }
    }

    /// Analyze sentiment of text
    ///
    /// Returns a score between -1.0 (very negative) and 1.0 (very positive)
    pub fn analyze(&self, text: &str) -> Result<f64, SentimentError> {
        if text.trim().is_empty() {
            return Err(SentimentError::EmptyText);
        }

        let lower_text = text.to_lowercase();
        let words: Vec<String> = lower_text
            .split_whitespace()
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .filter(|w| !w.is_empty())
            .collect();

        let mut score = 0.0;
        let mut i = 0;

        while i < words.len() {
            let word = &words[i];
            let mut word_score = 0.0;
            let mut multiplier = 1.0;

            // Check for intensifiers
            if i > 0 {
                if let Some(&intensity) = self.intensifiers.get(&words[i - 1]) {
                    multiplier = intensity;
                }
            }

            // Check for positive words
            if self.positive_words.contains(word) {
                word_score = 1.0;
            }
            // Check for negative words
            else if self.negative_words.contains(word) {
                word_score = -1.0;
            }

            score += word_score * multiplier;
            i += 1;
        }

        // Normalize score
        let normalized = (score / words.len() as f64).clamp(-1.0, 1.0);
        Ok(normalized)
    }

    /// Get sentiment label
    pub fn label(&self, score: f64) -> &'static str {
        if score > 0.3 {
            "positive"
        } else if score < -0.3 {
            "negative"
        } else {
            "neutral"
        }
    }

    /// Analyze multiple texts and return average sentiment
    pub fn analyze_batch(&self, texts: &[&str]) -> Result<f64, SentimentError> {
        if texts.is_empty() {
            return Err(SentimentError::EmptyText);
        }

        let scores: Result<Vec<f64>, _> = texts.iter().map(|t| self.analyze(t)).collect();
        let scores = scores?;
        let avg = scores.iter().sum::<f64>() / scores.len() as f64;
        Ok(avg)
    }
}

impl Default for SentimentAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_sentiment() {
        let analyzer = SentimentAnalyzer::new();
        let score = analyzer.analyze("This stock is performing very well and showing strong growth").unwrap();
        assert!(score > 0.0);
    }

    #[test]
    fn test_negative_sentiment() {
        let analyzer = SentimentAnalyzer::new();
        let score = analyzer.analyze("The market is crashing and prices are falling").unwrap();
        assert!(score < 0.0);
    }

    #[test]
    fn test_neutral_sentiment() {
        let analyzer = SentimentAnalyzer::new();
        let score = analyzer.analyze("The price is stable").unwrap();
        assert!(score.abs() < 0.3);
    }

    #[test]
    fn test_label() {
        let analyzer = SentimentAnalyzer::new();
        assert_eq!(analyzer.label(0.5), "positive");
        assert_eq!(analyzer.label(-0.5), "negative");
        assert_eq!(analyzer.label(0.1), "neutral");
    }
}

