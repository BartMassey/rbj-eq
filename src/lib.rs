//! https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html

mod filter_names;
pub use filter_names::*;

use std::f64::consts::TAU;

#[derive(Debug, Clone, Copy)]
pub enum BasicFilter {
    LowPass,
    HighPass,
    BandPassQ,
    BandPassC,
    BandNotch,
    AllPass,
}
use BasicFilter::*;

#[derive(Debug, Clone, Copy)]
pub enum ShelfFilter {
    LowShelf,
    HighShelf,
}
use ShelfFilter::*;

#[derive(Debug, Clone, Copy)]
pub enum EqFilter {
    Shelf(ShelfFilter),
    Peaking,
}
use EqFilter::*;

#[derive(Debug, Clone, Copy)]
pub enum FilterWidth {
    Q(f64),
    BandWidth(f64),
    Slope { gain: f64, slope: f64 },
}

#[derive(Debug, Clone, Copy)]
pub enum FilterType {
    Basic(BasicFilter),
    Eq(EqFilter),
}
use FilterType::*;

impl FilterType {
    pub fn coeffs(self, fs: f64, f0: f64, width: FilterWidth) -> FilterCoeffs {
        let w0 = TAU * f0 / fs;
        let sin_w0 = w0.sin();
        let (a2, alpha) = match width {
            FilterWidth::Q(q) => {
                let alpha = 0.5 * sin_w0 / q;
                (0.0, alpha)
            }
            FilterWidth::BandWidth(bw) => {
                #[rustfmt::skip]
                let alpha = sin_w0 * f64::sinh(
                    0.5 * f64::ln(2.0) * bw * w0 / sin_w0
                );
                (0.0, alpha)
            }
            FilterWidth::Slope { gain, slope } => {
                let a2 = f64::powf(10.0, gain / 80.0);
                let a = a2 * a2;
                #[rustfmt::skip]
                let alpha = sin_w0 * 0.5 * f64::sqrt(
                    (a + 1.0 / a) * (1.0 / slope - 1.0) + 2.0
                );
                (a2, alpha)
            }
        };
        let cos_w0 = w0.cos();
        let cos_2m = -2.0 * cos_w0;
        let (b, a) = match self {
            Basic(filter_type) => {
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
                (b, a)
            }
            Eq(filter_type) => {
                let a1 = a2 * a2;
                match filter_type {
                    Peaking => {
                        let b = [1.0 + alpha * a1, cos_2m, 1.0 - alpha * a1];
                        let a = [1.0 + alpha / a1, cos_2m, 1.0 - alpha / a1];
                        (b, a)
                    }
                    Shelf(filter_type) => {
                        let a1p = a1 + 1.0;
                        let a1m = a1 - 1.0;
                        let a1pc = a1p * cos_w0;
                        let a1mc = a1m * cos_w0;
                        let s2 = 2.0 * a2 * alpha;
                        match filter_type {
                            LowShelf => {
                                let b = [
                                    a1 * (a1p - a1mc + s2),
                                    2.0 * a1 * (a1m - a1pc),
                                    a1 * (a1p - a1mc + s2),
                                ];
                                #[rustfmt::skip]
                                let a = [
                                    a1p + a1mc + s2,
                                    2.0 * (a1m - a1pc),
                                    a1p + a1mc - s2,
                                ];
                                (b, a)
                            }
                            HighShelf => {
                                let b = [
                                    a1 * (a1p + a1mc + s2),
                                    -2.0 * a1 * (a1m + a1pc),
                                    a1 * (a1p + a1mc - s2),
                                ];
                                #[rustfmt::skip]
                                let a = [
                                    a1p - a1mc + s2,
                                    2.0 * (a1m - a1pc),
                                    a1p - a1mc - s2,
                                ];
                                (b, a)
                            }
                        }
                    }
                }
            }
        };
        FilterCoeffs::new(b, a)
    }
}

#[derive(Clone)]
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

    // https://dsp.stackexchange.com/a/16911
    pub fn transfer(&self, w: f64) -> f64 {
        let phi = f64::sin(0.5 * w);
        let phi = phi * phi;
        let a = [1.0, self.a[0], self.a[1]];
        let b = [self.g, self.b[0], self.b[1]];
        let erator = |c: [f64; 3]| -> f64 {
            let t1 = 0.5 * (c[0] + c[1] + c[2]);
            #[rustfmt::skip]
            let t2 = -phi * (
                4.0 * c[0] * c[2] * (1.0 - phi) +
                c[1] * (c[0] + c[2])
            );
            t1 * t1 + t2
        };
        f64::sqrt(erator(b) / erator(a))
    }
}

#[derive(Clone)]
pub struct Filter {
    ys: [f64; 2],
    xs: [f64; 2],
    coeffs: FilterCoeffs,
}

impl Filter {
    pub fn new(coeffs: FilterCoeffs) -> Self {
        Self {
            ys: [0.0; 2],
            xs: [0.0; 2],
            coeffs,
        }
    }

    pub fn filter(&mut self, x: f64) -> f64 {
        #[rustfmt::skip]
        let y = self.coeffs.g * x
            + self.coeffs.b[0] * self.xs[0] + self.coeffs.b[1] * self.xs[1]
            - self.coeffs.a[0] * self.ys[0] - self.coeffs.a[1] * self.ys[1];
        self.ys[1] = self.ys[0];
        self.ys[0] = y;
        self.xs[1] = self.xs[0];
        self.xs[0] = x;
        y
    }
}
