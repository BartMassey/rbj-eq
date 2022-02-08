use std::f64::consts::TAU;

use rbj_eq::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let f0: f64 = args[1].parse().unwrap();
    let fs = 24_000;
    let cs = PeakingFilter.coeffs(
        fs as f64,
        f0,
        FilterWidth::Slope {
            gain: 10.0,
            slope: 1.0,
        },
    );
    let mut filter = cs.make_filter();

    for i in 0..fs {
        // https://en.wikipedia.org/wiki/Chirp#Linear
        let t = i as f64 / fs as f64;
        let x = f64::sin(0.5 * 0.5 * fs as f64 * TAU * t * t);
        let y = filter(x);
        println!("{} {}", i / 2, y);
    }
}
