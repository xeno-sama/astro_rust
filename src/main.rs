#![allow(unused_variables, unused_assignments, unused_imports, dead_code)]
// #[derive(Debug)]
use chrono::prelude::*;
mod defs;
use defs::astronomy;
use defs::consts;
use defs::funcs;
use defs::structs;
use strum::IntoEnumIterator;
// use strum_macros::EnumIter;

fn main() {
    let natal = structs::Natal {
        year: 1977,
        month: 5,
        day: 27,
        hour: 16,
        minute: 52,
        sec: 0,
        lat: 42.9,
        lon: 74.6,
    };

    let data =
        Utc.ymd(natal.year, natal.month, natal.day)
            .and_hms(natal.hour, natal.minute, natal.sec);

    let (observer, time) = astronomy::parse_args(data, natal.lat, natal.lon);

    for body in consts::Body::iter() {
        if body == consts::Body::Pluto {
            println!("{:?}", body);
        }
    }

    //
    // println!("{:?} {:?}", time, observer.latitude)
}
