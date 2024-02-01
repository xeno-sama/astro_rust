use crate::defs::consts;
use crate::defs::espenac;
use crate::defs::funcs;
use crate::defs::moon;
use crate::defs::pluto;
use crate::defs::rotate_vector::ecliptic;
use crate::defs::structs;
use crate::defs::vsop;
use chrono::prelude::*;
//
// # -------------------------
// # 1. Парсинг входной даты
// # -------------------------
pub(crate) fn parse_args(data: DateTime<Utc>, lat: f64, lon: f64) -> (structs::Observer, f64) {
    let time = make(data);
    let observer = structs::Observer {
        latitude: lat,
        longitude: lon,
    };
    (observer, dt(time))
}
// # --------------------------------------
// # 2. Получение обьекта времени Time(ut)
// # --------------------------------------
fn dt(time: f64) -> f64 {
    let delta_t = espenac::Timex::delta_espenac(time);
    time + delta_t / 86400.0
}
//
fn make(data: DateTime<Utc>) -> f64 {
    let dt1 = Utc.ymd(2000, 1, 1).and_hms(12, 0, 0);
    data.signed_duration_since(dt1).num_seconds() as f64 / 86400.0
}
//
// -------------------------
//  3. секция ГЕЛИОЦЕНТРИКИ
// -------------------------
//
fn helio_vector(body: usize, time: f64) -> structs::Vector {
    if body > 8 {
        structs::Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            t: time,
        }
    } else {
        vsop::calc_vsop(&consts::vsop()[body], time)
    }
}
//
pub(crate) fn helio_lon(body: usize, time: f64) -> f64 {
    if body == 9 {
        0.0
    } else if body == 10 {0.01}
    else if body == 8 {
        pluto::calc_pluto(time)[0]
    } else {
        let hv = helio_vector(body, time);
        let helio = ecliptic(hv);
        helio.elon
    }
}
//
// -----------------------
//  4. секция ГЕОЦЕНТРИКИ
// -----------------------
//
fn calc_earth(time: f64) -> structs::Vector {
    vsop::calc_vsop(&consts::vsop()[2], time)
}
//
fn geo_vector(body: usize, time: f64) -> structs::Vector {
    let earth = calc_earth(time);
    let helio = helio_vector(body, time);
    let geo = structs::Vector {
        x: helio.x - earth.x,
        y: helio.y - earth.y,
        z: helio.z - earth.z,
        t: time,
    };
    geo
}
//
pub(crate) fn geo_lon(body: usize, time: f64, observer: &structs::Observer) -> f64 {
    if body == 2 {
        0.0
    } else if body == 8 {
        pluto::calc_pluto(time)[1]
    } else if body == 10 {
        moon::geo_moon(time)
    } else {
        let gc_observer = funcs::geo_pos(time, &observer);
        let gc = geo_vector(body, time);

        let j2000: [f64; 3] = [
            gc.x - gc_observer[0],
            gc.y - gc_observer[1],
            gc.z - gc_observer[2],
        ];
        let prec_vec = funcs::precession(j2000, time, &consts::PD.from2000);
        let nut_vec = funcs::nutation(prec_vec, time, &consts::PD.from2000);
        let geo_vect = structs::Vector {
            x: nut_vec[0],
            y: nut_vec[1],
            z: nut_vec[2],
            t: time,
        };
        let _geo = ecliptic(geo_vect);
        _geo.elon
    }
}
