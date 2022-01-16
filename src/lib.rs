//! https://webaudio.github.io/Audio-EQ-Cookbook/Audio-EQ-Cookbook.txt

use std::f64::consts::PI;

#[derive(Clone, Copy)]
pub enum BasicFilterType {
    LowPass,
    HighPass,
    BandPassQ,
    BandPassC,
    BandNotch,
    AllPass,
}
use BasicFilterType::*;

#[derive(Clone, Copy)]
pub enum BasicFilterWidth {
    Q(f64),
    BandWidth(f64),
}

#[derive(Clone, Copy)]
pub struct FilterCoeffs {
    g: f64,
    b: [f64; 2],
    a: [f64; 2],
}

impl FilterCoeffs {
    fn new(b: [f64; 3], a: [f64; 3]) -> Self {
        let a_inv = 1.0 / a[0];
        FilterCoeffs {
            g: b[0] * a_inv,
            b: [b[1] * a_inv, b[2] * a_inv],
            a: [a[1] * a_inv, a[2] * a_inv],
        }
    }
}

pub fn basic_filter_coeffs(
    filter_type: BasicFilterType,
    fs: f64,
    f0: f64,
    width: BasicFilterWidth,
) -> FilterCoeffs {
    let w0 = 2.0 * PI * f0 / fs;
    let sin_w0 = w0.sin();
    let alpha = match width {
        BasicFilterWidth::Q(q) =>
            0.5 * sin_w0 / q,
        BasicFilterWidth::BandWidth(bw) =>
            sin_w0 * f64::sinh(
                0.5 * f64::ln(2.0) * bw * w0 / sin_w0
            ),
    };
    let cos_w0 = w0.cos();
    let cos_2m = -2.0 * cos_w0;
    let sin_p2 = 0.5 * sin_w0;
    let sin_m2 = -0.5 * sin_w0;
    let cos_1m = 1.0 - cos_w0;
    let cos_1pm = -(1.0 + cos_w0);
    let cos_1m2 = 0.5 - 0.5 * cos_w0;
    let cos_1p2 = 0.5 + 0.5 * cos_w0;
    let a_1m = 1.0 - alpha;
    let a_1p = 1.0 + alpha;
    let a = [a_1p, cos_2m, a_1m];
    let b = match filter_type {
        LowPass => [cos_1m2, cos_1m, cos_1m2],
        HighPass => [cos_1p2, cos_1pm, cos_1p2],
        BandPassQ => [sin_p2, 0.0, sin_m2],
        BandPassC => [alpha, 0.0, -alpha],
        BandNotch => [1.0, cos_2m, 1.0],
        AllPass => [a_1m, cos_2m, a_1p],
    };
    FilterCoeffs::new(b, a)
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
