![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# rbj_eq: Rust implementation of RBJ EQ filters
Bart Massey 2022 (version 0.6.0)

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
let mut filter = cs.make_filter();

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
  `num-traits` crate. Implies `std`. At least one of
  `math_libm` or `math_std` must be enabled.

* `std`: Use the Rust `std` library. When not present, the
  crate will be built `no_std`.

* `capi`: Include a C FFI API.


## C API

This crate has the `capi` feature enabled by default, to
build C libraries that you can install using `cargo-c`. The
build and install is as follows:

```
cargo build --release
cargo install cbindgen
cargo install cargo-c
cargo +nightly cbuild --release
cargo cinstall --release --prefix=/usr/local
```

See the `cargo-c` [repo](http://github.com/lu-zero/cargo-c)
`README` for more information and options. The `c-examples/`
directory in this distribution has an example use of the C
library.

This crate can be used as a "`no_std`" library for embedded
work. In this case it might use the Rust `libm` crate for
its floating-point numeric functions.

## Addenda

Full crate [rustdoc](https://bartmassey.github.io/rbj-eq/rbj_eq/index.html)
is available.

This crate is made available under the "MIT
license". Please see the file `LICENSE.txt` in this distribution
for license terms.

Thanks to Robert Bristow-Johnson for sharing not only these
filters but a bunch of knowledge about how to implement
them. Thanks to YouTuber Dan Worrall for introducing me to
the RBJ filters, as well as for some amazing audio DSP
content. Thanks to the authors of the `num-traits` and
`numeric_literals` crates for making support for `f32`
easy. Thanks to the `cargo-c` authors for making the C
library build mess manageable.  Finally, thanks to the
`cargo-readme` crate for generation of this `README`.
