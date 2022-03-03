use crate::defs::funcs::deg_360;

pub(crate) fn calc_pluto(time: f64) -> [f64; 2] {
    let xsun = data_sun(time)[2];
    let ysun = data_sun(time)[3];
    let s = 50.03 + 0.033459652 * time;
    let p = 238.95 + 0.003968789 * time;

    let lon_ecl = 238.9508 + 0.00400703 * time - 19.799 * p.to_radians().sin()
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

    let _lon_topos = ygeoc.atan2(xgeoc);
    let lon_topos = deg_360(_lon_topos.to_degrees());

    [lon_ecl, lon_topos]
}

fn data_sun(time: f64) -> Vec<f64> {
    // Шаг 1 - вычислить E экцентрическую аномалию
    // sin/cos выдают в радианах - поэтому сразу конвертировать *pi/180
    let w_sun = 282.9404 + 4.70935E-5 * time;
    let e_sun = 0.016709 - 1.151E-9 * time;
    let m_sun = deg_360(356.0470 + 0.9856002585 * time);

    let __e =
        m_sun + e_sun.to_degrees() * m_sun.to_radians().sin() * (1.0 + m_sun.to_radians().cos());
    let e = __e.to_radians();

    // Шаг 2 - вычислить прямоугольные координаты солнца в плоскости эклиптики
    // где x - угол направленный к перигелию
    let x = e.cos() - e_sun;
    let y = e.sin() * ((1.0 - e_sun * e_sun).sqrt());

    // Шаг 3 - конвертировать в расстояние и истинную аномалию
    let r = (x * x + y * y).sqrt();
    let _v = &y.atan2(x);
    let v = deg_360(_v.to_degrees());

    // Шаг 4 (главный) - Теперь подсчитать ГЕОЦЕНТРИКУ !!!
    // lat = 0 # т.к это само солнце в своей плоскости
    // R = 0   # т.к это само солнце и расст до солнца = 0
    let _lon = v + w_sun;
    let _lon_ecl = deg_360(_lon);
    let lon_ecl = deg_360(_lon + 180.0);
    let lon_topos = deg_360(_lon); // по факту = топоцентрике

    let x_eclip = r * _lon_ecl.to_radians().cos();
    let y_eclip = r * _lon_ecl.to_radians().sin();

    [lon_topos, lon_ecl, x_eclip, y_eclip].to_vec()
}
