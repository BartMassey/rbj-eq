![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# {{crate}}: Rust implementation of RBJ EQ filters
Bart Massey 2022 (version {{version}})

{{readme}}

## C API

This crate has the `capi` feature enabled by default, to
build C libraries that you can install using `cargo-c`. The
build and install is as follows:

```
cargo build --release
cargo install cbindgen
cargo install cargo-c
cargo +nightly cbuild --release
sudo cargo +nightly cinstall --release --prefix=/usr/local
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

This crate is made available under the "{{license}}
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
