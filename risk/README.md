# Risk Analysis

Risk metrics and analysis tools for quantitative finance.

## Usage

```rust
use rust_quant_risk::*;
use ndarray::array;

let returns = array![0.01, -0.02, 0.03, -0.01, 0.02];

// Value at Risk
let var_95 = var_historical(returns.view(), 0.95)?;

// Conditional VaR
let cvar_95 = cvar_historical(returns.view(), 0.95)?;

// Sharpe Ratio
let sharpe = sharpe_ratio(returns.view(), 0.0)?;

// Sortino Ratio
let sortino = sortino_ratio(returns.view(), 0.0)?;

// Maximum Drawdown
let prices = array![100.0, 110.0, 105.0, 120.0, 100.0];
let max_dd = max_drawdown(prices.view())?;

// Beta and Alpha
let market_returns = array![0.02, -0.01, 0.02, 0.01, 0.01];
let beta_val = beta(returns.view(), market_returns.view())?;
let alpha_val = alpha(returns.view(), market_returns.view(), 0.0)?;
```

## Features

- Value at Risk (VaR)
- Conditional VaR (CVaR)
- Sharpe Ratio
- Sortino Ratio
- Maximum Drawdown
- Beta and Alpha

