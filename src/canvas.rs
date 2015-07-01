// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
extern crate sdl2;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;

pub struct Canvas<'_> {
    renderer: &'_ mut Renderer<'_>,
}

impl<'_> Canvas<'_> {
    pub fn new(x: u32, y: u32) -> Canvas<'_> {
        let mut sdl_context = sdl2::init().video().unwrap();

        let window = sdl_context.window("rust-3d-renderer", x, y)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut renderer = window.renderer().build().unwrap();
       
        Canvas { renderer: &mut renderer }
    }
    
    pub fn point(&self, x: u32, y: u32) {
        // FIXME: rework it
        let mut texture = self.renderer.create_texture_streaming(PixelFormatEnum::RGB24, (256, 256)).unwrap();
        // Create a red-green gradient
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in (0..256) {
                for x in (0..256) {
                    let offset = y*pitch + x*3;
                    buffer[offset + 0] = x as u8;
                    buffer[offset + 1] = y as u8;
                    buffer[offset + 2] = 0;
                }
            }
        }).unwrap();

        self.renderer.clear();
        self.renderer.copy(&texture, None, Some(Rect::new_unwrap(100, 100, 256, 256)));
        self.renderer.copy_ex(&texture, None, Some(Rect::new_unwrap(450, 100, 256, 256)), 30.0, None, (false, false));
        self.renderer.present();
    }
    
    pub fn wait_for_ESC(&self) {
        let mut sdl_context = sdl2::init().video().unwrap();

        let mut running = true;

        while running {
            for event in sdl_context.event_pump().poll_iter() {
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

