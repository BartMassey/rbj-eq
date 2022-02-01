use std::f64::consts::PI;

use rbj_eq::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let f0: f64 = args[1].parse().unwrap();
    let fs = 24_000;
    let coeffs = PeakingFilter.coeffs(
        fs as f64,
        f0,
        FilterWidth::Slope { gain: 10.0, slope: 1.0 },
    );

    for i in 0..fs {
        let x = PI * i as f64 / fs as f64;
        let y = coeffs.transfer(x);
        println!("{} {}", i / 2, y);
    }
}
