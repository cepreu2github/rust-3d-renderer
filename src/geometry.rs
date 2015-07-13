// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
use std::fmt;

pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Point3D {
        Point3D {
            x: x,
            y: y,
            z: z,
        }
    }
}
impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

