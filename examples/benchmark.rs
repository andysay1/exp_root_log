use std::time::Instant;
use exp_root_log::approx_exp_root_log;
use nalgebra as na;

// -------- тестовые функции -------------------------------------------------
fn sin_func(x: f64) -> f64    { (2.0 * std::f64::consts::PI * x).sin() }
fn exp_decay(x: f64) -> f64   { (-5.0 * x).exp() }
fn step(x: f64) -> f64        { if x < 0.5 { 1.0 } else { 0.0 } }
fn spike(x: f64) -> f64       { (-100.0 * (x - 0.5).powi(2)).exp() }

// ---------------------------------------------------------------------------
fn mse(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(u, v)| (u - v).powi(2)).sum::<f64>() / a.len() as f64
}

fn poly_ls(x: &[f64], y: &[f64], deg: usize) -> na::DVector<f64> {
    let n = x.len();
    let mut m = na::DMatrix::zeros(n, deg + 1);
    for (i, &xi) in x.iter().enumerate() {
        for j in 0..=deg {
            m[(i, j)] = xi.powi(j as i32);
        }
    }
    m.svd(true, true)
        .solve(&na::DVector::from_column_slice(y), 1e-12)
        .expect("poly solve")
}

fn poly_predict(coeffs: &[f64], x: f64) -> f64 {
    coeffs
        .iter()
        .enumerate()
        .map(|(j, &c)| c * x.powi(j as i32))
        .sum()
}

// --------------------------------------------------------------------------
fn bench_one<F>(name: &str, f: F)
where
    F: Fn(f64) -> f64,
{
    let x: Vec<f64> = (0..2000).map(|i| i as f64 / 2000.0).collect();
    let y: Vec<f64> = x.iter().map(|&xi| f(xi)).collect();

    // -------- ExpRoot+Log ---------------------------------------------------
    let t0 = Instant::now();
    let exp_fn = approx_exp_root_log(
        &x,
        &y,
        &[0.5, 2.0, 5.0, 10.0],
        5,
        &[1.0, 5.0, 10.0, 20.0],
    );
    let y_pred_exp: Vec<f64> = x.iter().map(|&xi| exp_fn(xi)).collect();
    let mse_exp = mse(&y, &y_pred_exp);
    let dt_exp = t0.elapsed();

    // -------- Полином (deg = 10) -------------------------------------------
    let t1 = Instant::now();
    let coeffs = poly_ls(&x, &y, 10);
    let y_pred_poly: Vec<f64> =
        x.iter().map(|&xi| poly_predict(coeffs.as_slice(), xi)).collect();
    let mse_poly = mse(&y, &y_pred_poly);
    let dt_poly = t1.elapsed();

    println!(
        "{:<8} | ExpRoot MSE = {:8.2e} {:>6?} || Poly MSE = {:8.2e} {:>6?}",
        name, mse_exp, dt_exp, mse_poly, dt_poly
    );
}

fn main() {
    println!("Function |     ExpRoot+Log             |       Polynomial(deg=10)");
    println!("--------------------------------------------------------------------------");
    bench_one("Sin", sin_func);
    bench_one("ExpDecay", exp_decay);
    bench_one("Step", step);
    bench_one("Spike", spike);
}
