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
//const BLUE: u32 = 0x0000FF;
const GREEN: u32 = 0x00FF00;
const WIDTH: usize = 200;
const HEIGHT: usize = 200;

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
    canvas.triangle(10, 70, 50, 160, 70, 80, RED);
    canvas.triangle(180, 50, 150, 1, 70, 180, WHITE);
    canvas.triangle(180, 150, 120, 160, 130, 180, GREEN);
    info!("drawing result");
    canvas.show();
    info!("waiting for ESC");
    canvas.wait_for_esc();
}
