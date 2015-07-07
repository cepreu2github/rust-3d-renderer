// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3D {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3D {
        Vector3D {
            x: x,
            y: y,
            z: z,
        }
    }
}

