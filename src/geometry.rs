// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::BitXor;

#[derive(Copy, Clone)]
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
    pub fn norm(self) -> f32 {
        return (self.x*self.x+self.y*self.y+self.z*self.z).sqrt();
    }
    pub fn normalized(self, l: f32) -> Vector3D {
        return self*(l/self.norm());
    }
}
impl fmt::Display for Vector3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}
impl Add for Vector3D {
    type Output = Vector3D;
    fn add(self, other: Vector3D) -> Vector3D {
        Vector3D { x: self.x + other.x, y: self.y + other.y, z:  self.z + other.z}
    }
}
impl Sub for Vector3D {
    type Output = Vector3D;
    fn sub(self, other: Vector3D) -> Vector3D {
        Vector3D { x: self.x - other.x, y: self.y - other.y, z:  self.z - other.z}
    }
}
impl Mul for Vector3D {
    type Output = f32;
    fn mul(self, other: Vector3D) -> f32 {
        return self.x*other.x + self.y*other.y + self.z*other.z;
    }
}
impl Mul<f32> for Vector3D {
    type Output = Vector3D;
    fn mul(self, other: f32) -> Vector3D {
        Vector3D { x: self.x * other, y: self.y * other, z:  self.z * other}
    }
}
impl BitXor for Vector3D {
    type Output = Vector3D;
    fn bitxor(self, v: Vector3D) -> Vector3D {
        Vector3D { x: self.y*v.z-self.z*v.y, y: self.z*v.x-self.x*v.z, z: self.x*v.y-self.y*v.x}
    }
}

