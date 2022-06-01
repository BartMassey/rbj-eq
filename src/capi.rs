/*!

A C interface to the filter library.

*/

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum filter_t {
    /// Lowpass filter.
    LOW_PASS_FILTER,
    /// Highpass filter.
    HIGH_PASS_FILTER,
    /// Bandpass filter, with constant skirt gain and peak gain Q.
    BAND_PASS_Q_FILTER,
    /// Bandpass filter, with constant 0dB peak gain.
    BAND_PASS_C_FILTER,
    /// Bandnotch filter.
    BAND_NOTCH_FILTER,
    /// All-pass filter.
    ALL_PASS_FILTER,
    /// Lowpass shelf filter.
    LOW_SHELF_FILTER,
    /// Highpass shelf filter.
    HIGH_SHELF_FILTER,
    /// Peaking bandpass filter.
    PEAKING_FILTER,
}
use filter_t::*;

fn get_filter_type(filter_type: filter_t) -> crate::FilterType {
    match filter_type {
        LOW_PASS_FILTER => crate::LowPassFilter,
        HIGH_PASS_FILTER => crate::HighPassFilter,
        BAND_PASS_Q_FILTER => crate::BandPassQFilter,
        BAND_PASS_C_FILTER => crate::BandPassCFilter,
        BAND_NOTCH_FILTER => crate::BandNotchFilter,
        ALL_PASS_FILTER => crate::AllPassFilter,
        LOW_SHELF_FILTER => crate::LowShelfFilter,
        HIGH_SHELF_FILTER => crate::HighShelfFilter,
        PEAKING_FILTER => crate::PeakingFilter,
    }
}

macro_rules! filter_coeffs_t {
    ($name:ident, $ft:ty) => {
        #[allow(non_camel_case_types)]
        #[repr(C)]
        pub struct $name {
            pub b: [$ft; 3],
            pub a: [$ft; 3],
        }
    };
}

filter_coeffs_t!(filter_coeffs_d_t, f64);
filter_coeffs_t!(filter_coeffs_f_t, f32);

macro_rules! filter_coeffs_q {
    ($name:ident, $ft:ty, $result:tt) => {
        #[no_mangle]
        pub extern "C" fn $name(filter_type: filter_t, fc: $ft, q: $ft) -> $result {
            let filter_type = get_filter_type(filter_type);
            let crate::FilterCoeffs { a, b } =
                filter_type.coeffs(fc, crate::FilterWidth::Q(q));
            $result { a, b }
        }
    };
}

filter_coeffs_q!(filter_coeffs_q_d, f64, filter_coeffs_d_t);
filter_coeffs_q!(filter_coeffs_q_f, f32, filter_coeffs_f_t);

macro_rules! filter_coeffs_bandwidth {
    ($name:ident, $ft:ty, $result:tt) => {
        #[no_mangle]
        pub extern "C" fn $name(filter_type: filter_t, fc: $ft, band_width: $ft) -> $result {
            let filter_type = get_filter_type(filter_type);
            let crate::FilterCoeffs { a, b } =
                filter_type.coeffs(fc, crate::FilterWidth::BandWidth(band_width));
            $result { a, b }
        }
    };
}

filter_coeffs_bandwidth!(filter_coeffs_bandwidth_d, f64, filter_coeffs_d_t);
filter_coeffs_bandwidth!(filter_coeffs_bandwidth_f, f32, filter_coeffs_f_t);


macro_rules! filter_coeffs_slope {
    ($name:ident, $ft:ty, $result:tt) => {
        #[no_mangle]
        pub extern "C" fn $name(filter_type: filter_t, fc: $ft, gain: $ft, slope: $ft) -> $result {
            let filter_type = get_filter_type(filter_type);
            let crate::FilterCoeffs { a, b } =
                filter_type.coeffs(fc, crate::FilterWidth::Slope { gain, slope });
            $result { a, b }
        }
    };
}

filter_coeffs_slope!(filter_coeffs_slope_d, f64, filter_coeffs_d_t);
filter_coeffs_slope!(filter_coeffs_slope_f, f32, filter_coeffs_f_t);

macro_rules! filter_coeffs_df1_t {
    ($name:ident, $ft:ty) => {
        #[allow(non_camel_case_types)]
        #[repr(C)]
        pub struct $name {
            pub g: $ft,
            pub b: [$ft; 2],
            pub a: [$ft; 2],
        }
    };
}

filter_coeffs_df1_t!(filter_coeffs_df1_d_t, f64);
filter_coeffs_df1_t!(filter_coeffs_df1_f_t, f32);

macro_rules! to_df1 {
    ($name:ident, $src:ty, $result:tt) => {
        #[no_mangle]
        pub extern "C" fn $name(coeffs: &$src) -> $result {
            let a_inv = 1.0 / coeffs.a[0];
            $result {
                g: coeffs.b[0] * a_inv,
                b: [coeffs.b[1] * a_inv, coeffs.b[2] * a_inv],
                a: [coeffs.a[1] * a_inv, coeffs.a[2] * a_inv],
            }
        }
    };
}

to_df1!(to_df1_d, filter_coeffs_d_t, filter_coeffs_df1_d_t);
to_df1!(to_df1_f, filter_coeffs_f_t, filter_coeffs_df1_f_t);

macro_rules! filter_df1 {
    ($name:ident, $ft:ty, $src:ty) => {
        #[no_mangle]
        pub extern "C" fn $name(cs: &$src, s: &mut [$ft;4], x: $ft) -> $ft {
            #[rustfmt::skip]
            let y = cs.g * x
                + cs.b[0] * s[0] + cs.b[1] * s[1]
                - cs.a[0] * s[2] - cs.a[1] * s[3];
            s[3] = s[2];
            s[2] = y;
            s[1] = s[0];
            s[0] = x;
            y
        }
    };
}

filter_df1!(filter_df1_d, f64, filter_coeffs_df1_d_t);
filter_df1!(filter_df1_f, f32, filter_coeffs_df1_f_t);
