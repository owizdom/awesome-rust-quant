# Machine Learning

Basic machine learning utilities for quantitative finance.

## Usage

```rust
use rust_quant_machine_learning::{LinearRegression, KMeans};
use ndarray::array;

// Linear Regression
let mut model = LinearRegression::new();
let X = array![[1.0, 2.0], [2.0, 3.0], [3.0, 4.0]];
let y = array![3.0, 5.0, 7.0];
model.fit(X.view(), y.view())?;
let predictions = model.predict(X.view());

// K-Means Clustering
let mut kmeans = KMeans::new(3);
kmeans.fit(X.view(), 100)?;
let clusters = kmeans.predict(X.view())?;
```

## Features

- Linear Regression (OLS)
- K-Means Clustering
- Basic classification utilities

