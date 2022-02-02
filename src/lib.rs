/*!
# Background

Back in the day, DSP guru Robert Bristow-Johnson published a
famous document titled [*Cookbook formulae for audio
equalizer biquad filter
coefficients*](https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html). This
is a really nice account of how to build "biquad" IIR
filters useful for audio equalization, and also for a variety of
other audio tasks. The RBJ filters are characterized by
being extremely cheap to run, cheap to build on-the-fly, and
having nice composability properties.

Many implementations of the RBJ filters exist in a variety
of languages. This is the author's implementation in Rust.

# RBJ Filters

This crate provides implementations of the RBJ filters in
safe Rust. What you get:

* Function to compute filter coefficients for the various
  RBJ filter types.

* Transfer function magnitude, derived from the
  coefficients.

* A stateful filter function, based on the coefficients.

# Examples

```
use rbj_eq::{LowPassFilter, Filter, FilterWidth};

// Make a sine wave at Nyquist.
let samples: Vec<f64> = (0..128)
    .map(|i| if i % 2 == 0 { 1.0 } else { -1.0 })
    .collect();

// Construct a half-band filter.
let cs = LowPassFilter.coeffs(
    24_000.0,
    6_000.0,
    FilterWidth::Slope {
        gain: 0.0,
        slope: 1.0,
    },
);
let mut filter = Filter::new(cs);

// Filter the signal.
let filtered: Vec<f64> =
    samples.into_iter().map(|x| filter.filter(x)).collect();

// The signal is damped. (The filter takes a few samples to converge.)
for (i, y) in filtered.iter().skip(4).enumerate() {
    assert!(y.abs() < 0.01, "filter fail: {i} {y}");
}
```

(See the `examples` directory of this distribution for more examples.)

*/

mod filter_names;
pub use filter_names::*;

use std::f64::consts::TAU;

#[doc(hidden)]
/// Filter types for "standard" filters.
#[derive(Debug, Clone, Copy)]
pub enum BasicFilter {
    /// Lowpass filter.
    LowPass,
    /// Highpass filter.
    HighPass,
    /// Bandpass filter, with constant skirt gain and peak gain Q.
    BandPassQ,
    /// Bandpass filter, with constant 0dB peak gain.
    BandPassC,
    /// Bandnotch filter.
    BandNotch,
    /// All-pass filter.
    AllPass,
}
use BasicFilter::*;

#[doc(hidden)]
/// Filter types for EQ shelf filters.
#[derive(Debug, Clone, Copy)]
pub enum ShelfFilter {
    /// Lowpass shelf filter.
    LowShelf,
    /// Highpass shelf filter.
    HighShelf,
}
use ShelfFilter::*;

#[doc(hidden)]
/// Filter types for EQ filters.
#[derive(Debug, Clone, Copy)]
pub enum EqFilter {
    /// Shelf filter.
    Shelf(ShelfFilter),
    /// Peaking bandpass filter.
    Peaking,
}
use EqFilter::*;

#[doc(hidden)]
/// Filters are either "standard" or RBJ-eq-style.
#[derive(Debug, Clone, Copy)]
pub enum FilterType {
    /// "Standard" filter.
    Basic(BasicFilter),
    /// RBJ equalizer filter.
    Eq(EqFilter),
}
use FilterType::*;

/// Width / gain specification for filters.
#[derive(Debug, Clone, Copy)]
pub enum FilterWidth {
    /// Specify width / gain using "EE Q".
    Q(f64),
    /// Specify width / gain using filter half-band width.
    BandWidth(f64),
    /// Specify width / gain using filter peak gain and tail slope.
    Slope { gain: f64, slope: f64 },
}

impl FilterType {
    /// Calculate biquad filter coefficients for the given filter type,
    /// sampling frequency (actually sampling rate, so twice Nyquist),
    /// critical frequency, and filter width.
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

/// Biquad filter coefficients (normalized for filter operation).
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

    /// Transfer function magnitude for filter, at given
    /// fraction of unit frequency.  This uses a nice
    /// [transformation by RBJ](https://dsp.stackexchange.com/a/16911)
    /// for better numerical stability.
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

/// Biquad filter state (including coefficients).
#[derive(Clone)]
pub struct Filter {
    ys: [f64; 2],
    xs: [f64; 2],
    coeffs: FilterCoeffs,
}

impl Filter {
    /// Make a new biquad filter with the given coefficients.
    /// Initial state is zeros.
    pub fn new(coeffs: FilterCoeffs) -> Self {
        Self {
            ys: [0.0; 2],
            xs: [0.0; 2],
            coeffs,
        }
    }

    /// Step the filter forward, advancing the state and returning
    /// one output.
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
