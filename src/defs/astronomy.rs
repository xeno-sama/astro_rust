#![allow(unused_variables, unused_assignments, unused_imports, dead_code)]
use crate::defs::consts::*;
use crate::defs::espenac;
use crate::defs::structs::*;
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
    let dt1 = Utc.ymd(2000, 1, 1).and_hms(0, 0, 0);
    data.signed_duration_since(dt1).num_seconds() as f64 / 86400.0
}
// # ------------------------
// # 3. секция ГЕЛИОЦЕНТРИКИ
// # ------------------------
