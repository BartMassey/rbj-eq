This directory contains a thrown-together Python
implementation of the RBJ equalizer filter code. It is
barely tested.

A filter sweep demo is included, in two forms. `sweep.py`
requires a frequency argument (0..1) and uses the
`make_filter` function in the module to run the filter
sweep. `sweep-scipy.py` uses `scipy.signal.sosfilt` to run
the filter, as a demo of that functionality.
