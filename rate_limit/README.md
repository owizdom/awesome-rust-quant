# Rate Limiting

Token bucket rate limiter for API calls and trading operations.

## Usage

```rust
use rust_quant_rate_limit::RateLimiter;
use std::time::Duration;

// Create a rate limiter: 100 requests per second, burst capacity of 10
let limiter = RateLimiter::new(10, 100, Duration::from_secs(1));

// Try to acquire a token (non-blocking)
match limiter.try_acquire().await {
    Ok(()) => {
        // Make API call
    }
    Err(e) => {
        // Rate limit exceeded
    }
}

// Acquire a token (blocking until available)
limiter.acquire().await;
// Make API call
```

## Features

- Token bucket algorithm
- Configurable rate and burst capacity
- Non-blocking and blocking acquisition
- Async/await support

