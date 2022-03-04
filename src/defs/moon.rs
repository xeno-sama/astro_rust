use crate::defs::consts;
// use crate::defs::funcs;
use crate::defs::moon_vars;
use crate::defs::structs;
use num::complex::Complex;
use num::complex::Complex64;
use std::collections::HashMap;
use std::f64::consts::PI;
//
fn sine(phi: f64) -> f64 {
    (consts::PI2 * phi).sin()
}
//
fn frac(x: f64) -> f64 {
    x - x.floor()
}
//
fn arr2(xmin: i32, xmax: i32, ymin: i32, ymax: i32) -> HashMap<i32, HashMap<i32, Complex64>> {
    let mut _ar1: HashMap<i32, Complex64> = HashMap::new();
    let mut _ar2: HashMap<i32, HashMap<i32, Complex64>> = HashMap::new();

    fn arr1(xmin: i32, xmax: i32, mut _ar1: HashMap<i32, Complex64>) -> HashMap<i32, Complex64> {
        for i in xmin..=xmax {
            _ar1.insert(i, Complex::new(0.0, 0.0));
        }
        _ar1
    }
    for i in xmin..=xmax {
        _ar2.insert(i, arr1(ymin, ymax, HashMap::new()));
    }
    _ar2
}
//
fn calc_moon(time: f64) -> structs::MoonPos {
    // let mut ex = arr2(-6, 6, 1, 4);
    let mut ex = arr2(-6, 6, 1, 4);

    let t = time / 36525.0;
    let t2 = t * t;
    let mut dlam = 0.0;
    let mut ds = 0.0;
    let mut gam1c = 0.0;
    let mut sinpi = 3422.7000;

    let s1 = sine(0.19833 + 0.05611 * t);
    let s2 = sine(0.27869 + 0.04508 * t);
    let s3 = sine(0.16827 - 0.36903 * t);
    let s4 = sine(0.34734 - 5.37261 * t);
    let s5 = sine(0.10498 - 5.37899 * t);
    let s6 = sine(0.42681 - 0.41855 * t);
    let s7 = sine(0.14943 - 5.37511 * t);
    let dl0 = 0.84 * s1 + 0.31 * s2 + 14.27 * s3 + 7.26 * s4 + 0.28 * s5 + 0.24 * s6;
    let dl = 2.94 * s1 + 0.31 * s2 + 14.27 * s3 + 9.34 * s4 + 1.12 * s5 + 0.83 * s6;
    let dls = -6.40 * s1 - 1.89 * s6;
    let df = 0.21 * s1 + 0.31 * s2 + 14.27 * s3 - 88.70 * s4 - 15.30 * s5 + 0.24 * s6 - 1.86 * s7;
    let dd = dl0 - dls;
    let dgam = -3332E-9 * sine(0.59734 - 5.37261 * t)
        - 539E-9 * sine(0.35498 - 5.37899 * t)
        - 64E-9 * sine(0.39943 - 5.37511 * t);

    let l0 =
        consts::PI2 * frac(0.60643382 + 1336.85522467 * t - 0.00000313 * t2) + dl0 / consts::ARC;
    let l = consts::PI2 * frac(0.37489701 + 1325.55240982 * t + 0.00002565 * t2) + dl / consts::ARC;
    let ls = consts::PI2 * frac(0.99312619 + 99.99735956 * t - 0.00000044 * t2) + dls / consts::ARC;
    let f = consts::PI2 * frac(0.25909118 + 1342.22782980 * t - 0.00000892 * t2) + df / consts::ARC;
    let d = consts::PI2 * frac(0.82736186 + 1236.85308708 * t - 0.00000397 * t2) + dd / consts::ARC;

    let (mut i, mut j, mut _max) = (0, 0, 0);
    let mut fac = 0.0;
    let mut arg = 0.0;

    let mut x0: HashMap<i32, Complex64> = HashMap::new();
    let mut x1: HashMap<i32, Complex64> = HashMap::new();

    while i <= 4 {
        if i == 1 {
            arg = l;
            _max = 4;
            fac = 1.000002208;
        } else if i == 2 {
            arg = ls;
            _max = 3;
            fac = 0.997504612 - 0.002495388 * t;
        } else if i == 3 {
            arg = f;
            _max = 4;
            fac = 1.000002708 + 139.978 * dgam;
        } else {
            arg = d;
            _max = 6;
            fac = 1.0;
        }

        x0.insert(i, Complex::new(0.0, 0.0));
        x1.insert(
            i,
            Complex {
                re: arg.cos() * fac,
                im: arg.sin() * fac,
            },
        );
        i += 1
    }
    ex.insert(0, x0);
    ex.insert(1, x1);

    // println!("{:?} \n\n", ex);
    let mut map: HashMap<i32, HashMap<i32, Complex64>> = HashMap::new();
    let mut _map: HashMap<i32, Complex64> = HashMap::new();
    //
    i = 0;
    j = 1;
    while j <= 4 {
        _map.entry(j).or_insert(1.0 + ex[&i][&j]);
        j += 1;
    }
    map.entry(i).or_insert(_map);
    _map = HashMap::new();

    i = 1;
    j = 1;
    while j <= 4 {
        _map.entry(j).or_insert(ex[&i][&j]);
        // println!("{:?}", tep[&j]);
        j += 1;
    }
    map.entry(i).or_insert(_map);
    _map = HashMap::new();

    i = 2;
    j = 1;
    while j <= 4 {
        _map.entry(j).or_insert(ex[&(i - 1)][&j] * ex[&1][&j]);
        // println!("{:?}", tep[&j]);
        j += 1;
    }
    map.entry(i).or_insert(_map);
    _map = HashMap::new();

    i = 3;
    let mut j = 1;
    while j <= 4 {
        _map.entry(j).or_insert(map[&(i - 1)][&j] * ex[&1][&j]);
        // println!("{:?}", tep[&j]);
        j += 1;
    }
    map.entry(i).or_insert(_map);
    _map = HashMap::new();

    i = 4;
    j = 1;
    while j <= 4 {
        _map.entry(j).or_insert(map[&(i - 1)][&j] * ex[&1][&j]);
        // println!("{:?}", tep[&j]);
        j += 1;
    }
    map.entry(i).or_insert(_map);
    _map = HashMap::new();

    i = 5;
    let mut j = 1;
    while j <= 4 {
        _map.entry(j).or_insert(map[&(i - 1)][&j] * ex[&1][&j]);
        // println!("{:?}", tep[&j]);
        j += 1;
    }
    map.entry(i).or_insert(_map);
    _map = HashMap::new();

    i = 6;
    j = 1;
    while j <= 4 {
        _map.entry(j).or_insert(map[&(i - 1)][&j] * ex[&1][&j]);
        // println!("{:?}", tep[&j]);
        j += 1;
    }
    map.entry(i).or_insert(_map);
    _map = HashMap::new();
    //
    i = -6;
    j = 1;
    while j <= 4 {
        _map.entry(j).or_insert((map[&-i][&j]).conj());
        j += 1;
    }
    map.entry(i).or_insert(_map);
    _map = HashMap::new();

    i = -5;
    j = 1;
    while j <= 4 {
        _map.entry(j).or_insert((map[&-i][&j]).conj());
        // println!("{:?}", tep[&j]);
        j += 1;
    }
    map.entry(i).or_insert(_map);
    _map = HashMap::new();

    i = -4;
    j = 1;
    while j <= 4 {
        _map.entry(j).or_insert((map[&-i][&j]).conj());
        j += 1;
    }
    map.entry(i).or_insert(_map);
    _map = HashMap::new();

    i = -3;
    j = 1;
    while j <= 4 {
        _map.entry(j).or_insert((map[&-i][&j]).conj());
        j += 1;
    }
    map.entry(i).or_insert(_map);
    _map = HashMap::new();

    i = -2;
    j = 1;
    while j <= 4 {
        _map.entry(j).or_insert((map[&-i][&j]).conj());
        j += 1;
    }
    map.entry(i).or_insert(_map);
    _map = HashMap::new();

    i = -1;
    j = 1;
    while j <= 4 {
        _map.entry(j).or_insert((ex[&-i][&j]).conj());
        j += 1;
    }
    map.entry(i).or_insert(_map);

    // println!("{:?} \n\n", map);

    // расчет переменных через обьединение в отдельный файл vars
    let _vars = moon_vars::vars(map, dlam, ds, gam1c, sinpi);
    dlam = _vars[0];
    ds = _vars[1];
    gam1c = _vars[2];
    sinpi = _vars[3];

    dlam += 0.82 * sine(0.7736 - 62.5512 * t)
        + 0.31 * sine(0.0466 - 125.1025 * t)
        + 0.35 * sine(0.5785 - 25.1042 * t)
        + 0.66 * sine(0.4591 + 1335.8075 * t)
        + 0.64 * sine(0.3130 - 91.5680 * t)
        + 1.14 * sine(0.1480 + 1331.2898 * t)
        + 0.21 * sine(0.5918 + 1056.5859 * t)
        + 0.44 * sine(0.5784 + 1322.8595 * t)
        + 0.24 * sine(0.2275 - 5.7374 * t)
        + 0.28 * sine(0.2965 + 2.6929 * t)
        + 0.33 * sine(0.3132 + 6.3368 * t);

    let s = f + ds / consts::ARC;
    let n = 0.0;
    let lat_seconds = (1.000002708 + 139.978 * dgam) * (18518.511 + 1.189 + gam1c) * (s).sin()
        - 6.24 * (3.0 * s).sin()
        + n;

    let fin1 = consts::PI2 * frac((l0 + dlam / consts::ARC) / consts::PI2);
    let fin2 = PI / (180.0 * 3600.0) * lat_seconds;
    let fin3 = consts::ARC * consts::EARTH_EQUATORIAL_RADIUS_AU / (0.999953253 * sinpi);

    structs::MoonPos {
        lon: fin1,
        lat: fin2,
        dist: fin3,
    }
}

pub(crate) fn geo_moon(time: f64) -> f64 {
    //Вычисление геоцентрической позиции Луны
    let mp = calc_moon(time);
    mp.lon.to_degrees()
}

// pub(crate) fn geo_moon(time: f64) -> f64 {
//     //Вычисление геоцентрической позиции Луны
//     let mp = calc_moon(time);

//     // Преобразование геоцентрических эклиптических сферических координат в прямоугольные/Cartesian
//     let dist_cos_lat = mp.dist * mp.lat.cos();
//     let gepos = [
//         dist_cos_lat * mp.lon.cos(),
//         dist_cos_lat * mp.lon.sin(),
//         mp.dist * mp.lat.sin(),
//     ];

//     // Преобразование эклиптических координат в экваториальные
//     let mpos1 = funcs::ecl2_equ_vec(time, gepos.to_vec());

//     // Convert from mean equinox of date to J2000
//     let mpos2 = funcs::precession(mpos1, time, consts::PD.into2000);

//     // return Vector { x: mpos2[0], y: mpos2[1], z: mpos2[2], t: time }
//     mp.lon.to_degrees()
// }