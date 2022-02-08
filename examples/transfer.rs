use std::f64::consts::TAU;

use rbj_eq::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fc: f64 = args[1].parse().unwrap();
    let coeffs = PeakingFilter.coeffs(
        fc,
        FilterWidth::Slope {
            gain: 10.0,
            slope: 1.0,
        },
    );

    let transfer = coeffs.make_transfer_mag();
    for i in 0..10_000 {
        let x = 0.5 * TAU * i as f64 / 10_000.0;
        let y = transfer(x);
        println!("{} {}", i / 2, y);
    }
}
