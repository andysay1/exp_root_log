# ExpRoot+Log: A Linear and Universal Basis for Function Approximation

ExpRoot+Log is a fast and interpretable function approximation method based on a hybrid linear basis. It combines exponential square-root, polynomial, and logarithmic terms to efficiently approximate a wide range of functions, including smooth, discontinuous, and decaying ones.

## Features

-   **Fast and accurate**: Uses a minimal set of basis functions for efficient function approximation.
-   **Interpretable**: Each term in the basis has a clear mathematical interpretation.
-   **Flexible**: Can handle smooth, discontinuous, and asymptotically decaying functions.
-   **Linear regression**: Uses standard least-squares fitting for optimal performance.

## Usage

### Add the dependency to `Cargo.toml`:

```toml
[dependencies]
exp_root_log = "0.1.0"
```

---

## 📂 `examples/demo.rs`:

```rust
use exp_root_log::approx_exp_root_log;

fn main() {
    // Generate test data
    let x: Vec<f64> = (0..100).map(|i| i as f64 / 100.0).collect();
    let y: Vec<f64> = x.iter().map(|&x| (2.0 * std::f64::consts::PI * x).sin()).collect();

    // Create the approximation function using ExpRoot+Log
    let approx_fn = approx_exp_root_log(
        &x,
        &y,
        &[0.5, 2.0, 5.0, 10.0, 20.0],    //  b_i
        5,                                  //  x^5
        &[1.0, 5.0, 10.0, 20.0],           // log params
    );


    // Evaluate the approximation
    let y_pred: Vec<f64> = x.iter().map(|&xi| approx_fn(xi)).collect();

    // Print the result
    println!("Approximated values: {:?}", y_pred);
}
```
