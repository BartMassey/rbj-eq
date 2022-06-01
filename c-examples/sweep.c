#include <stdio.h>
#include <stdlib.h>
#define __USE_XOPEN
#include <math.h>
#include <rbj_eq.h>

#define M_TAU (2.0 * M_PI)

int main(int argc, char **argv) {
    double fc = atof(argv[1]);
    
    filter_coeffs_d_t coeffs =
        filter_coeffs_slope_d(PEAKING_FILTER, fc, 10.0, 1.0);
    filter_coeffs_df1_d_t coeffs_df1 = to_df1_d(&coeffs);
    double state[4] = {0.0, 0.0, 0.0, 0.0};

    for (int i = 0; i < 10000; i++) {
        double t = (double) i / 10000.0;
        double x = sin(0.5 * 0.5 * 10000.0 * M_TAU * t * t);
        double y = filter_df1_d(&coeffs_df1, &state, x);
        printf("%g %g\n", (double) i / 2.0, y);
    }

    return 0;
}
