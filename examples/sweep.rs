use std::f64::consts::TAU;

use rbj_eq::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fc: f64 = args[1].parse().unwrap();
    let cs = PeakingFilter.coeffs(
        fc,
        FilterWidth::Slope {
            gain: 10.0,
            slope: 1.0,
        },
    );
    let mut filter = cs.make_filter();

    for i in 0..10_000 {
        // https://en.wikipedia.org/wiki/Chirp#Linear
        let t = i as f64 / 10_000.0;
        let x = f64::sin(0.5 * 0.5 * 10_000.0 * TAU * t * t);
        let y = filter(x);
        println!("{} {}", i / 2, y);
    }
}
