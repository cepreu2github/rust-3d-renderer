// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::BitXor;
use num::traits::NumCast;

#[derive(Copy, Clone)]
pub struct Vector3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
impl<T> Vector3D<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3D<T> {
        Vector3D {
            x: x,
            y: y,
            z: z,
        }
    }
}
impl<T: NumCast> Vector3D<T> {
    pub fn to<V: NumCast>(self) -> Vector3D<V> {
        Vector3D {
            x: NumCast::from(self.x).unwrap(),
            y: NumCast::from(self.y).unwrap(),
            z: NumCast::from(self.z).unwrap(),
        }
    }
}
impl Vector3D<f32> {
    pub fn norm(self) -> f32 {
        return (self.x*self.x+self.y*self.y+self.z*self.z).sqrt();
    }
    pub fn normalized(self, l: f32) -> Vector3D<f32> {
        return self*(l/self.norm());
    }
}
impl<T: fmt::Display> fmt::Display for Vector3D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}
impl<T: Add<Output = T>> Add for Vector3D<T> {
    type Output = Vector3D<T>;
    fn add(self, other: Vector3D<T>) -> Vector3D<T> {
        Vector3D { x: self.x + other.x, y: self.y + other.y, z:  self.z + other.z}
    }
}
impl<T: Sub<Output = T>> Sub for Vector3D<T> {
    type Output = Vector3D<T>;
    fn sub(self, other: Vector3D<T>) -> Vector3D<T> {
        Vector3D { x: self.x - other.x, y: self.y - other.y, z:  self.z - other.z}
    }
}
impl<T: Mul<Output = T> + Add<Output = T>> Mul for Vector3D<T> {
    type Output = T;
    fn mul(self, other: Vector3D<T>) -> T {
        return self.x*other.x + self.y*other.y + self.z*other.z;
    }
}
impl<T: Mul<Output = T> + Copy> Mul<T> for Vector3D<T> {
    type Output = Vector3D<T>;
    fn mul(self, other: T) -> Vector3D<T> {
        Vector3D { x: self.x * other, y: self.y * other, z:  self.z * other}
    }
}
impl<T: Mul<Output = T> + Sub<Output = T> + Copy> BitXor for Vector3D<T> {
    type Output = Vector3D<T>;
    fn bitxor(self, v: Vector3D<T>) -> Vector3D<T> {
        Vector3D { x: self.y*v.z-self.z*v.y, y: self.z*v.x-self.x*v.z, z: self.x*v.y-self.y*v.x}
    }
}

pub struct Matrix {
    rows: u32,
    cols: u32,
    elems: Vec<f32>,
}

impl Matrix {
    pub fn new(rows: u32, cols: u32) -> Matrix{
        Matrix {
            rows: rows,
            cols: cols,
            elems: vec![0.0; (cols*rows) as usize],
        }
    }
    pub fn rows(&self) -> u32{
        return self.rows;
    }
    pub fn cols(&self) -> u32{
        return self.cols;
    }
    pub fn identity(dimensions: u32) -> Matrix{
        let result = Matrix::new(dimensions, dimensions);
        for i in 0..dimensions {
            result[i][i] = 1.0;
        }
        return result;
    }

}
