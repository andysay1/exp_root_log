use nalgebra::{DMatrix, DVector};

/// Constructs the design matrix using basis functions:
/// - exp(-sqrt(b * x))
/// - x^j (polynomials)
/// - log(1 + λ * x)
fn build_basis(
    x: &[f64],
    b_list: &[f64],
    poly_deg: usize,
    log_lambdas: &[f64],
) -> DMatrix<f64> {
    let n = x.len();
    let n_cols = b_list.len() + (poly_deg + 1) + log_lambdas.len();
    let mut mat = DMatrix::zeros(n, n_cols);

    for (i, &xi) in x.iter().enumerate() {
        let mut col = 0;

        // Exponential-root terms
        for &b in b_list {
            mat[(i, col)] = (-((b * xi).sqrt())).exp();
            col += 1;
        }

        // Polynomial terms
        for j in 0..=poly_deg {
            mat[(i, col)] = xi.powi(j as i32);
            col += 1;
        }

        // Logarithmic terms
        for &lambda in log_lambdas {
            mat[(i, col)] = (1.0 + lambda * xi).ln();
            col += 1;
        }
    }

    mat
}

/// Fits an ExpRoot+Log approximation to the (x, y) data,
/// and returns a closure representing the approximate function.
pub fn approx_exp_root_log(
    x: &[f64],
    y: &[f64],
    b_list: &[f64],
    poly_deg: usize,
    log_lambdas: &[f64],
) -> impl Fn(f64) -> f64 {
    let design = build_basis(x, b_list, poly_deg, log_lambdas);
    let y_vec = DVector::from_column_slice(y);

    // Solve least squares using QR decomposition
    let coeffs = design
    .svd(true, true)
    .solve(&y_vec, 1e-10)
    .expect("SVD solve failed");


    let coeffs = coeffs.data.as_vec().clone();
    let b_list = b_list.to_vec();
    let log_lambdas = log_lambdas.to_vec();

    move |x: f64| {
        let mut result = 0.0;
        let mut idx = 0;

        for &b in &b_list {
            result += coeffs[idx] * (-((b * x).sqrt())).exp();
            idx += 1;
        }

        for j in 0..=poly_deg {
            result += coeffs[idx] * x.powi(j as i32);
            idx += 1;
        }

        for &lambda in &log_lambdas {
            result += coeffs[idx] * (1.0 + lambda * x).ln();
            idx += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mse(y_true: &[f64], y_pred: &[f64]) -> f64 {
        y_true
            .iter()
            .zip(y_pred)
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            / y_true.len() as f64
    }

    #[test]
    fn test_sin_approximation() {
        let x: Vec<f64> = (0..100).map(|i| i as f64 / 100.0).collect();
        let y: Vec<f64> = x
            .iter()
            .map(|&x| (2.0 * std::f64::consts::PI * x).sin())
            .collect();

        let approx_fn = approx_exp_root_log(
            &x,
            &y,
            &[0.5, 2.0, 5.0],   // b_i
            3,                  // polynomial degree
            &[1.0, 5.0, 10.0],  // λ_k
        );

        let y_pred: Vec<f64> = x.iter().map(|&xi| approx_fn(xi)).collect();
        let error = mse(&y, &y_pred);

        println!("MSE for sin approximation: {:.2e}", error);
        assert!(error < 1e-3, "MSE too high: {}", error);
    }
}
