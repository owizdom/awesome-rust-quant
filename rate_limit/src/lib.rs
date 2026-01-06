//! Rate limiting for API calls and trading operations
//!
//! This module provides token bucket rate limiting to control
//! the rate of API requests and trading operations.

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RateLimitError {
    #[error("Rate limit exceeded: {0} requests per {1:?}")]
    RateLimitExceeded(u32, Duration),
}

/// Token bucket rate limiter
pub struct RateLimiter {
    capacity: u32,
    tokens: Arc<Mutex<u32>>,
    refill_rate: f64, // tokens per second
    last_refill: Arc<Mutex<Instant>>,
    window: Duration,
}

impl RateLimiter {
    /// Create a new rate limiter
    ///
    /// # Arguments
    /// * `capacity` - Maximum number of tokens (burst capacity)
    /// * `rate` - Number of requests allowed per `window`
    /// * `window` - Time window for the rate limit
    pub fn new(capacity: u32, rate: u32, window: Duration) -> Self {
        let refill_rate = rate as f64 / window.as_secs_f64();
        Self {
            capacity,
            tokens: Arc::new(Mutex::new(capacity)),
            refill_rate,
            last_refill: Arc::new(Mutex::new(Instant::now())),
            window,
        }
    }

    /// Try to acquire a token (non-blocking)
    pub async fn try_acquire(&self) -> Result<(), RateLimitError> {
        let mut tokens = self.tokens.lock().await;
        let mut last_refill = self.last_refill.lock().await;

        // Refill tokens based on elapsed time
        let now = Instant::now();
        let elapsed = now.duration_since(*last_refill);
        let tokens_to_add = (elapsed.as_secs_f64() * self.refill_rate) as u32;

        if tokens_to_add > 0 {
            *tokens = (*tokens + tokens_to_add).min(self.capacity);
            *last_refill = now;
        }

        if *tokens > 0 {
            *tokens -= 1;
            Ok(())
        } else {
            Err(RateLimitError::RateLimitExceeded(
                (self.refill_rate * self.window.as_secs_f64()) as u32,
                self.window,
            ))
        }
    }

    /// Acquire a token (blocking until available)
    pub async fn acquire(&self) {
        loop {
            match self.try_acquire().await {
                Ok(()) => break,
                Err(_) => {
                    // Calculate wait time
                    let wait_time = Duration::from_secs_f64(1.0 / self.refill_rate);
                    tokio::time::sleep(wait_time).await;
                }
            }
        }
    }

    /// Get current number of available tokens
    pub async fn available(&self) -> u32 {
        let mut tokens = self.tokens.lock().await;
        let mut last_refill = self.last_refill.lock().await;

        // Refill tokens
        let now = Instant::now();
        let elapsed = now.duration_since(*last_refill);
        let tokens_to_add = (elapsed.as_secs_f64() * self.refill_rate) as u32;

        if tokens_to_add > 0 {
            *tokens = (*tokens + tokens_to_add).min(self.capacity);
            *last_refill = now;
        }

        *tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_rate_limiter() {
        let limiter = RateLimiter::new(10, 5, Duration::from_secs(1));

        // Should be able to acquire tokens up to capacity
        for _ in 0..10 {
            assert!(limiter.try_acquire().await.is_ok());
        }

        // Should fail after capacity is exhausted
        assert!(limiter.try_acquire().await.is_err());
    }

    #[tokio::test]
    async fn test_rate_limiter_refill() {
        let limiter = RateLimiter::new(10, 10, Duration::from_secs(1));

        // Exhaust tokens
        for _ in 0..10 {
            limiter.try_acquire().await.unwrap();
        }

        // Wait for refill
        sleep(Duration::from_millis(1100)).await;

        // Should be able to acquire again
        assert!(limiter.try_acquire().await.is_ok());
    }
}

