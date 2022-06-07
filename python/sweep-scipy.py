import rbj_eq, sys
import numpy as np
import scipy.signal as ss

tau = 2 * np.pi

fc = float(sys.argv[1])
cs = rbj_eq.filter_coeffs(
    'peaking',
    fc,
    (
        'slope',
        10.0,
        1.0,
    ),
)
filter = ss.tf2sos(cs[0], cs[1])

# https://en.wikipedia.org/wiki/Chirp#Linear
t = np.linspace(0, 1, 10_000, endpoint=False, dtype=np.float64)
x = np.sin(0.5 * 0.5 * 10_000.0 * tau * t * t)
y = ss.sosfilt(filter, x)

for i, yi in enumerate(y):
    print(i / 10_000.0, round(yi, 6))
