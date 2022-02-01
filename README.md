# rbj-filters: Rust implementation of RBJ filters
Bart Massey 2021

**This is a very, very early work-in-progress crate. Please
treat it as radioactive until further notice.**

Back in the day, DSP guru Robert Bristow-Johnson published a
famous document entitled
[Cookbook formulae for audio equalizer biquad filter coefficients](https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html). This
is a really nice account of how to build "bi-quad" IIR
filters useful in audio equalization, and also a variety of
other audio tasks. The RBJ filters are characterized by
being extremely cheap to run, cheap to build on-the-fly, and
having nice composability properties.

Many implementations of the RBJ filters exist, but I
couldn't find one in Rust, and I wanted it for a Rust
project I was working on. This proposes to be such.

For now, the crate provides barely-tried implementations of
the RBJ filters in safe Rust. You can look at the Rustdoc
for these once it's available. There is also an attempt at
transfer function calculation for these filters.

No license as of yet, since I wouldn't encourage anyone to
use this until it is more cooked. *Caveat emptor.*
