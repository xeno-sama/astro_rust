#![allow(unused_variables, unused_assignments, unused_imports, dead_code)]

pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub t: f64,
}
impl Vector {
    pub fn length(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    } //Returns the length of the vector in AU. (xˆ2+yˆ2+zˆ2)
}

pub struct EclipticCoordinates {
    // Ecliptic angular and Cartesian coordinates.
    pub vec: Vec<f64>,
    pub elat: f64,
    pub elon: f64,
}

pub struct Observer {
    //Represents the geographic location of an observer on the surface of the Earth.
    pub latitude: f64,
    pub longitude: f64,
}

pub struct TerseVector {
    //A combination of a position vector, a velocity vector, and a time.
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
// Convert _TerseVector object to Vector object
impl TerseVector {
    pub fn to_astro_vector(&self, time: f64) -> Vector {
        Vector {
            x: self.x,
            y: self.y,
            z: self.z,
            t: time,
        }
    }
}
//  Return magnitude squared of this vector.
impl TerseVector {
    pub fn quadrature(&self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)
    }
}
//  Return the average of this vector and another vector.
impl TerseVector {
    pub fn mean(&self, other: TerseVector) -> TerseVector {
        TerseVector {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
            z: (self.z + other.z) / 2.0,
        }
    }
}
// RotationMatrix - contains a rotation matrix 3x3 that can be used to
// transform one coordinate system into another.
pub struct RotationMatrix {
    pub rot: [[f64; 3]; 3],
}

pub struct Etilt {
    pub dpsi: f64,
    pub deps: f64,
    pub mobl: f64,
    pub tobl: f64,
    pub ee: f64,
    pub tt: f64,
}
