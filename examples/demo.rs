use exp_root_log::approx_exp_root_log;

fn main() {
    // Sample data: x values and corresponding y values
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
