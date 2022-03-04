#![allow(
    unused_assignments,
)]
// #[derive(Debug)]  /&'static
mod defs;
use chrono::prelude::*;
use defs::astronomy;
use defs::structs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

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

    let body_list: [usize; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // Mercury_0,Venus_1,Earth_2,Mars_3,Jupiter_4,Saturn_5,Uranus_6,Neptune_7,Pluto_8,Sun_9,Moon_10

    for body in body_list {
        let gm = astronomy::helio_lon(body, time);
        println!("{gm}");
    }

    println!("");
    for body in body_list {
        let gl = astronomy::geo_lon(body, time, &observer);
        println!("{}", gl);
    }

    
    let dm = defs::domes::domes(data, natal.lat, natal.lon);
    println!("\n{:?}", dm);

    let duration = start.elapsed().as_secs_f64();
    println!("\ntime elapsed {:?}", duration)
}
