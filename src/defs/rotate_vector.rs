// # -------------------------
// # Преобразование координат
// # -------------------------
use crate::defs::funcs;
use crate::defs::structs;

use super::structs::EclipticCoordinates;
use super::structs::Vector;
//
pub(crate) fn ecliptic(equ: Vector, time: f64) -> EclipticCoordinates {
    //Преобразование прямоугольных в эклиптические координаты.
    //Based on NOVAS functions equ2ecl() and equ2ecl_vec().
    let ob2000 = 0.40909260059599012; // mean obliquity of the J2000 ecliptic in radians
    return rotate_equatorial_to_ecliptic([equ.x, equ.y, equ.z], ob2000, equ.t);
}
//
pub(crate) fn rotate_equatorial_to_ecliptic(
    pos: [f64; 3],
    obliq_radians: f64,
    time: f64,
) -> EclipticCoordinates {
    let cos_ob = (obliq_radians).cos();
    let sin_ob = (obliq_radians).sin();
    let ex = pos[0];
    let ey = pos[1] * cos_ob + pos[2] * sin_ob;
    let ez = pos[1] * sin_ob + pos[2] * cos_ob;
    let mut elon;

    let xyproj = (ex * ex + ey * ey).sqrt();
    if xyproj > 0.0 {
        elon = (ey.atan2(ex)).to_degrees();
        if elon < 0.0 {
            elon += 360.0;
        }
    } else {
        elon = 0.0;
    }

    let elat = (ez.atan2(xyproj)).to_degrees();
    let vec = Vector {
        x: ex,
        y: ey,
        z: ez,
        t: time,
    };

    return EclipticCoordinates { vec, elat, elon };
}
