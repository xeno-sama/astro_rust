// # -------------------------
// # Преобразование координат
// # -------------------------
use super::structs::EclipticCoordinates;
use super::structs::TerseVector;
use super::structs::Vector;
//
pub(crate) fn ecliptic(vec: Vector) -> EclipticCoordinates {
    //Преобразование прямоугольных в эклиптические координаты.
    //Based on NOVAS functions equ2ecl() and equ2ecl_vec().
    let ob2000 = 0.40909260059599012; // mean obliquity of the J2000 ecliptic in radians
    return rotate_equatorial_to_ecliptic([vec.x, vec.y, vec.z], ob2000, vec.t);
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
//
pub(crate) fn vsop_rotate(eclip: TerseVector) -> TerseVector {
    let x = eclip.x + 0.000000440360 * eclip.y - 0.000000190919 * eclip.z;
    let y = -0.000000479966 * eclip.x + 0.917482137087 * eclip.y - 0.397776982902 * eclip.z;
    let z = 0.397776982902 * eclip.y + 0.917482137087 * eclip.z;
    return TerseVector { x, y, z };
}
//
pub(crate) fn vsop_sphere_to_rect(lon: f64, lat: f64, rad: f64) -> TerseVector {
    let r_cos_lat = rad * (lat).cos();
    TerseVector {
        x: r_cos_lat * (lon).cos(),
        y: r_cos_lat * (lon).sin(),
        z: rad * (lat).sin(),
    }
}
