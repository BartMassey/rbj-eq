//! https://webaudio.github.io/Audio-EQ-Cookbook/Audio-EQ-Cookbook.txt

use std::f64::consts::PI;

#[derive(Clone, Copy)]
pub enum FilterType {
    LowPass,
    HighPass,
    BandPassQ,
    BandPassC,
    BandNotch,
    AllPass,
    PeakingEq,
    LowShelf,
    HighShelf,
}
use FilterType::*;

#[derive(Clone, Copy)]
pub enum FilterWidth {
    Q(f64),
    BandWidth(f64),
    Slope(f64),
}

#[derive(Clone, Copy)]
pub struct FilterCoeffs {
    g: f64,
    b: [f64; 2],
    a: [f64; 2],
}

pub fn coeffs(
    filter_type: FilterType,
    fs: f64,
    f0: f64,
    gain: f64,
    width: FilterWidth,
) -> FilterCoeffs {
    let w0 = 2.0 * PI * f0 / fs;
    let cos_w0 = w0.cos();
    let sin_w0 = w0.sin();
    let alpha = match width {
        FilterWidth::Q(q) => 0.5 * sin_w0 / q,
        FilterWidth::BandWidth(bw) => sin_w0 * f64::sinh(
            0.5 * f64::ln(2.0) * bw * w0 / sin_w0
        ),
        FilterWidth::Slope(s) => {
            let a = f64::powf(10.0, gain / 40.0);
            0.5 * sin_w0 * f64::sqrt((a + 1.0 / a) * (1.0 / s - 1.0) + 2.0)
        },
    };
    let (b, a) = match filter_type {
        LowPass => (
            [0.5 - 0.5 * cos_w0, 1.0 - cos_w0, 0.5 - 0.5 * cos_w0],
            [1.0 + alpha, -2.0 * cos_w0, 1.0 - alpha],
        ),
        _ => todo!(),
    };
    FilterCoeffs {
        g: b[0] / a[0],
        b: [b[1] / a[0], b[2] / a[0]],
        a: [a[1] / a[0], a[2] / a[0]],
    }
}

pub fn make_filter(coeffs: FilterCoeffs) -> Box<dyn Fn(&[f64], &[f64]) -> f64> {
    Box::new(
        move |xs: &[f64], ys: &[f64]| {
            assert_eq!(3, xs.len());
            assert_eq!(2, ys.len());
            coeffs.g * xs[2] + coeffs.b[0] * xs[1] + coeffs.b[1] * xs[0]
                - coeffs.a[0] * ys[1] - coeffs.a[1] * ys[0]
        }
    )
}
