use crate::defs::consts;
use crate::defs::funcs;
use crate::defs::rotate_vector;
use crate::defs::structs;
//
pub(crate) fn vsop_formula(formula: Vec<Vec<Vec<f64>>>, t: f64, clamp_angle: bool) -> f64 {
    let mut tpower = 1.0;
    let mut coord = 0.0;
    let mut incr = 0.0;

    for lst in &formula {
        incr = 0.0;
        for val in lst {
            incr += val[0] * (val[1] + val[2] * &t).cos();
        }
        incr *= &tpower;
        tpower *= &t;

        if clamp_angle == true {
            incr = funcs::fmod(incr, consts::PI2);
        }
        coord += incr;
    }
    coord
}
//
pub(crate) fn calc_vsop(model: &Vec<Vec<Vec<Vec<f64>>>>, time: f64) -> structs::Vector {
    let t = &time / consts::DAYS_PER_MILLENNIUM;
    let lon = &vsop_formula(model[0].to_vec(), t, true);
    let lat = &vsop_formula(model[1].to_vec(), t, false);
    let rad = &vsop_formula(model[2].to_vec(), t, false);
    let eclip = rotate_vector::vsop_sphere_to_rect(*lon, *lat, *rad);
    let tmp = rotate_vector::vsop_rotate(eclip);
    tmp.to_astro_vector(time)
}
//
