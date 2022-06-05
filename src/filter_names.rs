#![allow(non_upper_case_globals)]

//! Filter name consts for the various filter types, to ease
//! filter specification.

use crate::*;

/// Lowpass filter.
pub const LowPassFilter: FilterType = FilterType::Basic(LowPass);
/// Highpass filter.
pub const HighPassFilter: FilterType = FilterType::Basic(HighPass);
/// Bandpass filter, with constant skirt gain and peak gain Q.
pub const BandPassQFilter: FilterType = FilterType::Basic(BandPassQ);
/// Bandpass filter, with constant 0dB peak gain.
pub const BandPassCFilter: FilterType = FilterType::Basic(BandPassC);
/// Bandnotch filter.
pub const BandNotchFilter: FilterType = FilterType::Basic(BandNotch);
/// All-pass filter.
pub const AllPassFilter: FilterType = FilterType::Basic(AllPass);
/// Lowpass shelf filter.
pub const LowShelfFilter: FilterType = FilterType::Eq(Shelf(LowShelf));
/// Highpass shelf filter.
pub const HighShelfFilter: FilterType = FilterType::Eq(Shelf(HighShelf));
/// Peaking bandpass filter.
pub const PeakingFilter: FilterType = FilterType::Eq(Peaking);
