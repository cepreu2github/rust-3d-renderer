// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
use std::mem;
use geometry::Vector3D;
use tgacanvas::TgaCanvas;

fn get_gray(color: u32, intensity: f32) -> u32 {
    debug!("get color {:X} with intensity {}", color, intensity);
    let mut result = ((color as u8) as f32*intensity) as u32;
    result += (((color >> 8) as u8) as f32*intensity) as u32*256;
    result += (((color >> 16) as u8) as f32*intensity) as u32*256*256;
    debug!("result is {:X}", result);
    return result;
}

pub trait Canvas {
    fn canvas(&mut self) -> &mut Vec<Vec<u32>>;
    fn zbuffer(&mut self) -> &mut Vec<Vec<i32>>;
    fn xsize(&self) -> usize;
    fn ysize(&self) -> usize;

    fn new(x: usize, y: usize) -> Self;
    fn read(filename: &str) -> Self;
    fn out(&mut self);
    fn wait_for_enter(&mut self); 

    fn set(&mut self, x: i32, y: i32, color: u32) {
        if x < 0 || y < 0 {
            return;
        }
        if x >= self.xsize() as i32 || y >= self.ysize() as i32{
            return; 
        }
        self.canvas()[x as usize][y as usize] = color;
    }

    fn get(&mut self, x: i32, y: i32) -> u32 {
        if x < 0 || y < 0 {
            return 0;
        }
        if x >= self.xsize() as i32 || y >= self.ysize() as i32{
            return 0; 
        }
        return self.canvas()[x as usize][y as usize];
    }

    fn triangle(&mut self, mut p0: Vector3D<i32>, mut p1: Vector3D<i32>, mut p2: Vector3D<i32>,
                mut uv0: Vector3D<i32>, mut uv1: Vector3D<i32>, mut uv2: Vector3D<i32>, intensity: f32, diffusemap : &mut TgaCanvas) {
        debug!("drawing triangle {} - {} - {}", p0, p1, p2);
        debug!("with textures {} - {} - {}", uv0, uv1, uv2);
        if p0.y==p1.y && p0.y==p2.y {
            return; // i dont care about degenerate triangles
        }
        // sort the vertices, t0, t1, t2 lower-to-upper (bubblesort yay!)
        if p0.y > p1.y {
            mem::swap(&mut p0, &mut p1);
            mem::swap(&mut uv0, &mut uv1);
        }
        if p0.y > p2.y {
            mem::swap(&mut p0, &mut p2);
            mem::swap(&mut uv0, &mut uv2);
        }
        if p1.y > p2.y {
            mem::swap(&mut p1, &mut p2);
            mem::swap(&mut uv1, &mut uv2);
        }
        let total_height = p2.y - p0.y;
        for i in 0..total_height {
            let second_half = i > p1.y - p0.y || p1.y == p0.y;
            let segment_height = if second_half { p2.y - p1.y } else { p1.y - p0.y };
            let alpha = i as f32/total_height as f32;
            let beta  = (i - if second_half { p1.y - p0.y } else { 0 }) as f32/segment_height as f32; // be careful: with above conditions no division by zero here
            let mut a = p0.to::<f32>() + (p2-p0).to::<f32>()*alpha;
            let mut b = if second_half { p1.to::<f32>() + (p2-p1).to::<f32>()*beta } else { p0.to::<f32>() + (p1-p0).to::<f32>()*beta };
            let mut auv = uv0.to::<f32>() + (uv2-uv0).to::<f32>()*alpha;
            let mut buv = if second_half { uv1.to::<f32>() + (uv2-uv1).to::<f32>()*beta } else { uv0.to::<f32>() + (uv1-uv0).to::<f32>()*beta };
            if a.x>b.x{
                mem::swap(&mut a, &mut b);
                mem::swap(&mut auv, &mut buv);
            }
            for j in a.x as i32..b.x as i32+1 {
                let phi = if b.x == a.x { 1. } else { (j as f32 - a.x)/(b.x - a.x) };
                let p = (a + (b-a)*phi).to::<i32>();
                let puv = (auv + (buv-auv)*phi).to::<i32>();
                debug!("drawing point {} with color coords {}", p, puv);
                if self.zbuffer()[p.x as usize][p.y as usize]<p.z-1 { // -1 is hack, quick and dirty round-up problems solution
                    self.zbuffer()[p.x as usize][p.y as usize] = p.z;
                    self.set(p.x, p.y, get_gray(diffusemap.get(puv.x, puv.y), intensity));
                }
            }
        }
    }

}

