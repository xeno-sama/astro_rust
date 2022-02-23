#![allow(unused_variables, unused_assignments, unused_imports, dead_code)]
use std::collections::HashMap;

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
    let gm = astronomy::geo_lon(consts::Body::Pluto, time, observer);

    let tmp = [1, 2, 3];
    let i = 4;
    // let tm = test(0, i, tmp);

    fn test(mut y: i32, i: i32, tmp: [i32; 3]) -> i32 {
        for x in tmp {
            if x <= i {
                y += x;
                println!("{y}")
            } else {
                y += 10;
            }
        }
        return y;
    }
    let dict = HashMap::from([(1, 3), (2, 31)]);

    println!("{:?}", dict[&2])
}
