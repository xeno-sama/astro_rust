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

    let body_list: [usize;7] = [0, 1, 2, 3, 4, 5, 6]; //, 7, 8, 9
    // Mercury_0,Venus_1,Earth_2,Mars_3,Jupiter_4,Saturn_5,Uranus_6,Neptune_7,Pluto_8,Sun_9,Moon_10

    for body in body_list {
        let gm = astronomy::helio_eclip_lon(body, time);
        println!("{gm}");
    }
    

    let dt = funcs::num_date(data);
    // println!("{:?} {}", gm, dt);
}

// let mut incr = 0.0;
//     let mut incr_sum = 0.0;
//     let _vsop = &consts::vsop()[0];
//     for val in &_vsop[1][1] {
//         incr = 0.0;
//         for i in val {
//             incr += i
//         }
//         incr_sum += incr;
//     }
//     println!("{incr_sum}");
