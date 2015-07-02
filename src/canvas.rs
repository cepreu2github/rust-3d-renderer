// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub struct Canvas<'_> {
    renderer: Renderer<'_>,
    sdl_context: Sdl,
}

impl<'_> Canvas<'_> {
    pub fn new(x: u32, y: u32) -> Canvas<'_> {
        let sdl_context = sdl2::init().video().unwrap();

        let window = sdl_context.window("rust-3d-renderer", x, y)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let renderer = window.renderer().build().unwrap();
       
        Canvas { renderer: renderer, sdl_context: sdl_context }
    }
    
    pub fn set(&mut self, x: i32, y: i32, color: u32) {
        self.renderer.set_draw_color(Color::RGB((color >> (8*2)) as u8, (color >> (8*1)) as u8, color as u8));
        self.renderer.draw_point(Point::new(x, y));
        self.renderer.present();
    }
    
    pub fn wait_for_esc(&mut self) {

        let mut running = true;

        while running {
            for event in self.sdl_context.event_pump().poll_iter() {
                use sdl2::event::Event;

                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        running = false
                    },
                    _ => {}
                }
            }
        }
    }

}

