#![allow(unused_variables, unused_assignments, unused_imports, dead_code)]
// #[derive(Debug)]
mod defs;
use defs::consts;
use defs::funcs;
use defs::structs;

use crate::defs::consts::PD;

// use crate::defs::*;

fn main() {
    let vsop = consts::vsop();
    println!("{:?}", vsop)
}
