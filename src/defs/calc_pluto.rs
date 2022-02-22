use crate::defs::funcs::deg_360;

pub(crate) fn data_pluto(xsun: f64, ysun: f64, _d: f64) -> f64 {
    let s = 50.03 + 0.033459652 * _d;
    let p = 238.95 + 0.003968789 * _d;

    let lon_ecl = 238.9508 + 0.00400703 * _d - 19.799 * p.to_radians().sin()
        + 19.848 * p.to_radians().cos()
        + 0.897 * (2.0 * p).to_radians().sin()
        - 4.956 * (2.0 * p).to_radians().cos()
        + 0.610 * (3.0 * p).to_radians().sin()
        + 1.211 * (3.0 * p).to_radians().cos()
        - 0.341 * (4.0 * p).to_radians().sin()
        - 0.190 * (4.0 * p).to_radians().cos()
        + 0.128 * (5.0 * p).to_radians().sin()
        - 0.034 * (5.0 * p).to_radians().cos()
        - 0.038 * (6.0 * p).to_radians().sin()
        + 0.031 * (6.0 * p).to_radians().cos()
        + 0.020 * (s - p).to_radians().sin()
        - 0.010 * (s - p).to_radians().cos();

    let lat = -3.9082 - 5.453 * p.to_radians().sin() - 14.975 * p.to_radians().cos()
        + 3.527 * (2.0 * p).to_radians().sin()
        + 1.673 * (2.0 * p).to_radians().cos()
        - 1.051 * (3.0 * p).to_radians().sin()
        + 0.328 * (3.0 * p).to_radians().cos()
        + 0.179 * (4.0 * p).to_radians().sin()
        - 0.292 * (4.0 * p).to_radians().cos()
        + 0.019 * (5.0 * p).to_radians().sin()
        + 0.100 * (5.0 * p).to_radians().cos()
        - 0.031 * (6.0 * p).to_radians().sin()
        - 0.026 * (6.0 * p).to_radians().cos()
        + 0.011 * (s - p).to_radians().sin();

    let r = 40.72 + 6.68 * p.to_radians().sin() + 6.90 * p.to_radians().cos()
        - 1.18 * (2.0 * p).to_radians().sin()
        - 0.03 * (2.0 * p).to_radians().cos()
        + 0.15 * (3.0 * p).to_radians().sin()
        - 0.14 * (3.0 * p).to_radians().cos();

    // Шаг 6 - Переводим в ГЕОЦЕНТРИЧЕСКИЕ координаты
    let xh = r * lon_ecl.to_radians().cos() * lat.to_radians().cos();
    let yh = r * lon_ecl.to_radians().sin() * lat.to_radians().cos();

    let xgeoc = xh + xsun;
    let ygeoc = yh + ysun;
    // println!("{}", 20_f64.to_radians());
    // println!("{r} {p} {lon_ecl} {xh} {yh} {xgeoc} {ygeoc}");
    let _lon_topos = ygeoc.atan2(xgeoc);
    let lon_topos = deg_360(_lon_topos.to_degrees());

    // println!("pluto {lon_topos} {xgeoc} {ygeoc}");
    lon_topos
}
// println!("{x} {y}");
//p.to_radians().sin()
//+1.0*(2.0*p).to_radians().cos()
