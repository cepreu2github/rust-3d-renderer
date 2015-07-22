// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
use std;
use sdl2;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;
use sdl2::render::Renderer;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use canvas::Canvas;

pub struct SdlCanvas {
    sdl_context: Sdl,
    renderer: Renderer<'static>,
    canvas: Vec<Vec<u32>>,
    zbuffer: Vec<Vec<i32>>,
    xsize: usize,
    ysize: usize,
}

impl Canvas for SdlCanvas {
    fn new(x: usize, y: usize) -> SdlCanvas {
        let sdl_context = sdl2::init().video().unwrap();

        let window = sdl_context.window("rust-3d-renderer", x as u32, y as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let renderer = window.renderer().build().unwrap();

        SdlCanvas {
            sdl_context: sdl_context,
            renderer: renderer,
            canvas: vec![vec![0;y];x],
            zbuffer: vec![vec![std::i32::MIN; y]; x],
            xsize: x,
            ysize: y,
        }
    }

    fn read(filename: &str) -> SdlCanvas{
        panic!("Not implemented: SdlCanvas can't be readed from {}", filename);
    }

    fn out(&mut self) {
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
    
    fn wait_for_enter(&mut self) {

        let mut running = true;

        while running {
            for event in self.sdl_context.event_pump().poll_iter() {
                use sdl2::event::Event;

                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                        running = false
                    },
                    _ => {}
                }
            }
        }
    }

    fn canvas(&mut self) -> &mut Vec<Vec<u32>>{
        &mut self.canvas
    }
    fn zbuffer(&mut self) -> &mut Vec<Vec<i32>>{
        &mut self.zbuffer
    }
    fn xsize(&self) -> usize{
        self.xsize
    }
    fn ysize(&self) -> usize{
        self.ysize
    }
}
