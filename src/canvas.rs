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
    xsize: u32,
    ysize: u32,
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
       
        Canvas { 
            renderer: renderer,
            sdl_context: sdl_context,
            xsize: x,
            ysize: y,
        }
    }
    
    pub fn set(&mut self, x: i32, y: i32, color: u32) {
        self.renderer.set_draw_color(Color::RGB((color >> (8*2)) as u8, (color >> (8*1)) as u8, color as u8));
        self.renderer.draw_point(Point::new(x, self.ysize as i32 - y));
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

    
    // ============================================ BELOW IS PLATFORM INDEPENDENT CODE

    pub fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) {
        if (x1 == x2) && (y1 == y2) {
            self.set(x1, y1, color);
        }
        if (x1-x2).abs() > (y1-y2).abs() {
            if x1 < x2 {
                let mut yi = y1;
                for xi in x1..x2+1 {
                    let p = (y2-y1)*xi + x2*y1 - x1*y2;
                    if yi*(x2-x1) < p + (y2-y1)/2 {
                        yi = yi+1;
                    }
                    self.set(xi, yi, color);
                }
            } else {
                let mut yi = y2;
                for xi in x2..x1+1 {
                    let p = (y1-y2)*xi + x1*y2 - x2*y1;
                    if yi*(x1-x2) < p + (y1-y2)/2 {
                        yi = yi+1;
                    }
                    self.set(xi, yi, color);
                }
            }
        } else {
            if y1 < y2 {
                let mut xi = x1;
                for yi in y1..y2+1 {
                    let p = yi*(x2-x1) - x2*y1 + x1*y2;
                    if xi*(y2-y1) < p + (y2-y1)/2 {
                        xi = xi+1;
                    }
                    self.set(xi, yi, color);
                }
            } else {
                let mut xi = x2;
                for yi in y2..y1+1 {
                    let p = yi*(x1-x2) - x1*y2 + x2*y1;
                    if xi*(y1-y2) < p + (y1-y2)/2 {
                        xi = xi+1;
                    }
                    self.set(xi, yi, color);
                }
            }
        }
    }

}

