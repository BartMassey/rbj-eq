use rbj_eq::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let f0: f64 = args[1].parse().unwrap();
    let fs = 24_000;
    let transfer = PeakingFilter.transfer(
        f0 / fs as f64,
        FilterWidth::Slope { gain: 10.0, slope: 1.0 },
    );

    let x = (100..fs).map(|i| i as f64 / fs as f64);
    for y in x.into_iter().map(|x| transfer.transfer(x)) {
        println!("{}", y);
    }
}
