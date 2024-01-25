# crabby-ml

[![crates.io](https://img.shields.io/crates/v/crabby-ml.svg)](https://crates.io/crates/crabby-ml)
[![docs.rs](https://docs.rs/crabby-ml/badge.svg)](https://docs.rs/crabby-ml)
[![Build Status](https://travis-ci.com/andrew-johnson-4/crabby-ml.svg?branch=master)](https://travis-ci.com/andrew-johnson-4/crabby-ml)
[![codecov](https://codecov.io/gh/andrew-johnson-4/crabby-ml/branch/master/graph/badge.svg)](https://codecov.io/gh/andrew-johnson-4/crabby-ml)

## CURRENT DEV PLAN:
```text
crabby-ml/
│
├── Cargo.toml                    # Project dependencies and metadata
├── Cargo.lock                    # Auto-generated lock file for dependencies
│
├── src/
│   ├── lib.rs                    # Entry point for the library
│   │
│   │── crabby/                   # Core builder struct for the library (Crabby) and its implementation
│   │   ├── mod.rs                # Module declaration and common code
│   │   └── crabby.rs             # Core `Crabby` builder struct and its implementation
│   │
│   ├── data/                     # Data handling using Polars
│   │   ├── mod.rs                # Module declaration and common code
│   │   ├── data.rs               # Data struct and its implementation
│   │   └── polars_integration.rs # Integration and utilities for Polars
│   │
│   ├── loss_fn/                  # Loss functions to be used in optimization
│   │   ├── mod.rs                # Module declaration and common code
│   │   ├── mse.rs                # Mean Squared Error/L2 loss function
│   │   ├── mae.rs                # Mean Absolute Error/L1 loss function
│   │   ├── elastic.rs            # Elastic net loss function
│   │   ├── log.rs                # Log loss function
│   │   ├── cross_entropy.rs      # Categorical Cross-Entropy loss function
│   │   ├── bce.rs                # Categorical Binary Cross-Entropy loss function
│   │   ├── hinge.rs              # Hinge loss function
│   │   └── kl_div.rs             # Kullback-Leibler Divergence loss function
│   │
│   ├── optim/                    # Optimizers for gradient descent and/or other algorithms
│   │   ├── mod.rs                # Module declaration and common code
│   │   ├── optim.rs              # Base trait for optimizers. All optimizers must implement this trait.
│   │   ├── gradient_descent.rs   # Standard gradient descent implementation
│   │   ├── sgd.rs                # Stochastic gradient descent implementation
│   │   ├── minibatch_gd.rs       # Mini-batch gradient descent implementation
│   │   └── minibatch_sgd.rs      # Mini-batch SGD implementation
│   │
│   ├── utils/                    # Utility functions and common helpers
│   │   └── mod.rs                # Module declaration and common utilities
│   │
│   └── config/                   # Configuration structures and builders
│       ├── mod.rs                # Module declaration and common code
│       └── builder.rs            # Builder pattern implementation for configurations
│
├── tests/                        # Integration and unit tests
│   ├── gd_tests.rs               # Tests for standard gradient descent
│   ├── sgd_tests.rs              # Tests for stochastic gradient descent
│   ├── minibatch_gd_tests.rs     # Tests for mini-batch gradient descent
│   └── minibatch_sgd_tests.rs    # Tests for mini-batch SGD
│
├── examples/                     # Example scripts demonstrating usage
│   └── basic_usage.rs            # Basic usage of the gradient descent implementation
│
└── README.md                     # Project README with an overview, setup, and usage instructions
```