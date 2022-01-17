use std::f64::consts::PI;

use rbj_eq::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let f0: f64 = args[1].parse().unwrap();
    let fs = 48_000;
    let cs = BandPassQFilter.coeffs(
        fs as f64,
        f0,
        FilterWidth::Q(1.414),
    );
    let mut filter = Filter::new(cs);

    let x = (0..fs)
        .map(|i| {
            // https://en.wikipedia.org/wiki/Chirp#Linear
            let f0 = 100.0_f64;
            let f1 = 23_900.0;
            let c = 0.5 * (f1 - f0);
            let t = i as f64 / fs as f64;
            f64::sin(2.0 * PI * (c * t * t + f0 * t))
        });

    for y in x.into_iter().map(|x| filter.filter(x)) {
        println!("{}", y);
    }
}
