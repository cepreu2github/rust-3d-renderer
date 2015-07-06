// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
extern crate sdl2;
#[macro_use]
extern crate log;
extern crate env_logger;

mod canvas;

const WHITE: u32 = 0xFFFFFF;
const RED: u32 = 0xFF0000;
const BLUE: u32 = 0x0000FF;

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
    
    let mut canvas = canvas::Canvas::new(100, 100);
    canvas.line(13, 20, 80, 40, WHITE);
    canvas.line(20, 13, 40, 80, RED);
    canvas.line(80, 40, 13, 20, BLUE);
    canvas.wait_for_esc();
}
