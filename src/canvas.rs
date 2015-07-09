// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
use sdl2;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;
use sdl2::render::Renderer;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use std::mem;

pub struct Canvas {
    sdl_context: Sdl,
    renderer: Renderer<'static>,
    canvas: Vec<Vec<u32>>,
    xsize: usize,
    ysize: usize,
}

impl Canvas {
    pub fn new(x: usize, y: usize) -> Canvas {
        let sdl_context = sdl2::init().video().unwrap();

        let window = sdl_context.window("rust-3d-renderer", x as u32, y as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let renderer = window.renderer().build().unwrap();

        Canvas {
            sdl_context: sdl_context,
            renderer: renderer,
            canvas: vec![vec![0;y];x], 
            xsize: x,
            ysize: y,
        }
    }

    pub fn show(&mut self) {
        let mut texture = self.renderer.create_texture_streaming(PixelFormatEnum::RGB24, 
                                       (self.xsize as u32, self.ysize as u32)).unwrap();
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in (0..self.ysize) {
                for x in (0..self.xsize) {
                    let offset = y*pitch + x*3;
                    let color = self.canvas[x][self.ysize - y - 1];
                    buffer[offset + 0] = (color >> (8*2)) as u8;
                    buffer[offset + 1] = (color >> (8*1)) as u8;
                    buffer[offset + 2] = color as u8;
                }
            }
        }).unwrap();

        self.renderer.clear();
        self.renderer.copy(&texture, None, Some(Rect::new_unwrap(0, 0, 
                                                self.xsize as u32, self.ysize as u32)));
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

    pub fn set(&mut self, x: i32, y: i32, color: u32) {
        if x < 0 || y < 0 {
            return;
        }
        if x >= self.xsize as i32 || y >= self.ysize as i32{
            return; 
        }
        self.canvas[x as usize][y as usize] = color;
    }

    pub fn line(&mut self, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, color: u32) {
        let mut steep = false;
        if (x0-x1).abs() < (y0-y1).abs() {
            mem::swap(&mut x0, &mut y0);
            mem::swap(&mut x1, &mut y1);
            steep = true;
        }
        if x0>x1 {
            mem::swap(&mut x0, &mut x1);
            mem::swap(&mut y0, &mut y1);
        }
        let dx = x1-x0;
        let dy = y1-y0;
        let derror2 = dy.abs()*2;
        let mut error2 = 0;
        let mut y = y0;
        for x in x0..x1+1 {
            if steep {
                self.set(y, x, color);
            } else {
                self.set(x, y, color);
            }
            error2 += derror2;

            if error2 > dx {
                y += if y1>y0 { 1 } else { -1 };
                error2 -= dx*2;
            }
        }
    }

}

