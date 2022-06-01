import rbj_eq, sys
import numpy as np

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
filter = rbj_eq.make_filter(cs)

for i in range(10_000):
    # https://en.wikipedia.org/wiki/Chirp#Linear
    t = i / 10_000.0
    x = np.sin(0.5 * 0.5 * 10_000.0 * tau * t * t)
    y = filter(x)
    print(i / 2, y)
