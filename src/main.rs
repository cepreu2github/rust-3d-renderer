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
use geometry::Matrix;

//const WHITE: u32 = 0xFFFFFF;
//const RED: u32 = 0xFF0000;
//const BLUE: u32 = 0x0000FF;
//const GREEN: u32 = 0x00FF00;
const WIDTH: usize = 700;
const HEIGHT: usize = 700;
const DEPTH: usize = 255;

fn main() {
    env_logger::init().unwrap();
    info!("starting up");
    let light_direction = Vector3D::new(0.0, 0.0, -1.0);
    let camera = Vector3D::new(0.0, 0.0, 3.0);   
    let mut model = Model::new("obj_african/african_head.obj");
    let mut canvas: SdlCanvas = Canvas::new(WIDTH, HEIGHT);
    info!("drawing model");
    let mut projection = Matrix::identity(4);
    let view_port   = Matrix::viewport(WIDTH/8, HEIGHT/8, WIDTH*3/4, HEIGHT*3/4, DEPTH);
    projection[3][2] = -1.0/camera.z;
    for i in 0..model.faces.len() {
        let face = model.faces[i];
        debug!("processing face:");
        debug!("({}, {}, {})", face[0][0], face[1][0], face[2][0]);
        let mut p: [Vector3D<i32>; 3] = [Vector3D::new(0, 0, 0); 3];
        let mut world_p: [Vector3D<f32>; 3] = [Vector3D::new(0.0, 0.0, 0.0); 3];
        for j in 0..3 {
            world_p[j] = model.vertices[face[j][0] as usize];
            let mul = &view_port*&projection;
            p[j] = (Matrix::m2v(&mul*&Matrix::v2m(world_p[j]))).to::<i32>();
        }
        let n = (world_p[2]-world_p[0])^(world_p[1]-world_p[0]);
        let n = n.normalized(1.0);
        let intensity = n*light_direction;
        if intensity>0.0 {
            canvas.triangle(p[0], p[1], p[2], model.uv(i, 0), model.uv(i, 1), model.uv(i, 2), intensity, &mut model.diffusemap);
        }
    }
    info!("drawing result");
    canvas.out();
    info!("waiting for Enter");
    canvas.wait_for_enter();
}
