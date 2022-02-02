![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# rbj-eq: Rust implementation of RBJ EQ filters
Bart Massey 2022 (version 0.1.0)

## Background

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

## RBJ Filters

This crate provides implementations of the RBJ filters in
safe Rust. What you get:

* Function to compute filter coefficients for the various
  RBJ filter types.

* Transfer function magnitude, derived from the
  coefficients.

* A stateful filter function, based on the coefficients.

## Examples

```rust
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


Full crate [rustdoc](https://bartmassey.github.io/rbj-eq/rbj-eq/index.html)
is available.

This crate is made available under the "MIT
license". Please see the file `LICENSE.txt` in this distribution
for license terms.

Thanks to the `cargo-readme` crate for generation of this `README`.
