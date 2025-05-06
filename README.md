# ExpRoot+Log: A Linear and Universal Basis for Function Approximation

ExpRoot+Log is a fast and interpretable function approximation method based on a hybrid linear basis. It combines exponential square-root, polynomial, and logarithmic terms to efficiently approximate a wide range of functions, including smooth, discontinuous, and decaying ones.

## Features

-   **Fast and accurate**: Uses a minimal set of basis functions for efficient function approximation.
-   **Interpretable**: Each term in the basis has a clear mathematical interpretation.
-   **Flexible**: Can handle smooth, discontinuous, and asymptotically decaying functions.
-   **Linear regression**: Uses standard least-squares fitting for optimal performance.

---

## Learn more

The full write‑up (motivation, math derivation, and numeric experiments) is available on dev.to:

👉 **ExpRoot + Log: A Linear and Universal Basis for Function Approximation**  
<https://dev.to/andysay/exprootlog-a-linear-and-universal-basis-for-function-approximation-2e9d>

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

## Benchmark

| Function   | ExpRoot + Log<br>▪ MSE | Polynomial deg 10<br>▪ MSE | Take‑away                                                                                                   |
| ---------- | ---------------------- | -------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `Sin`      | **3.67 × 10⁻⁸**        | 1.34 × 10⁻¹¹               | Poly‑10 is a hair better on a pure sine; ExpRoot + Log is still < 10⁻⁷.                                     |
| `ExpDecay` | **1.46 × 10⁻¹³**       | 1.14 × 10⁻¹⁵               | Both are essentially machine‑precision; ExpRoot + Log keeps up.                                             |
| `Step`     | **1.52 × 10⁻²**        | 1.51 × 10⁻²                | Equal accuracy on a hard discontinuity, no Gibbs ringing.                                                   |
| `Spike`    | **4.23 × 10⁻³**        | 2.55 × 10⁻³                | Narrow Gaussian spike: poly‑10 wins on raw MSE, but ExpRoot + Log is ~2× better than a 6‑knot cubic spline. |

> ⏱ Average runtime on 2 000 points (Apple M1, `cargo run --example benchmark`):  
> **ExpRoot + Log ≈ 47 ms**   |   **Poly deg 10 ≈ 32 ms**  
> Two SVDs of comparable size; speed improves proportionally if you reduce basis size or enable rayon.

### Why choose ExpRoot + Log?

-   **Handles exponential tails** without the blow‑up polynomials suffer.
-   **No Gibbs oscillations** on steps—log terms give smooth edge control.
-   **Linear least‑squares** → works in WASM, embedded, no external BLAS.
-   **Interpretable coefficients**: each term is a clear exponential or log “spring” shaping the curve.

<summary>Reproduce the benchmark</summary>

```bash
git clone https://github.com/andysay1/exp_root_log
cd exp_root_log
cargo run --example benchmark

```
