#![doc(html_root_url = "https://docs.rs/rbj_eq/0.7.0")]

/*!

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

## Rust RBJ Filter Crate

This `no_std` crate provides implementations of the RBJ
filters in safe Rust. What you get:

* Function to compute filter coefficients for the various
  RBJ filter types.

* Transfer function magnitude, derived from the
  coefficients.

* A stateful filter function, based on the coefficients.

## Examples

```
use rbj_eq::{LowPassFilter, FilterWidth};

// Make a sine wave at Nyquist.
let samples: Vec<f64> = (0..128)
    .map(|i| if i % 2 == 0 { 1.0 } else { -1.0 })
    .collect();

// Construct a half-band filter.
let cs = LowPassFilter.coeffs(
    0.5,
    FilterWidth::Slope {
        gain: 0.0,
        slope: 1.0,
    },
);
let mut filter = cs.to_filter();

// Filter the signal.
let filtered: Vec<f64> =
    samples.into_iter().map(|x| filter(x)).collect();

// The signal is damped. (The filter takes a few samples to converge.)
for (i, y) in filtered.iter().skip(4).enumerate() {
    assert!(y.abs() < 0.01, "filter fail: {i} {y}");
}
```

(See the `examples` directory of this distribution for more examples.)

## Feature Flags

* `math_libm`: Use the `libm` crate and its port of the MUSL
  floating point libraries to Rust, via the `num-traits`
  crate. At least one of `math_libm` or `math_std` must be
  enabled.

* `math_std`: Use the Rust `std` math library via the
  `num-traits` crate. At least one of `math_libm` or
  `math_std` must be enabled.

* `capi`: Include a C FFI API.

* `serde`: Support `serde::Serialize` and
  `serde::Deserialize` for all data structures.

*/

#![no_std]

#[cfg(feature = "capi")]
mod capi;
#[cfg(feature = "capi")]
pub use capi::*;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use num_traits::float::*;
use numeric_literals::replace_float_literals;

mod filter_names;
pub use filter_names::*;

#[doc(hidden)]
/// Filter types for "standard" filters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ShelfFilter {
    /// Lowpass shelf filter.
    LowShelf,
    /// Highpass shelf filter.
    HighShelf,
}
use ShelfFilter::*;

#[doc(hidden)]
/// Filter types for EQ filters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EqFilter {
    /// Shelf filter.
    Shelf(ShelfFilter),
    /// Peaking bandpass filter.
    Peaking,
}
use EqFilter::*;

#[doc(hidden)]
/// Filters are either "standard" or RBJ-eq-style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FilterType {
    /// "Standard" filter.
    Basic(BasicFilter),
    /// RBJ equalizer filter.
    Eq(EqFilter),
}
use FilterType::*;

/// Width / gain specification for filters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FilterWidth<F: Float> {
    /// Specify width / gain using "EE Q".
    Q(F),
    /// Specify width / gain using filter half-band width.
    BandWidth(F),
    /// Specify width / gain using filter peak gain and tail slope.
    Slope { gain: F, slope: F },
}

impl FilterType {
    /// Calculate biquad filter coefficients for the given filter type,
    /// critical frequency (as fraction of Nyquist), and filter width.
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn coeffs<F: Float + FloatConst>(self, fc: F, width: FilterWidth<F>) -> FilterCoeffs<F> {
        let w0 = 0.5 * FloatConst::TAU() * fc;
        let sin_w0 = w0.sin();
        let (a2, alpha) = match width {
            FilterWidth::Q(q) => {
                let alpha = 0.5 * sin_w0 / q;
                (0.0, alpha)
            }
            FilterWidth::BandWidth(bw) => {
                #[rustfmt::skip]
                let alpha = sin_w0 * F::sinh(
                    0.5 * F::ln(2.0) * bw * w0 / sin_w0
                );
                (0.0, alpha)
            }
            FilterWidth::Slope { gain, slope } => {
                let a2 = F::powf(10.0, gain / 80.0);
                let a = a2 * a2;
                #[rustfmt::skip]
                let alpha = sin_w0 * 0.5 * F::sqrt(
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

/// Biquad filter coefficients.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FilterCoeffs<F: Float> {
    pub b: [F; 3],
    pub a: [F; 3],
}

impl<F: Float> FilterCoeffs<F> {
    fn new(b: [F; 3], a: [F; 3]) -> Self {
        Self { b, a }
    }

    /// Returns a transfer function for this filter. Input
    /// is fraction of unit filter frequency, output is
    /// magnitude of gain.  This uses a nice [transformation
    /// by RBJ](https://dsp.stackexchange.com/a/16911) for
    /// better numerical stability.
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn to_transfer_fn(&self) -> impl Fn(F) -> F + '_ {
        |w: F| {
            let phi = F::sin(0.5 * w);
            let phi = phi * phi;
            let erator = |c: &[F; 3]| -> F {
                let t1 = 0.5 * (c[0] + c[1] + c[2]);
                #[rustfmt::skip]
                let t2 = -phi * (
                    4.0 * c[0] * c[2] * (1.0 - phi) +
                    c[1] * (c[0] + c[2])
                );
                t1 * t1 + t2
            };
            F::sqrt(erator(&self.b) / erator(&self.a))
        }
    }

    /// Make a new biquad filter function with the given
    /// coefficients.  Initial state is zeros.
    // (This is a Direct Form I implementation, per RBJ
    // recommendation.)
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn to_filter(&self) -> impl FnMut(F) -> F + '_ {
        let a_inv = 1.0 / self.a[0];
        let g = self.b[0] * a_inv;
        let b = [self.b[1] * a_inv, self.b[2] * a_inv];
        let a = [self.a[1] * a_inv, self.a[2] * a_inv];
        let mut ys = [0.0, 0.0];
        let mut xs = [0.0, 0.0];
        move |x| {
            #[rustfmt::skip]
            let y = g * x
                + b[0] * xs[0] + b[1] * xs[1]
                - a[0] * ys[0] - a[1] * ys[1];
            ys[1] = ys[0];
            ys[0] = y;
            xs[1] = xs[0];
            xs[0] = x;
            y
        }
    }
}
