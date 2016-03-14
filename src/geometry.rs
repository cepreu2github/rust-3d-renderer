// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::BitXor;
use num::traits::NumCast;
use std::ops::IndexMut;
use std::ops::Index;

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
    rows: usize,
    cols: usize,
    elems: Vec<Vec<f32>>,
}
impl Index<usize> for Matrix {
    type Output = Vec<f32>;

    fn index<'a>(&'a self, _index: usize) -> &'a Vec<f32> {
        return &self.elems[_index];
    }
}
impl IndexMut<usize> for Matrix {
    fn index_mut<'a>(&'a mut self, _index: usize) -> &'a mut Vec<f32> {
        return &mut self.elems[_index];
    }
}
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.rows {
            if i != 0 {
                write!(f, "\n");
            }
            for j in 0..self.cols {
                write!(f, "{}", self[i][j]);
                if j != self.cols-1 {
                    write!(f, ", ");
                }
            }
        }
        Ok(())
    }
}
impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        if self.cols != rhs.rows {
            panic!("matrix sizes are not correspond (lhs.cos={} and rhs.rows={})", self.cols, rhs.rows);
        }
        let mut result = Matrix::new(self.rows, rhs.cols);
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                result[i][j] = 0.0;
                for k in 0..self.cols {
                    result[i][j] += self[i][k] * rhs[k][j];
                }
                debug!("result[{}][{}] = {}", i, j, result[i][j]);
            }
        }
        return result;
    }
}
impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix{
        Matrix {
            rows: rows,
            cols: cols,
            elems: vec![vec![0.0; cols]; rows],
        }
    }
    pub fn rows(&self) -> usize{
        return self.rows;
    }
    pub fn cols(&self) -> usize{
        return self.cols;
    }
    pub fn identity(dimensions: usize) -> Matrix{
        let mut result = Matrix::new(dimensions, dimensions);
        for i in 0..dimensions {
            result[i][i] = 1.0;
        }
        return result;
    }
    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[j][i] = self[i][j];
            }
        }
        return result;
    }
    pub fn inverse(&self) -> Result<Matrix, String> {
        if self.cols != self.rows {
            return Err("matrix are not square".to_owned());
        }
        // augmenting the square matrix with the identity matrix of the same 
        // dimensions a => [ai]
        let mut result = Matrix::new(self.rows, self.cols*2);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[i][j] = self[i][j];
            }
        }
        for i in 0..self.rows {
            result[i][i+self.cols] = 1.;
        }
        // first pass
        for i in 0..self.rows-1 {
            for j in (0..result.cols).rev() {
                result[i][j] /= result[i][i];
            }
            for k in i+1..self.rows {
                let coeff = result[k][i];
                for j in 0..result.cols {
                    result[k][j] -= result[i][j]*coeff;
                }
            }
        }
        // normalize the last row
        for j in (self.rows-1..result.cols).rev() {
            result[self.rows-1][j] /= result[self.rows-1][self.rows-1];
        }
        // second pass
        for i in (1..self.rows).rev() {
            for k in (0..i).rev() {
                let coeff = result[k][i];
                for j in 0..result.cols {
                    result[k][j] -= result[i][j]*coeff;
                }
            }
        }
        // cut the identity matrix back
        let mut truncate = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                truncate[i][j] = result[i][j+self.cols];
            }
        }
        return Ok(truncate);
    }
    pub fn m2v(m: Matrix) -> Vector3D<f32> {
        return Vector3D::new(m[0][0]/m[3][0], m[1][0]/m[3][0], m[2][0]/m[3][0]);
    }
    pub fn v2m(v: Vector3D<f32>) -> Matrix {
        let mut m = Matrix::new(4, 1);
        m[0][0] = v.x;
        m[1][0] = v.y;
        m[2][0] = v.z;
        m[3][0] = 1.0;
        return m;
    }
}
