// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
extern crate sdl2;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rand;

mod canvas;
mod geometry;
mod model;

use rand::Rng;
use model::Model;

//const WHITE: u32 = 0xFFFFFF;
//const RED: u32 = 0xFF0000;
//const BLUE: u32 = 0x0000FF;
//const GREEN: u32 = 0x00FF00;
const WIDTH: usize = 700;
const HEIGHT: usize = 700;

#[test]
fn test_line() {
    let mut canvas = canvas::Canvas::new(100, 100);
    canvas.line(13, 20, 80, 40, WHITE);
    canvas.line(20, 13, 40, 80, RED);
    canvas.line(80, 40, 13, 20, BLUE);
}

#[test]
fn test_triangle() {
    let mut canvas = canvas::Canvas::new(200, 200);
    canvas.triangle(10, 70, 50, 160, 70, 80, RED);
    canvas.triangle(180, 50, 150, 1, 70, 180, WHITE);
    canvas.triangle(180, 150, 120, 160, 130, 180, GREEN);
}

fn main() {
    env_logger::init().unwrap();
    info!("starting up");
    let model = Model::new("african_head.obj");
    let mut canvas = canvas::Canvas::new(WIDTH, HEIGHT);
    debug!("drawing wireframe");
    for face in model.faces {
        debug!("processing face:");
        debug!("({}, {}, {})", face[0], face[1], face[2]);
        let mut p: [[i32; 2]; 3] = [[0; 2]; 3];
        for j in 0..3 {
            let x = model.vertices[face[j] as usize].x;
            let y = model.vertices[face[j] as usize].y;
            p[j][0] = ((x+1.)*WIDTH as f32/2.) as i32;
            p[j][1] = ((y+1.)*HEIGHT as f32/2.) as i32;
        }
        canvas.triangle(p[0][0], p[0][1], p[1][0], p[1][1], p[2][0], p[2][1], rand::thread_rng().gen_range(0x000000, 0xFFFFFF));
    }
    info!("drawing result");
    canvas.show();
    info!("waiting for ESC");
    canvas.wait_for_esc();
}
