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
use chrono::{Duration, TimeZone, Utc};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let body_list: [usize; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let body_list: [usize; 5] = [0, 1, 2, 3, 4];
    // Mercury_0,Venus_1,Earth_2,Mars_3,Jupiter_4,Saturn_5,Uranus_6,Neptune_7,Pluto_8,Sun_9,Moon_10

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

    let mut data = Utc.ymd(1977, 5, 27).and_hms(16, 52, 0);
   
    let (observer, time) = astronomy::parse_args(data, natal.lat, natal.lon);

    let mut helio = vec![];
    let mut geo = vec![];
    let mut dm = vec![];

    for body in body_list {
        helio.push(astronomy::helio_lon(body, time));
    }

    for body in body_list {
        geo.push(astronomy::geo_lon(body, time, &observer));
    }

    dm = defs::domes::domes(data, natal.lat, natal.lon);
    println!("\n{:?}", helio);
    println!("\n{:?}", geo);
    println!("{}", data);
    println!("\n{:?}", dm[0]);
        
    

    let duration = start.elapsed().as_secs_f64();
    println!("\ntime elapsed {:?}", duration)
}

 // let data_end = Utc.ymd(1977, 5, 28).and_hms(16, 52, 0);
    // while data < data_end {
    //     data = data + (Duration::days(1));
    // }