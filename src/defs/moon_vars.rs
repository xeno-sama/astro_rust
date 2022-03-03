use num::complex::Complex64;
use std::collections::HashMap;

pub(crate) fn vars(
    ex: HashMap<i32, HashMap<i32, Complex64>>,
    mut dlam: f64,
    mut ds: f64,
    mut gam1c: f64,
    mut sinpi: f64,
) -> [f64; 4] {
    let mut z = ex[&4][&4];
    dlam += 13.902 * z.im;
    ds += 14.06 * z.im;
    gam1c += -0.001 * z.re;
    sinpi += 0.2607 * z.re;

    z = ex[&3][&4];
    dlam += 0.403 * z.im;
    ds += -4.01 * z.im;
    gam1c += 0.394 * z.re;
    sinpi += 0.0023 * z.re;

    z = ex[&2][&4];
    dlam += 2369.912 * z.im;
    ds += 2373.36 * z.im;
    gam1c += 0.601 * z.re;
    sinpi += 28.2333 * z.re;

    z = ex[&1][&4];
    dlam += -125.154 * z.im;
    ds += -112.79 * z.im;
    gam1c += -0.725 * z.re;
    sinpi += -0.9781 * z.re;

    z = ex[&1][&1] * ex[&4][&4];
    dlam += 1.979 * z.im;
    ds += 6.98 * z.im;
    gam1c += -0.445 * z.re;
    sinpi += 0.0433 * z.re;

    z = ex[&1][&1] * ex[&2][&4];
    dlam += 191.953 * z.im;
    ds += 192.72 * z.im;
    gam1c += 0.029 * z.re;
    sinpi += 3.0861 * z.re;

    z = ex[&1][&1] * ex[&1][&4];
    dlam += -8.466 * z.im;
    ds += -13.51 * z.im;
    gam1c += 0.455 * z.re;
    sinpi += -0.1093 * z.re;

    z = ex[&1][&1];
    dlam += 22639.500 * z.im;
    ds += 22609.07 * z.im;
    gam1c += 0.079 * z.re;
    sinpi += 186.5398 * z.re;

    z = ex[&1][&1] * ex[&-1][&4];
    dlam += 18.609 * z.im;
    ds += 3.59 * z.im;
    gam1c += -0.094 * z.re;
    sinpi += 0.0118 * z.re;

    z = ex[&1][&1] * ex[&-2][&4];
    dlam += -4586.465 * z.im;
    ds += -4578.13 * z.im;
    gam1c += -0.077 * z.re;
    sinpi += 34.3117 * z.re;

    z = ex[&1][&1] * ex[&-3][&4];
    dlam += 3.215 * z.im;
    ds += 5.44 * z.im;
    gam1c += 0.192 * z.re;
    sinpi += -0.0386 * z.re;

    z = ex[&1][&1] * ex[&-4][&4];
    dlam += -38.428 * z.im;
    ds += -38.64 * z.im;
    gam1c += 0.001 * z.re;
    sinpi += 0.6008 * z.re;

    z = ex[&1][&1] * ex[&-6][&4];
    dlam += -0.393 * z.im;
    ds += -1.43 * z.im;
    gam1c += -0.092 * z.re;
    sinpi += 0.0086 * z.re;

    z = ex[&1][&2] * ex[&4][&4];
    dlam += -0.289 * z.im;
    ds += -1.59 * z.im;
    gam1c += 0.123 * z.re;
    sinpi += -0.0053 * z.re;

    z = ex[&1][&2] * ex[&2][&4];
    dlam += -24.420 * z.im;
    ds += -25.10 * z.im;
    gam1c += 0.040 * z.re;
    sinpi += -0.3000 * z.re;

    z = ex[&1][&2] * ex[&1][&4];
    dlam += 18.023 * z.im;
    ds += 17.93 * z.im;
    gam1c += 0.007 * z.re;
    sinpi += 0.1494 * z.re;

    z = ex[&1][&2];
    dlam += -668.146 * z.im;
    ds += -126.98 * z.im;
    gam1c += -1.302 * z.re;
    sinpi += -0.3997 * z.re;

    z = ex[&1][&2] * ex[&-1][&4];
    dlam += 0.560 * z.im;
    ds += 0.32 * z.im;
    gam1c += -0.001 * z.re;
    sinpi += -0.0037 * z.re;

    z = ex[&1][&2] * ex[&-2][&4];
    dlam += -165.145 * z.im;
    ds += -165.06 * z.im;
    gam1c += 0.054 * z.re;
    sinpi += 1.9178 * z.re;

    z = ex[&1][&2] * ex[&-4][&4];
    dlam += -1.877 * z.im;
    ds += -6.46 * z.im;
    gam1c += -0.416 * z.re;
    sinpi += 0.0339 * z.re;

    z = ex[&2][&1] * ex[&4][&4];
    dlam += 0.213 * z.im;
    ds += 1.02 * z.im;
    gam1c += -0.074 * z.re;
    sinpi += 0.0054 * z.re;

    z = ex[&2][&1] * ex[&2][&4];
    dlam += 14.387 * z.im;
    ds += 14.78 * z.im;
    gam1c += -0.017 * z.re;
    sinpi += 0.2833 * z.re;

    z = ex[&2][&1] * ex[&1][&4];
    dlam += -0.586 * z.im;
    ds += -1.20 * z.im;
    gam1c += 0.054 * z.re;
    sinpi += -0.0100 * z.re;

    z = ex[&2][&1];
    dlam += 769.016 * z.im;
    ds += 767.96 * z.im;
    gam1c += 0.107 * z.re;
    sinpi += 10.1657 * z.re;

    z = ex[&2][&1] * ex[&-1][&4];
    dlam += 1.750 * z.im;
    ds += 2.01 * z.im;
    gam1c += -0.018 * z.re;
    sinpi += 0.0155 * z.re;

    z = ex[&2][&1] * ex[&-2][&4];
    dlam += -211.656 * z.im;
    ds += -152.53 * z.im;
    gam1c += 5.679 * z.re;
    sinpi += -0.3039 * z.re;

    z = ex[&2][&1] * ex[&-3][&4];
    dlam += 1.225 * z.im;
    ds += 0.91 * z.im;
    gam1c += -0.030 * z.re;
    sinpi += -0.0088 * z.re;

    z = ex[&2][&1] * ex[&-4][&4];
    dlam += -30.773 * z.im;
    ds += -34.07 * z.im;
    gam1c += -0.308 * z.re;
    sinpi += 0.3722 * z.re;

    z = ex[&2][&1] * ex[&-6][&4];
    dlam += -0.570 * z.im;
    ds += -1.40 * z.im;
    gam1c += -0.074 * z.re;
    sinpi += 0.0109 * z.re;

    z = ex[&1][&1] * ex[&1][&2] * ex[&2][&4];
    dlam += -2.921 * z.im;
    ds += -11.75 * z.im;
    gam1c += 0.787 * z.re;
    sinpi += -0.0484 * z.re;

    z = ex[&1][&1] * ex[&1][&2] * ex[&1][&4];
    dlam += 1.267 * z.im;
    ds += 1.52 * z.im;
    gam1c += -0.022 * z.re;
    sinpi += 0.0164 * z.re;

    z = ex[&1][&1] * ex[&1][&2];
    dlam += -109.673 * z.im;
    ds += -115.18 * z.im;
    gam1c += 0.461 * z.re;
    sinpi += -0.9490 * z.re;

    z = ex[&1][&1] * ex[&1][&2] * ex[&-2][&4];
    dlam += -205.962 * z.im;
    ds += -182.36 * z.im;
    gam1c += 2.056 * z.re;
    sinpi += 1.4437 * z.re;

    z = ex[&1][&1] * ex[&1][&2] * ex[&-3][&4];
    dlam += 0.233 * z.im;
    ds += 0.36 * z.im;
    gam1c += 0.012 * z.re;
    sinpi += -0.0025 * z.re;

    z = ex[&1][&1] * ex[&1][&2] * ex[&-4][&4];
    dlam += -4.391 * z.im;
    ds += -9.66 * z.im;
    gam1c += -0.471 * z.re;
    sinpi += 0.0673 * z.re;

    z = ex[&1][&1] * ex[&-1][&2] * ex[&4][&4];
    dlam += 0.283 * z.im;
    ds += 1.53 * z.im;
    gam1c += -0.111 * z.re;
    sinpi += 0.0060 * z.re;

    z = ex[&1][&1] * ex[&-1][&2] * ex[&2][&4];
    dlam += 14.577 * z.im;
    ds += 31.70 * z.im;
    gam1c += -1.540 * z.re;
    sinpi += 0.2302 * z.re;

    z = ex[&1][&1] * ex[&-1][&2];
    dlam += 147.687 * z.im;
    ds += 138.76 * z.im;
    gam1c += 0.679 * z.re;
    sinpi += 1.1528 * z.re;

    z = ex[&1][&1] * ex[&-1][&2] * ex[&-1][&4];
    dlam += -1.089 * z.im;
    ds += 0.55 * z.im;
    gam1c += 0.021 * z.re;

    z = ex[&1][&1] * ex[&-1][&2] * ex[&-2][&4];
    dlam += 28.475 * z.im;
    ds += 23.59 * z.im;
    gam1c += -0.443 * z.re;
    sinpi += -0.2257 * z.re;

    z = ex[&1][&1] * ex[&-1][&2] * ex[&-3][&4];
    dlam += -0.276 * z.im;
    ds += -0.38 * z.im;
    gam1c += -0.006 * z.re;
    sinpi += -0.0036 * z.re;

    z = ex[&1][&1] * ex[&-1][&2] * ex[&-4][&4];
    dlam += 0.636 * z.im;
    ds += 2.27 * z.im;
    gam1c += 0.146 * z.re;
    sinpi += -0.0102 * z.re;

    z = ex[&2][&2] * ex[&2][&4];
    dlam += -0.189 * z.im;
    ds += -1.68 * z.im;
    gam1c += 0.131 * z.re;
    sinpi += -0.0028 * z.re;

    z = ex[&2][&2];
    dlam += -7.486 * z.im;
    ds += -0.66 * z.im;
    gam1c += -0.037 * z.re;
    sinpi += -0.0086 * z.re;

    z = ex[&2][&2] * ex[&-2][&4];
    dlam += -8.096 * z.im;
    ds += -16.35 * z.im;
    gam1c += -0.740 * z.re;
    sinpi += 0.0918 * z.re;

    z = ex[&2][&3] * ex[&2][&4];
    dlam += -5.741 * z.im;
    ds += -0.04 * z.im;
    sinpi += -0.0009 * z.re;

    z = ex[&2][&3] * ex[&1][&4];
    dlam += 0.255 * z.im;

    z = ex[&2][&3];
    dlam += -411.608 * z.im;
    ds += -0.20 * z.im;
    sinpi += -0.0124 * z.re;

    z = ex[&2][&3] * ex[&-1][&4];
    dlam += 0.584 * z.im;
    ds += 0.84 * z.im;
    sinpi += 0.0071 * z.re;

    z = ex[&2][&3] * ex[&-2][&4];
    dlam += -55.173 * z.im;
    ds += -52.14 * z.im;
    sinpi += -0.1052 * z.re;

    z = ex[&2][&3] * ex[&-3][&4];
    dlam += 0.254 * z.im;
    ds += 0.25 * z.im;
    sinpi += -0.0017 * z.re;

    z = ex[&2][&3] * ex[&-4][&4];
    dlam += 0.025 * z.im;
    ds += -1.67 * z.im;
    sinpi += 0.0031 * z.re;

    z = ex[&3][&1] * ex[&2][&4];
    dlam += 1.060 * z.im;
    ds += 2.96 * z.im;
    gam1c += -0.166 * z.re;
    sinpi += 0.0243 * z.re;

    z = ex[&3][&1];
    dlam += 36.124 * z.im;
    ds += 50.64 * z.im;
    gam1c += -1.300 * z.re;
    sinpi += 0.6215 * z.re;

    z = ex[&3][&1] * ex[&-2][&4];
    dlam += -13.193 * z.im;
    ds += -16.40 * z.im;
    gam1c += 0.258 * z.re;
    sinpi += -0.1187 * z.re;

    z = ex[&3][&1] * ex[&-4][&4];
    dlam += -1.187 * z.im;
    ds += -0.74 * z.im;
    gam1c += 0.042 * z.re;
    sinpi += 0.0074 * z.re;

    z = ex[&3][&1] * ex[&-6][&4];
    dlam += -0.293 * z.im;
    ds += -0.31 * z.im;
    gam1c += -0.002 * z.re;
    sinpi += 0.0046 * z.re;

    z = ex[&2][&1] * ex[&1][&2] * ex[&2][&4];
    dlam += -0.290 * z.im;
    ds += -1.45 * z.im;
    gam1c += 0.116 * z.re;
    sinpi += -0.0051 * z.re;

    z = ex[&2][&1] * ex[&1][&2];
    dlam += -7.649 * z.im;
    ds += -10.56 * z.im;
    gam1c += 0.259 * z.re;
    sinpi += -0.1038 * z.re;

    z = ex[&2][&1] * ex[&1][&2] * ex[&-2][&4];
    dlam += -8.627 * z.im;
    ds += -7.59 * z.im;
    gam1c += 0.078 * z.re;
    sinpi += -0.0192 * z.re;

    z = ex[&2][&1] * ex[&1][&2] * ex[&-4][&4];
    dlam += -2.740 * z.im;
    ds += -2.54 * z.im;
    gam1c += 0.022 * z.re;
    sinpi += 0.0324 * z.re;

    z = ex[&2][&1] * ex[&-1][&2] * ex[&2][&4];
    dlam += 1.181 * z.im;
    ds += 3.32 * z.im;
    gam1c += -0.212 * z.re;
    sinpi += 0.0213 * z.re;

    z = ex[&2][&1] * ex[&-1][&2];
    dlam += 9.703 * z.im;
    ds += 11.67 * z.im;
    gam1c += -0.151 * z.re;
    sinpi += 0.1268 * z.re;

    z = ex[&2][&1] * ex[&-1][&2] * ex[&-1][&4];
    dlam += -0.352 * z.im;
    ds += -0.37 * z.im;
    gam1c += 0.001 * z.re;
    sinpi += -0.0028 * z.re;

    z = ex[&2][&1] * ex[&-1][&2] * ex[&-2][&4];
    dlam += -2.494 * z.im;
    ds += -1.17 * z.im;
    gam1c += -0.003 * z.re;
    sinpi += -0.0017 * z.re;

    z = ex[&2][&1] * ex[&-1][&2] * ex[&-4][&4];
    dlam += 0.360 * z.im;
    ds += 0.20 * z.im;
    gam1c += -0.012 * z.re;
    sinpi += -0.0043 * z.re;

    z = ex[&1][&1] * ex[&2][&2];
    dlam += -1.167 * z.im;
    ds += -1.25 * z.im;
    gam1c += 0.008 * z.re;
    sinpi += -0.0106 * z.re;

    z = ex[&1][&1] * ex[&2][&2] * ex[&-2][&4];
    dlam += -7.412 * z.im;
    ds += -6.12 * z.im;
    gam1c += 0.117 * z.re;
    sinpi += 0.0484 * z.re;

    z = ex[&1][&1] * ex[&2][&2] * ex[&-4][&4];
    dlam += -0.311 * z.im;
    ds += -0.65 * z.im;
    gam1c += -0.032 * z.re;
    sinpi += 0.0044 * z.re;

    z = ex[&1][&1] * ex[&-2][&2] * ex[&2][&4];
    dlam += 0.757 * z.im;
    ds += 1.82 * z.im;
    gam1c += -0.105 * z.re;
    sinpi += 0.0112 * z.re;

    z = ex[&1][&1] * ex[&-2][&2];
    dlam += 2.580 * z.im;
    ds += 2.32 * z.im;
    gam1c += 0.027 * z.re;
    sinpi += 0.0196 * z.re;

    z = ex[&1][&1] * ex[&-2][&2] * ex[&-2][&4];
    dlam += 2.533 * z.im;
    ds += 2.40 * z.im;
    gam1c += -0.014 * z.re;
    sinpi += -0.0212 * z.re;

    z = ex[&3][&2] * ex[&-2][&4];
    dlam += -0.344 * z.im;
    ds += -0.57 * z.im;
    gam1c += -0.025 * z.re;
    sinpi += 0.0036 * z.re;

    z = ex[&1][&1] * ex[&2][&3] * ex[&2][&4];
    dlam += -0.992 * z.im;
    ds += -0.02 * z.im;

    z = ex[&1][&1] * ex[&2][&3];
    dlam += -45.099 * z.im;
    ds += -0.02 * z.im;
    sinpi += -0.0010 * z.re;

    z = ex[&1][&1] * ex[&2][&3] * ex[&-2][&4];
    dlam += -0.179 * z.im;
    ds += -9.52 * z.im;
    sinpi += -0.0833 * z.re;

    z = ex[&1][&1] * ex[&2][&3] * ex[&-4][&4];
    dlam += -0.301 * z.im;
    ds += -0.33 * z.im;
    sinpi += 0.0014 * z.re;

    z = ex[&1][&1] * ex[&-2][&3] * ex[&2][&4];
    dlam += -6.382 * z.im;
    ds += -3.37 * z.im;
    sinpi += -0.0481 * z.re;

    z = ex[&1][&1] * ex[&-2][&3];
    dlam += 39.528 * z.im;
    ds += 85.13 * z.im;
    sinpi += -0.7136 * z.re;

    z = ex[&1][&1] * ex[&-2][&3] * ex[&-2][&4];
    dlam += 9.366 * z.im;
    ds += 0.71 * z.im;
    sinpi += -0.0112 * z.re;

    z = ex[&1][&1] * ex[&-2][&3] * ex[&-4][&4];
    dlam += 0.202 * z.im;
    ds += 0.02 * z.im;

    z = ex[&1][&2] * ex[&2][&3];
    dlam += 0.415 * z.im;
    ds += 0.10 * z.im;
    sinpi += 0.0013 * z.re;

    z = ex[&1][&2] * ex[&2][&3] * ex[&-2][&4];
    dlam += -2.152 * z.im;
    ds += -2.26 * z.im;
    sinpi += -0.0066 * z.re;

    z = ex[&1][&2] * ex[&-2][&3] * ex[&2][&4];
    dlam += -1.440 * z.im;
    ds += -1.30 * z.im;
    sinpi += 0.0014 * z.re;

    z = ex[&1][&2] * ex[&-2][&3] * ex[&-2][&4];
    dlam += 0.384 * z.im;
    ds += -0.04 * z.im;

    z = ex[&4][&1];
    dlam += 1.938 * z.im;
    ds += 3.60 * z.im;
    gam1c += -0.145 * z.re;
    sinpi += 0.0401 * z.re;

    z = ex[&4][&1] * ex[&-2][&4];
    dlam += -0.952 * z.im;
    ds += -1.58 * z.im;
    gam1c += 0.052 * z.re;
    sinpi += -0.0130 * z.re;

    z = ex[&3][&1] * ex[&1][&2];
    dlam += -0.551 * z.im;
    ds += -0.94 * z.im;
    gam1c += 0.032 * z.re;
    sinpi += -0.0097 * z.re;

    z = ex[&3][&1] * ex[&1][&2] * ex[&-2][&4];
    dlam += -0.482 * z.im;
    ds += -0.57 * z.im;
    gam1c += 0.005 * z.re;
    sinpi += -0.0045 * z.re;

    z = ex[&3][&1] * ex[&-1][&2];
    dlam += 0.681 * z.im;
    ds += 0.96 * z.im;
    gam1c += -0.026 * z.re;
    sinpi += 0.0115 * z.re;

    z = ex[&2][&1] * ex[&2][&2] * ex[&-2][&4];
    dlam += -0.297 * z.im;
    ds += -0.27 * z.im;
    gam1c += 0.002 * z.re;
    sinpi += -0.0009 * z.re;

    z = ex[&2][&1] * ex[&-2][&2] * ex[&-2][&4];
    dlam += 0.254 * z.im;
    ds += 0.21 * z.im;
    gam1c += -0.003 * z.re;

    z = ex[&1][&1] * ex[&3][&2] * ex[&-2][&4];
    dlam += -0.250 * z.im;
    ds += -0.22 * z.im;
    gam1c += 0.004 * z.re;
    sinpi += 0.0014 * z.re;

    z = ex[&2][&1] * ex[&2][&3];
    dlam += -3.996 * z.im;
    sinpi += 0.0004 * z.re;

    z = ex[&2][&1] * ex[&2][&3] * ex[&-2][&4];
    dlam += 0.557 * z.im;
    ds += -0.75 * z.im;
    sinpi += -0.0090 * z.re;

    z = ex[&2][&1] * ex[&-2][&3] * ex[&2][&4];
    dlam += -0.459 * z.im;
    ds += -0.38 * z.im;
    sinpi += -0.0053 * z.re;

    z = ex[&2][&1] * ex[&-2][&3];
    dlam += -1.298 * z.im;
    ds += 0.74 * z.im;
    sinpi += 0.0004 * z.re;

    z = ex[&2][&1] * ex[&-2][&3] * ex[&-2][&4];
    dlam += 0.538 * z.im;
    ds += 1.14 * z.im;
    sinpi += -0.0141 * z.re;

    z = ex[&1][&1] * ex[&1][&2] * ex[&2][&3];
    dlam += 0.263 * z.im;
    ds += 0.02 * z.im;

    z = ex[&1][&1] * ex[&1][&2] * ex[&-2][&3] * ex[&-2][&4];
    dlam += 0.426 * z.im;
    ds += 0.07 * z.im;
    sinpi += -0.0006 * z.re;

    z = ex[&1][&1] * ex[&-1][&2] * ex[&2][&3];
    dlam += -0.304 * z.im;
    ds += 0.03 * z.im;
    sinpi += 0.0003 * z.re;

    z = ex[&1][&1] * ex[&-1][&2] * ex[&-2][&3] * ex[&2][&4];
    dlam += -0.372 * z.im;
    ds += -0.19 * z.im;
    sinpi += -0.0027 * z.re;

    z = ex[&4][&3];
    dlam += 0.418 * z.im;

    z = ex[&3][&1] * ex[&2][&3];
    dlam += -0.330 * z.im;
    ds += -0.04 * z.im;

    [dlam, ds, gam1c, sinpi]
}
