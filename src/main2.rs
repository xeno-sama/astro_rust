#![allow(unused_variables, unused_assignments, unused_imports, dead_code, unused_mut)]
// #[derive(Debug)]  /&'static
use std::collections::HashMap;
mod defs;
use chrono::prelude::*;
use defs::astronomy;
use defs::consts;
use defs::funcs;
use defs::pluto;
use defs::moon;
use defs::structs;
use num::complex::Complex;
use num::complex::Complex64;

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

    let body_list: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    // Mercury_0,Venus_1,Earth_2,Mars_3,Jupiter_4,Saturn_5,Uranus_6,Neptune_7,Pluto_8,Sun_9,Moon_10

    for body in body_list {
        if body == 8 {
            let gm = pluto::calc_pluto(time)[0];
            println!("{gm}");
        } else {
            let gm = astronomy::helio_lon(body, time);
            println!("{gm}");
        }
    }
    println!("\n");
    for body in body_list {
        if body == 8 {
            let gm = pluto::calc_pluto(time)[1];
            println!("{gm}");
        } else {
            let gl = astronomy::geo_lon(body, time, &observer);
            println!("{}", gl);
        }
    }

    let mut map: HashMap<i32, HashMap<i32, Complex64>> = [
        (
            1,
            [
                (2, Complex { re: 1.0, im: 0.0 }),
                (1, Complex { re: 2.0, im: 0.0 }),
            ]
            .into(),
        ),
        (
            2,
            [
                (1, Complex { re: 3.0, im: 0.0 }),
                (2, Complex { re: 4.0, im: 0.0 }),
            ]
            .into(),
        ),
    ]
    .into();
    
}


// let dt = funcs::num_date(data);

// let gl = astronomy::geo_lon(7, time, &observer);
// let gl = pluto::calc_pluto(time);
// println!("{} {}", gl[0], gl[1]);
// let cn1 = Complex::new(1.0, 0.0);

// let mut tmp: HashMap<i32, HashMap<i32, &str>> = [(1, [(2, "2"), (1, "1")].into()), (2, [(1, "1"), (2, "2")].into())].into();

 // let mut map = HashMap::from([(1, "first"), (2, "second")]);
// println!("{:?}", map);

// let vec1 = Vec::from_iter(map.iter());
// println!("{:?}", vec1);


// let mut ex = moon::calc_moon(time);
    // println!("{:?}", ex);