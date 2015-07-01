// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
extern crate sdl2;

mod canvas;

fn main() {
    println!("Hello World!");
    let mut canvas = canvas::Canvas::new(800, 600);
    canvas.point(400, 300);
    canvas.wait_for_esc();
}
