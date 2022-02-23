// #![allow(unused_variables, unused_assignments, unused_imports, dead_code)]
use crate::defs::consts::*;
use crate::defs::structs::*;
use chrono::prelude::*;
//
pub(crate) fn fmod(x: f64, y: f64) -> f64 {
    x - (x / y).trunc() * y
}
//
pub(crate) fn deg_360(grad_in: f64) -> f64 {
    let mut grad_out = grad_in - (grad_in / 360.0).trunc() * 360.0;
    if grad_out < 0.0 {
        grad_out += 360.0
    };
    grad_out
}
//
pub(crate) fn time_24(t0: f64) -> f64 {
    let mut time24 = t0 - (t0 / 24.0).trunc() * 24.0;
    if time24 < 0.0 {
        time24 += 24.0
    };
    time24
}
//
pub(crate) fn time_ra(ra: f64) -> Vec<i32> {
    let h_ra = (ra / 15.0) as i32;
    let m_ra = ((ra / 15.0 - (h_ra as f64)) * 60.0) as i32;
    let s_ra = (((ra / 15.0 - (h_ra as f64)) * 60.0) % 1.0 * 60.0) as i32;
    [h_ra, m_ra, s_ra].to_vec()
}
//
pub(crate) fn num_date(dat: DateTime<Utc>) -> f64 {
    let m1 = 7 * (dat.year() + (dat.month() as i32 + 9) / 12) / 4;
    let m2 = 3 * ((dat.year() + (dat.month() as i32 - 9) / 7) / 100 + 1) / 4;
    let m3 = 275 * (dat.month() as i32) / 9;
    let ut = dat.hour() as f64 + dat.minute() as f64 / 60.0;
    let _d = 367 * dat.year() - m1 - m2 + m3 + (dat.day() as i32) - 730515;
    _d as f64 + ut / 24.0
}
//
pub(crate) fn local_sid_time(hour: u32, minute: u32, lst: f64, lon: f64) -> f64 {
    let ut = hour as f64 + (minute as f64 / 60.0);
    let gmst = time_24(lst / 15.0 + 12.0);
    let sid_time = gmst + ut + lon / 15.0;
    let sid_time_deg = sid_time * 15.0;
    sid_time_deg
}
//
pub(crate) fn sidereal_time(time: f64) -> f64 {
    let t = time / 36525.0;
    let theta = era(time);
    let st = 0.014506
        + ((((-0.0000000368 * t - 0.000029956) * t - 0.00000044) * t + 1.3915817) * t
            + 4612.156534)
            * t;
    let gst = fmod(st / 3600.0 + theta, 360.0) / 15.0;
    if gst < 0.0 {
        gst + 24.0
    } else {
        gst
    }
}
// Earth Rotation Angle
pub(crate) fn era(time: f64) -> f64 {
    let thet1 = 0.7790572732640 + 0.00273781191135448 * time;
    let thet3 = fmod(time, 1.0);
    let theta = 360.0 * fmod(thet1 + thet3, 1.0);
    if theta < 0.0 {
        theta + 360.0
    } else {
        theta
    }
}
//
pub(crate) fn terra_posvel(observer: Observer, st: f64) -> [f64; 3] {
    let df2 = EARTH_FLATTENING * EARTH_FLATTENING;
    let phi = (observer.latitude).to_radians();
    let sinphi = (phi).sin();
    let cosphi = (phi).cos();
    let c = 1.0 / (cosphi * cosphi + df2 * sinphi * sinphi).sqrt();
    let s = df2 * c;
    let ach = EARTH_EQUATORIAL_RADIUS_KM * c;
    let ash = EARTH_EQUATORIAL_RADIUS_KM * s;
    let stlocl = (15.0 * st + observer.longitude).to_radians();
    let sinst = (stlocl).sin();
    let cosst = (stlocl).cos();
    [
        ach * cosphi * cosst / KM_PER_AU,
        ach * cosphi * sinst / KM_PER_AU,
        ash * sinphi / KM_PER_AU,
    ]
}
//
pub(crate) fn terra(observer: Observer, st: f64) -> [f64; 3] {
    terra_posvel(observer, st)
}
//
pub(crate) fn precession_rot(time: f64, direction: &str) -> RotationMatrix {
    let mut eps0 = 84381.406;
    let t = time / DAYS_PER_MILLENNIUM;

    let mut psia = ((((-0.0000000951 * t + 0.000132851) * t - 0.00114045) * t - 1.0790069) * t
        + 5038.481507)
        * t;

    let mut omegaa =
        ((((0.0000003337 * t - 0.000000467) * t - 0.00772503) * t + 0.0512623) * t - 0.025754) * t
            + eps0;

    let mut chia = ((((-0.0000000560 * t + 0.000170663) * t - 0.00121197) * t - 2.3814292) * t
        + 10.556403)
        * t;

    eps0 *= ASEC2RAD;
    psia *= ASEC2RAD;
    omegaa *= ASEC2RAD;
    chia *= ASEC2RAD;

    let sa = eps0.sin();
    let ca = eps0.cos();
    let sb = -psia.sin();
    let cb = -psia.cos();
    let sc = -omegaa.sin();
    let cc = -omegaa.cos();
    let sd = chia.sin();
    let cd = chia.cos();

    let xx = cd * cb - sb * sd * cc;
    let yx = cd * sb * ca + sd * cc * cb * ca - sa * sd * sc;
    let zx = cd * sb * sa + sd * cc * cb * sa + ca * sd * sc;
    let xy = -sd * cb - sb * cd * cc;
    let yy = -sd * sb * ca + cd * cc * cb * ca - sa * cd * sc;
    let zy = -sd * sb * sa + cd * cc * cb * sa + ca * cd * sc;
    let xz = sb * sc;
    let yz = -sc * cb * ca - sa * cc;
    let zz = -sc * cb * sa + cc * ca;

    if direction == PD.into2000 {
        RotationMatrix {
            rot: [[xx, yx, zx], [xy, yy, zy], [xz, yz, zz]],
        }
    } else {
        RotationMatrix {
            rot: [[xx, xy, xz], [yx, yy, yz], [zx, zy, zz]],
        }
    }
}

pub(crate) fn mean_oblig(time: f64) -> f64 {
    let t = time / DAYS_PER_MILLENNIUM;
    let asec = ((((-0.0000000434 * t - 0.000000576) * t + 0.00200340) * t - 0.0001831) * t
        - 46.836769)
        * t
        + 84381.406;
    asec
}

pub(crate) fn ecl2_equ_vec(time: f64, ecl: Vec<f64>) -> [f64; 3] {
    let obl = mean_oblig(time).to_radians();
    let obl_cos = obl.cos();
    let obl_sin = obl.sin();
    [
        ecl[0],
        ecl[1] * obl_cos - ecl[2] * obl_sin,
        ecl[2] * obl_sin + ecl[2] * obl_cos,
    ]
}
//
pub(crate) fn precession(pos: [f64; 3], time: f64, direction: &str) -> [f64; 3] {
    let r = precession_rot(time, direction);
    rotate(r, pos)
}
//
pub(crate) fn nutation(pos: [f64; 3], time: f64, direction: &str) -> [f64; 3] {
    let r = nutation_rot(time, direction);
    rotate(r, pos)
}
// разобраться с конфликтом RotationMatrix и [f64; 3]
pub(crate) fn rotate(rot: RotationMatrix, vec: [f64; 3]) -> [f64; 3] {
    [
        rot.rot[0][0] * vec[0] + rot.rot[1][0] * vec[1] + rot.rot[2][0] * vec[2],
        rot.rot[0][1] * vec[0] + rot.rot[1][1] * vec[1] + rot.rot[2][1] * vec[2],
        rot.rot[0][2] * vec[0] + rot.rot[1][2] * vec[1] + rot.rot[2][2] * vec[2],
    ]
}
//
pub(crate) fn nutation_rot(time: f64, direction: &str) -> RotationMatrix {
    let tilt = iau2000b(time);

    let oblm = (tilt.mobl).to_radians();
    let oblt = (tilt.tobl).to_radians();
    let psi = tilt.dpsi * ASEC2RAD;
    let cobm = (oblm).cos();
    let sobm = (oblm).sin();
    let cobt = (oblt).cos();
    let sobt = (oblt).sin();
    let cpsi = (psi).cos();
    let spsi = (psi).sin();

    let xx = cpsi;
    let yx = -spsi * cobm;
    let zx = -spsi * sobm;
    let xy = spsi * cobt;
    let yy = cpsi * cobm * cobt + sobm * sobt;
    let zy = cpsi * sobm * cobt - cobm * sobt;
    let xz = spsi * sobt;
    let yz = cpsi * cobm * sobt - sobm * cobt;
    let zz = cpsi * sobm * sobt + cobm * cobt;

    if direction == PD.from2000
    // convert J2000 to of-date
    {
        RotationMatrix {
            rot: [[xx, xy, xz], [yx, yy, yz], [zx, zy, zz]],
        }
    } else
    // direction == PrecessDir.into2000 convert of-date to J2000
    {
        RotationMatrix {
            rot: [[xx, yx, zx], [xy, yy, zy], [xz, yz, zz]],
        }
    }
}
//
pub(crate) fn geo_pos(time: f64, observer: Observer) -> [f64; 3] {
    let gast = sidereal_time(time);
    let pos1 = terra(observer, gast);
    let pos2 = nutation(pos1, time, PD.into2000);
    let outpos = precession(pos2, time, PD.into2000);
    // print("nutation $pos2");
    outpos
}
//
pub(crate) fn iau2000b(time: f64) -> Etilt {
    //   double t, el, elp, f, d, om, dp, de, arg, sarg, carg, dpsi, deps;
    let t = time / 36525.0;
    let el = fmod(485868.249036 + t * 1717915923.2178, ASEC360) * ASEC2RAD;
    let elp = fmod(1287104.79305 + t * 129596581.0481, ASEC360) * ASEC2RAD;
    let f = fmod(335779.526232 + t * 1739527262.8478, ASEC360) * ASEC2RAD;
    let d = fmod(1072260.70369 + t * 1602961601.2090, ASEC360) * ASEC2RAD;
    let om = fmod(450160.398036 - t * 6962890.5431, ASEC360) * ASEC2RAD;
    let mut dp = 0.0;
    let mut de = 0.0;

    let mut arg = 2.0 * f - 2.0 * d + 2.0 * om;
    let mut sarg = (om).sin();
    let mut carg = (om).cos();

    let mut dpsi = -0.000135 + (dp * 1.0e-7);
    let mut deps = 0.000388 + (de * 1.0e-7);

    dp = (-172064161.0 - 174666.0 * t) * sarg + 33386.0 * carg;
    de += (92052331.0 + 9086.0 * t) * carg + 15377.0 * sarg;

    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-13170906.0 - 1675.0 * t) * sarg - 13696.0 * carg;
    de += (5730336.0 - 3015.0 * t) * carg - 4587.0 * sarg;

    arg = 2.0 * f + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-2276413.0 - 234.0 * t) * sarg + 2796.0 * carg;
    de += (978459.0 - 485.0 * t) * carg + 1374.0 * sarg;

    arg = 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (2074554.0 + 207.0 * t) * sarg - 698.0 * carg;
    de += (-897492.0 + 470.0 * t) * carg - 291.0 * sarg;

    sarg = (elp).sin();
    carg = (elp).cos();
    dp += (1475877.0 - 3633.0 * t) * sarg + 11817.0 * carg;
    de += (73871.0 - 184.0 * t) * carg - 1924.0 * sarg;

    arg = elp + 2.0 * f - 2.0 * d + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-516821.0 + 1226.0 * t) * sarg - 524.0 * carg;
    de += (224386.0 - 677.0 * t) * carg - 174.0 * sarg;

    sarg = (el).sin();
    carg = (el).cos();
    dp += (711159.0 + 73.0 * t) * sarg - 872.0 * carg;
    de += (-6750.0) * carg + 358.0 * sarg;

    arg = 2.0 * f + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-387298.0 - 367.0 * t) * sarg + 380.0 * carg;
    de += (200728.0 + 18.0 * t) * carg + 318.0 * sarg;

    arg = el + 2.0 * f + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-301461.0 - 36.0 * t) * sarg + 816.0 * carg;
    de += (129025.0 - 63.0 * t) * carg + 367.0 * sarg;

    arg = -elp + 2.0 * f - 2.0 * d + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (215829.0 - 494.0 * t) * sarg + 111.0 * carg;
    de += (-95929.0 + 299.0 * t) * carg + 132.0 * sarg;

    arg = 2.0 * f - 2.0 * d + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (128227.0 + 137.0 * t) * sarg + 181.0 * carg;
    de += (-68982.0 - 9.0 * t) * carg + 39.0 * sarg;

    arg = -el + 2.0 * f + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (123457.0 + 11.0 * t) * sarg + 19.0 * carg;
    de += (-53311.0 + 32.0 * t) * carg - 4.0 * sarg;

    arg = -el + 2.0 * d;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (156994.0 + 10.0 * t) * sarg - 168.0 * carg;
    de += (-1235.0) * carg + 82.0 * sarg;

    arg = el + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (63110.0 + 63.0 * t) * sarg + 27.0 * carg;
    de += (-33228.0) * carg - 9.0 * sarg;

    arg = -el + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-57976.0 - 63.0 * t) * sarg - 189.0 * carg;
    de += (31429.0) * carg - 75.0 * sarg;

    arg = -el + 2.0 * f + 2.0 * d + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-59641.0 - 11.0 * t) * sarg + 149.0 * carg;
    de += (25543.0 - 11.0 * t) * carg + 66.0 * sarg;

    arg = el + 2.0 * f + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-51613.0 - 42.0 * t) * sarg + 129.0 * carg;
    de += (26366.0) * carg + 78.0 * sarg;

    arg = -2.0 * el + 2.0 * f + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (45893.0 + 50.0 * t) * sarg + 31.0 * carg;
    de += (-24236.0 - 10.0 * t) * carg + 20.0 * sarg;

    arg = 2.0 * d;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (63384.0 + 11.0 * t) * sarg - 150.0 * carg;
    de += (-1220.0) * carg + 29.0 * sarg;

    arg = 2.0 * f + 2.0 * d + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-38571.0 - 1.0 * t) * sarg + 158.0 * carg;
    de += (16452.0 - 11.0 * t) * carg + 68.0 * sarg;

    arg = -2.0 * elp + 2.0 * f - 2.0 * d + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (32481.0) * sarg;
    de += (-13870.0) * carg;

    arg = -2.0 * el + 2.0 * d;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-47722.0) * sarg - 18.0 * carg;
    de += (477.0) * carg - 25.0 * sarg;

    arg = 2.0 * el + 2.0 * f + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-31046.0 - 1.0 * t) * sarg + 131.0 * carg;
    de += (13238.0 - 11.0 * t) * carg + 59.0 * sarg;

    arg = el + 2.0 * f - 2.0 * d + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (28593.0) * sarg - carg;
    de += (-12338.0 + 10.0 * t) * carg - 3.0 * sarg;

    arg = -el + 2.0 * f + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (20441.0 + 21.0 * t) * sarg + 10.0 * carg;
    de += (-10758.0) * carg - 3.0 * sarg;

    arg = 2.0 * el;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (29243.0) * sarg - 74.0 * carg;
    de += (-609.0) * carg + 13.0 * sarg;

    arg = 2.0 * f;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (25887.0) * sarg - 66.0 * carg;
    de += (-550.0) * carg + 11.0 * sarg;

    arg = elp + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-14053.0 - 25.0 * t) * sarg + 79.0 * carg;
    de += (8551.0 - 2.0 * t) * carg - 45.0 * sarg;

    arg = -el + 2.0 * d + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (15164.0 + 10.0 * t) * sarg + 11.0 * carg;
    de += (-8001.0) * carg - sarg;

    arg = 2.0 * elp + 2.0 * f - 2.0 * d + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-15794.0 + 72.0 * t) * sarg - 16.0 * carg;
    de += (6850.0 - 42.0 * t) * carg - 5.0 * sarg;

    arg = -2.0 * f + 2.0 * d;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (21783.0) * sarg + 13.0 * carg;
    de += (-167.0) * carg + 13.0 * sarg;

    arg = el - 2.0 * d + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-12873.0 - 10.0 * t) * sarg - 37.0 * carg;
    de += (6953.0) * carg - 14.0 * sarg;

    arg = -elp + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-12654.0 + 11.0 * t) * sarg + 63.0 * carg;
    de += (6415.0) * carg + 26.0 * sarg;

    arg = -el + 2.0 * f + 2.0 * d + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-10204.0) * sarg + 25.0 * carg;
    de += (5222.0) * carg + 15.0 * sarg;

    arg = 2.0 * elp;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (16707.0 - 85.0 * t) * sarg - 10.0 * carg;
    de += (168.0 - 1.0 * t) * carg + 10.0 * sarg;

    arg = el + 2.0 * f + 2.0 * d + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-7691.0) * sarg + 44.0 * carg;
    de += (3268.0) * carg + 19.0 * sarg;

    arg = -2.0 * el + 2.0 * f;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-11024.0) * sarg - 14.0 * carg;
    de += (104.0) * carg + 2.0 * sarg;

    arg = elp + 2.0 * f + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (7566.0 - 21.0 * t) * sarg - 11.0 * carg;
    de += (-3250.0) * carg - 5.0 * sarg;

    arg = 2.0 * f + 2.0 * d + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-6637.0 - 11.0 * t) * sarg + 25.0 * carg;
    de += (3353.0) * carg + 14.0 * sarg;

    arg = -elp + 2.0 * f + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-7141.0 + 21.0 * t) * sarg + 8.0 * carg;
    de += (3070.0) * carg + 4.0 * sarg;

    arg = 2.0 * d + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-6302.0 - 11.0 * t) * sarg + 2.0 * carg;
    de += (3272.0) * carg + 4.0 * sarg;

    arg = el + 2.0 * f - 2.0 * d + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (5800.0 + 10.0 * t) * sarg + 2.0 * carg;
    de += (-3045.0) * carg - sarg;

    arg = 2.0 * el + 2.0 * f - 2.0 * d + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (6443.0) * sarg - 7.0 * carg;
    de += (-2768.0) * carg - 4.0 * sarg;

    arg = -2.0 * el + 2.0 * d + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-5774.0 - 11.0 * t) * sarg - 15.0 * carg;
    de += (3041.0) * carg - 5.0 * sarg;

    arg = 2.0 * el + 2.0 * f + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-5350.0) * sarg + 21.0 * carg;
    de += (2695.0) * carg + 12.0 * sarg;

    arg = -elp + 2.0 * f - 2.0 * d + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-4752.0 - 11.0 * t) * sarg - 3.0 * carg;
    de += (2719.0) * carg - 3.0 * sarg;

    arg = -2.0 * d + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-4940.0 - 11.0 * t) * sarg - 21.0 * carg;
    de += (2720.0) * carg - 9.0 * sarg;

    arg = -el - elp + 2.0 * d;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (7350.0) * sarg - 8.0 * carg;
    de += (-51.0) * carg + 4.0 * sarg;

    arg = 2.0 * el - 2.0 * d + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (4065.0) * sarg + 6.0 * carg;
    de += (-2206.0) * carg + sarg;

    arg = el + 2.0 * d;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (6579.0) * sarg - 24.0 * carg;
    de += (-199.0) * carg + 2.0 * sarg;

    arg = elp + 2.0 * f - 2.0 * d + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (3579.0) * sarg + 5.0 * carg;
    de += (-1900.0) * carg + sarg;

    arg = el - elp;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (4725.0) * sarg - 6.0 * carg;
    de += (-41.0) * carg + 3.0 * sarg;

    arg = -2.0 * el + 2.0 * f + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-3075.0) * sarg - 2.0 * carg;
    de += (1313.0) * carg - sarg;

    arg = 3.0 * el + 2.0 * f + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-2904.0) * sarg + 15.0 * carg;
    de += (1233.0) * carg + 7.0 * sarg;

    arg = -elp + 2.0 * d;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (4348.0) * sarg - 10.0 * carg;
    de += (-81.0) * carg + 2.0 * sarg;

    arg = el - elp + 2.0 * f + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-2878.0) * sarg + 8.0 * carg;
    de += (1232.0) * carg + 4.0 * sarg;

    sarg = (d).sin();
    carg = (d).cos();
    dp += (-4230.0) * sarg + 5.0 * carg;
    de += (-20.0) * carg - 2.0 * sarg;

    arg = -el - elp + 2.0 * f + 2.0 * d + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-2819.0) * sarg + 7.0 * carg;
    de += (1207.0) * carg + 3.0 * sarg;

    arg = -el + 2.0 * f;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-4056.0) * sarg + 5.0 * carg;
    de += (40.0) * carg - 2.0 * sarg;

    arg = -elp + 2.0 * f + 2.0 * d + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-2647.0) * sarg + 11.0 * carg;
    de += (1129.0) * carg + 5.0 * sarg;

    arg = -2.0 * el + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-2294.0) * sarg - 10.0 * carg;
    de += (1266.0) * carg - 4.0 * sarg;

    arg = el + elp + 2.0 * f + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (2481.0) * sarg - 7.0 * carg;
    de += (-1062.0) * carg - 3.0 * sarg;

    arg = 2.0 * el + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (2179.0) * sarg - 2.0 * carg;
    de += (-1129.0) * carg - 2.0 * sarg;

    arg = -el + elp + d;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (3276.0) * sarg + carg;
    de += (-9.0) * carg;

    arg = el + elp;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-3389.0) * sarg + 5.0 * carg;
    de += (35.0) * carg - 2.0 * sarg;

    arg = el + 2.0 * f;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (3339.0) * sarg - 13.0 * carg;
    de += (-107.0) * carg + sarg;

    arg = -el + 2.0 * f - 2.0 * d + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-1987.0) * sarg - 6.0 * carg;
    de += (1073.0) * carg - 2.0 * sarg;

    arg = el + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-1981.0) * sarg;
    de += (854.0) * carg;

    arg = -el + d;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (4026.0) * sarg - 353.0 * carg;
    de += (-553.0) * carg - 139.0 * sarg;

    arg = 2.0 * f + d + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (1660.0) * sarg - 5.0 * carg;
    de += (-710.0) * carg - 2.0 * sarg;

    arg = -el + 2.0 * f + 4.0 * d + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-1521.0) * sarg + 9.0 * carg;
    de += (647.0) * carg + 4.0 * sarg;

    arg = -el + elp + d + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (1314.0) * sarg;
    de += (-700.0) * carg;

    arg = -2.0 * elp + 2.0 * f - 2.0 * d + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-1283.0) * sarg;
    de += (672.0) * carg;

    arg = el + 2.0 * f + 2.0 * d + om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (-1331.0) * sarg + 8.0 * carg;
    de += (663.0) * carg + 4.0 * sarg;

    arg = -2.0 * el + 2.0 * f + 2.0 * d + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (1383.0) * sarg - 2.0 * carg;
    de += (-594.0) * carg - 2.0 * sarg;

    arg = -el + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (1405.0) * sarg + 4.0 * carg;
    de += (-610.0) * carg + 2.0 * sarg;

    arg = el + elp + 2.0 * f - 2.0 * d + 2.0 * om;
    sarg = (arg).sin();
    carg = (arg).cos();
    dp += (1290.0) * sarg;
    de += (-556.0) * carg;

    dpsi = -0.000135 + (dp * 1.0e-7);
    deps = 0.000388 + (de * 1.0e-7);

    let mobl = mean_oblig(time);
    let tobl = mobl + (deps / 3600.0);
    let tt = time;
    let ee = dpsi * mobl.to_radians().cos() / 15.0;

    // Etilt(dpsi, deps, mobl, tobl, tt, ee)
    Etilt {
        dpsi,
        deps,
        mobl,
        tobl,
        tt,
        ee,
    }
}
