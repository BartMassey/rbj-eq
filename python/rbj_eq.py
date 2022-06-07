import numpy as np

tau = 2 * np.pi

_basic_filters = {
    "low_pass",
    "high_pass",
    "band_pass_q",
    "band_pass_c",
    "band_notch",
    "all_pass",
}

_shelf_filters = {
    "low_shelf",
    "high_shelf",
}

def filter_coeffs(ft, fc, width):
    """
    Use scipy.signal.tf2sos(coeffs[0], coeffs[1]) to
    convert to a form suitable for scipy.signal.sosfilt
    """
    w0 = 0.5 * tau * fc
    sin_w0 = np.sin(w0)
    if width[0] == 'q':
        q = width[1]
        alpha = 0.5 * sin_w0 / q
        a2 = 0.0
    elif width[0] == 'bw':
        bw = width[1]
        alpha = sin_w0 * np.sinh(
            0.5 * np.log(2.0) * bw * w0 / sin_w0
        )
        a2 = 0.0
    elif width[0] == 'slope':
        gain = width[1]
        slope = width[2]
        a2 = 10.0 ** (gain / 80.0)
        a = a2 * a2;
        alpha = sin_w0 * 0.5 * np.sqrt(
            (a + 1.0 / a) * (1.0 / slope - 1.0) + 2.0
        )
    else:
        assert False, f"unknown width '{width[0]}'"
    cos_w0 = np.cos(w0)
    cos_2m = -2.0 * cos_w0
    if ft in _basic_filters:
        sin_p2 = 0.5 * sin_w0
        sin_m2 = -0.5 * sin_w0
        cos_1m = 1.0 - cos_w0
        cos_1pm = -(1.0 + cos_w0)
        cos_1m2 = 0.5 - 0.5 * cos_w0
        cos_1p2 = 0.5 + 0.5 * cos_w0
        a_1m = 1.0 - alpha
        a_1p = 1.0 + alpha
        a = [a_1p, cos_2m, a_1m]
        if ft == 'low_pass':
            return ([cos_1m2, cos_1m, cos_1m2], a)
        if ft == 'high_pass':
            return ([cos_1p2, cos_1pm, cos_1p2], a)
        if ft == 'band_pass_q':
            return ([sin_p2, 0.0, sin_m2], a)
        if ft == 'band_pass_c':
            return ([alpha, 0.0, -alpha], a)
        if ft == 'band_notch':
            return ([1.0, cos_2m, 1.0], a)
        if ft == 'all_pass':
            return ([a_1m, cos_2m, a_1p], a)
        assert False, f"internal error: unknown basic filter {ft}"
    a1 = a2 * a2
    if ft == 'peaking':
        return (
            [1.0 + alpha * a1, cos_2m, 1.0 - alpha * a1],
            [1.0 + alpha / a1, cos_2m, 1.0 - alpha / a1],
        )
    if ft in _shelf_filters:
        a1p = a1 + 1.0
        a1m = a1 - 1.0
        a1pc = a1p * cos_w0
        a1mc = a1m * cos_w0
        s2 = 2.0 * a2 * alpha
        if ft == 'low_shelf':
            return (
                [
                    a1 * (a1p - a1mc + s2),
                    2.0 * a1 * (a1m - a1pc),
                    a1 * (a1p - a1mc + s2),
                ],
                [
                    a1p + a1mc + s2,
                    2.0 * (a1m - a1pc),
                    a1p + a1mc - s2,
                ],
            )
        if ft == 'high_shelf':
            return (
                [
                    a1 * (a1p + a1mc + s2),
                    -2.0 * a1 * (a1m + a1pc),
                    a1 * (a1p + a1mc - s2),
                ],
                [
                    a1p - a1mc + s2,
                    2.0 * (a1m - a1pc),
                    a1p - a1mc - s2,
                ],
            )
        assert False, f"internal error: unknown shelf filter {ft}"
    assert False,  f"unknown filter {ft}"

def make_transfer_mag(coeffs):
    def mag_fn(w):
        phi = np.sin(0.5 * w)**2

        def erator(c):
            t1 = 0.5 * (c[0] + c[1] + c[2])
            t2 = -phi * (
                4.0 * c[0] * c[2] * (1.0 - phi) +
                c[1] * (c[0] + c[2])
            )
            return t1 * t1 + t2

        return np.sqrt(erator(coeffs[0]) / erator(coeffs[1]))

    return mag_fn

def make_filter(coeffs):
    b, a = coeffs[0], coeffs[1]
    a_inv = 1.0 / a[0]
    g = b[0] * a_inv
    b = [b[1] * a_inv, b[2] * a_inv]
    a = [a[1] * a_inv, a[2] * a_inv]
    ys = [0.0, 0.0]
    xs = [0.0, 0.0]

    def filter_step(x):
        y = g * x \
            + b[0] * xs[0] + b[1] * xs[1] \
            - a[0] * ys[0] - a[1] * ys[1]
        ys[1] = ys[0]
        ys[0] = y
        xs[1] = xs[0]
        xs[0] = x
        return y

    return filter_step
