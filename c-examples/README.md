This C demo shows how to call into `librbj_eq`.

## Build

See `../README.md` for instructions on building and
installing the C library. Edit the `Makefile` in case things
have been installed in nonstandard places.

Once `librbj_eq` is in place, type `make`.

## Run

The demo frequency-sweeps a sine wave through an RBJ
"Peaking Filter". Specify the peak frequency as a fraction
of Nyquist, for example `./sweep 0.1`. The filtered sine
wave will be written to `stdout`. You can view it with
`gnuplot`: use something like

    ./sweep 0.1 >sweep.dat
    echo 'plot "sweep.dat" with lines; pause mouse close;' | gnuplot
