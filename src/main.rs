// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
extern crate sdl2;
#[macro_use]
extern crate log;
extern crate env_logger;

mod canvas;
mod geometry;
mod model;

use model::Model;

const WHITE: u32 = 0xFFFFFF;
const RED: u32 = 0xFF0000;
const BLUE: u32 = 0x0000FF;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

#[test]
fn test_line() {
    let mut canvas = canvas::Canvas::new(100, 100);
    canvas.line(13, 20, 80, 40, WHITE);
    canvas.line(20, 13, 40, 80, RED);
    canvas.line(80, 40, 13, 20, BLUE);
}

fn main() {
    env_logger::init().unwrap();
    info!("starting up");
    let model = Model::new("african_head.obj");
    let mut canvas = canvas::Canvas::new(WIDTH, HEIGHT);
    debug!("Drawing wireframe...");
    for face in model.faces {
        debug!("Processing face:");
        debug!("({}, {}, {})", face[0], face[1], face[2]);
        for j in 0..3 {
            let v0 = &model.vertices[face[j] as usize];
            let v1 = &model.vertices[face[(j+1)%3] as usize];
            let x0 = ((v0.x+1.)*WIDTH as f32/2.) as i32;
            let y0 = ((v0.y+1.)*HEIGHT as f32/2.) as i32;
            let x1 = ((v1.x+1.)*WIDTH as f32/2.) as i32;
            let y1 = ((v1.y+1.)*HEIGHT as f32/2.) as i32;
            debug!("Drawing line ({}, {}) - ({}, {})", x0, y0, x1, y1); 
            canvas.line(x0, y0, x1, y1, WHITE);
        }
    }
    canvas.wait_for_esc();
}
