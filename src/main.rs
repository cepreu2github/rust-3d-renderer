// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
extern crate sdl2;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rand;
extern crate num;

mod canvas;
mod sdlcanvas;
mod tgacanvas;
mod geometry;
mod model;

use model::Model;
use geometry::Vector3D;
use sdlcanvas::SdlCanvas;
use canvas::Canvas;

//const WHITE: u32 = 0xFFFFFF;
//const RED: u32 = 0xFF0000;
//const BLUE: u32 = 0x0000FF;
//const GREEN: u32 = 0x00FF00;
const WIDTH: usize = 700;
const HEIGHT: usize = 700;
const DEPTH: usize = 255;

fn get_gray(intensity: f32) -> u32 {
    debug!("intensity is {}", intensity);
    let mut result = (255.0*intensity) as u32;
    result += (255.0*intensity) as u32*256;
    result += (255.0*intensity) as u32*256*256;
    debug!("result is {:X}", result);
    return result;
}

fn main() {
    env_logger::init().unwrap();
    info!("starting up");
    let light_direction = Vector3D::new(0.0, 0.0, -1.0);
    let model = Model::new("obj_african/african_head.obj");
    let mut canvas: SdlCanvas = Canvas::new(WIDTH, HEIGHT);
    info!("drawing model");
    for face in model.faces {
        debug!("processing face:");
        debug!("({}, {}, {})", face[0], face[1], face[2]);
        let mut p: [Vector3D<i32>; 3] = [Vector3D::new(0, 0, 0); 3];
        let mut world_p: [Vector3D<f32>; 3] = [Vector3D::new(0.0, 0.0, 0.0); 3];
        for j in 0..3 {
            world_p[j] = model.vertices[face[j] as usize];
            p[j].x = ((world_p[j].x+1.)*WIDTH as f32/2.) as i32;
            p[j].y = ((world_p[j].y+1.)*HEIGHT as f32/2.) as i32;
            p[j].z = ((world_p[j].z+1.)*DEPTH as f32/2.) as i32;
        }
        let n = (world_p[2]-world_p[0])^(world_p[1]-world_p[0]);
        let n = n.normalized(1.0);
        let intensity = n*light_direction;
        if intensity>0.0 {
            canvas.triangle(p[0], p[1], p[2], get_gray(intensity));
        }
    }
    info!("drawing result");
    canvas.out();
    info!("waiting for Enter");
    canvas.wait_for_enter();
}
