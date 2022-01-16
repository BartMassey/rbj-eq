use std::f64::consts::PI;

use rbj_eq::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let f0: f64 = args[1].parse().unwrap();
    let fs = 48_000;
    let cs = basic_filter_coeffs(
        BasicFilterType::BandPassQ,
        fs as f64,
        f0,
        BasicFilterWidth::Q(1.414),
    );
    let mut filter = make_filter(cs);

    let x: Vec<f64> = (0..fs)
        .map(|i| {
            // https://en.wikipedia.org/wiki/Chirp#Linear
            let f0 = 100.0_f64;
            let f1 = 23_900.0;
            let c = 0.5 * (f1 - f0);
            let t = i as f64 / fs as f64;
            f64::sin(2.0 * PI * (c * t * t + f0 * t))
        })
        .collect();

    let y: Vec<f64> = x.into_iter().map(|x| filter(x)).collect();

    y.into_iter().for_each(|y| println!("{}", y));
}
