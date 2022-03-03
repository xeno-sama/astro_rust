#![allow(unused_variables, unused_assignments, unused_imports, dead_code)]
use super::rotate_vector::ecliptic;
use super::structs;
use crate::defs::consts::*;
use crate::defs::espenac;
use crate::defs::funcs;
use crate::defs::structs::*;
use crate::defs::vsop;
use chrono::prelude::*;
//
// # -------------------------
// # 1. Парсинг входной даты
// # -------------------------
pub(crate) fn parse_args(data: DateTime<Utc>, lat: f64, lon: f64) -> (Observer, f64) {
    let time = make(data);
    let observer = Observer {
        latitude: lat,
        longitude: lon,
    };
    (observer, dt(time))
}
// # --------------------------------------
// # 2. Получение обьекта времени Time(ut)
// # --------------------------------------
pub(crate) fn dt(time: f64) -> f64 {
    let delta_t = espenac::Timex::delta_espenac(time);
    time + delta_t / 86400.0
}
//
pub(crate) fn make(data: DateTime<Utc>) -> f64 {
    let dt1 = Utc.ymd(2000, 1, 1).and_hms(12, 0, 0);
    data.signed_duration_since(dt1).num_seconds() as f64 / 86400.0
}
//
// -------------------------
//  3. секция ГЕЛИОЦЕНТРИКИ
// -------------------------
//
pub(crate) fn helio_vector(body: usize, time: f64) -> Vector {
    if body >= 9 {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            t: time,
        }
    } else {
        vsop::calc_vsop(&vsop()[body], time)
    }
}
//
pub(crate) fn helio_lon(body: usize, time: f64) -> f64 {
    let hv = helio_vector(body, time);
    let helio = ecliptic(hv, time);
    helio.elon
}
//
// -----------------------
//  4. секция ГЕОЦЕНТРИКИ
// -----------------------
//
pub(crate) fn calc_earth(body: usize, time: f64) -> Vector {
    vsop::calc_vsop(&vsop()[2], time)
}
//
pub(crate) fn geo_vector(body: usize, time: f64) -> Vector {
    let earth = calc_earth(body, time);
    let helio = helio_vector(body, time);
    let geo = Vector {
        x: helio.x - earth.x,
        y: helio.y - earth.y,
        z: helio.z - earth.z,
        t: time,
    };
    geo
}
//
pub(crate) fn geo_lon(body: usize, time: f64, observer: &structs::Observer) -> f64 {
    let gc_observer = funcs::geo_pos(time, &observer);
    let gc = geo_vector(body, time);

    let j2000: [f64; 3] = [
        gc.x - gc_observer[0],
        gc.y - gc_observer[1],
        gc.z - gc_observer[2],
    ];
    // println!("{:?}", j2000); // OK
    let prec_vec = funcs::precession(j2000, time, &PD.from2000);
    // println!("{:?}", _tmp);
    let nut_vec = funcs::nutation(prec_vec, time, &PD.from2000);
    // println!("{:?}", data_vect);
    let geo_vect = Vector {
        x: nut_vec[0],
        y: nut_vec[1],
        z: nut_vec[2],
        t: time,
    };
    let _geo = ecliptic(geo_vect, time);
    if body == 2 {
        0.0
    } else {
        _geo.elon
    }
}
