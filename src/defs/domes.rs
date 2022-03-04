use crate::defs::funcs;
use chrono::prelude::*;

pub(crate) fn domes(dat: DateTime<Utc>, lat: f64, lon: f64) -> Vec<f64> {
    let d = funcs::num_date(dat);
    let hour = dat.hour();
    let minute = dat.minute();

    let w_sun = 282.9404 + 4.70935E-5 * d;
    let m_sun = funcs::deg_360(356.0470 + 0.9856002585 * d);
    let l_sun = funcs::deg_360(w_sun + m_sun); // Звездное время
   
    let ra = funcs::local_sid_time(hour, minute, l_sun, lon); // прямое восхождение

    let pv = ra.to_radians();
    let lat = lat.to_radians();
    // e - склонение эклиптики в радианах
    let ecl = 23.4393 - 3.563E-7 * d;
    let e = ecl.to_radians();

    // формула для МС/ASC
    let mut mc = ((pv.tan() / e.cos()).atan()).to_degrees();
    let mut asc = funcs::deg_360(
        (pv.cos() / -(lat.tan() * e.sin() + pv.sin() * e.cos()))
            .atan()
            .to_degrees(),
    );

    // c - вспомогательный угол для куспидов
    let c = (-e.tan() * pv.sin() * lat.tan()).acos();

    let mut ii = ((pv + c / 3.0).cos() / -(lat.tan() * e.sin() + (pv + c / 3.0).sin() * e.cos()))
        .atan()
        .to_degrees();

    let mut iii = ((pv + 2.0 * c / 3.0).cos()
        / -(lat.tan() * e.sin() + (pv + 2.0 * c / 3.0).sin() * e.cos()))
    .atan()
    .to_degrees();

    let mut xi = ((pv - 2.0 * c / 3.0).cos()
        / -(lat.tan() * e.sin() + (pv - 2.0 * c / 3.0).sin() * e.cos()))
    .atan()
    .to_degrees();

    let mut xii = ((pv - c / 3.0).cos() / -(lat.tan() * e.sin() + (pv - c / 3.0).sin() * e.cos()))
        .atan()
        .to_degrees();

    if (mc - pv).abs() > 10.0 {
        mc += 180.0
    }

    let ic = if mc < 180.0 { &mc + 180.0 } else { &mc - 180.0 };

    if asc < 0.0 {
        asc += 180.0
    } else if asc > 360.0 {
        asc -= 360.0;
    }

    if asc > 0.0 && asc < mc {
        asc += 180.0
    }

    let ds = if asc < 180.0 {
        &asc + 180.0
    } else {
        &asc - 180.0
    };

    while ii < asc {
        ii += 180.0
    }
    if ii > 360.0 {
        ii -= 360.0
    }

    let viii = if ii > 180.0 { &ii - 180.0 } else { &ii + 180.0 };

    while iii < asc {
        iii += 180.0
    }
    if iii > 360.0 {
        iii -= 360.0
    }

    let ix = if iii > 180.0 {
        &iii - 180.0
    } else {
        &iii + 180.0
    };

    while xi < mc {
        xi += 180.0
    }
    if xi > 360.0 {
        xi -= 360.0
    }

    let v = if xi > 180.0 { &xi - 180.0 } else { &xi + 180.0 };

    while xii < mc {
        xii += 180.0
    }
    if xii > 360.0 {
        xii -= 360.0
    }

    let vi = if xii > 180.0 {
        &xii - 180.0
    } else {
        &xii + 180.0
    };

    [asc, ii, iii, ic, v, vi, ds, viii, ix, mc, xi, xii].to_vec()
}
