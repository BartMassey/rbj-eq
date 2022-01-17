#![allow(non_upper_case_globals)]

use crate::*;

pub const LowPassFilter: FilterType = FilterType::Basic(LowPass);
pub const HighPassFilter: FilterType = FilterType::Basic(HighPass);
pub const BandPassQFilter: FilterType = FilterType::Basic(BandPassQ);
pub const BandPassCFilter: FilterType = FilterType::Basic(BandPassC);
pub const BandNotchFilter: FilterType = FilterType::Basic(BandNotch);
pub const AllPassFilter: FilterType = FilterType::Basic(AllPass);
pub const LowShelfFilter: FilterType = FilterType::Eq(Shelf(LowShelf));
pub const HighShelfFilter: FilterType = FilterType::Eq(Shelf(HighShelf));
pub const PeakingFilter: FilterType = FilterType::Eq(Peaking);
