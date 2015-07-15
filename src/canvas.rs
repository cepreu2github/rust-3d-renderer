// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
use std;
use sdl2;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;
use sdl2::render::Renderer;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use std::mem;
use geometry::Vector3D;

pub struct Canvas {
    sdl_context: Sdl,
    renderer: Renderer<'static>,
    canvas: Vec<Vec<u32>>,
    zbuffer: Vec<Vec<i32>>,
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
            zbuffer: vec![vec![std::i32::MIN; y]; x],
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

    pub fn triangle(&mut self, mut p0: Vector3D<i32>, mut p1: Vector3D<i32>, mut p2: Vector3D<i32>, color: u32) {
        if p0.y==p1.y && p0.y==p2.y {
            return; // i dont care about degenerate triangles
        }
        // sort the vertices, t0, t1, t2 lower-to-upper (bubblesort yay!)
        if p0.y > p1.y {
            mem::swap(&mut p0, &mut p1);
        }
        if p0.y > p2.y {
            mem::swap(&mut p0, &mut p2);
        }
        if p1.y > p2.y {
            mem::swap(&mut p1, &mut p2);
        }
        let total_height = p2.y - p0.y;
        for i in 0..total_height {
            let second_half = i > p1.y - p0.y || p1.y == p0.y;
            let segment_height = if second_half { p2.y - p1.y } else { p1.y - p0.y };
            let alpha = i as f32/total_height as f32;
            let beta  = (i - if second_half { p1.y - p0.y } else { 0 }) as f32/segment_height as f32; // be careful: with above conditions no division by zero here
            let mut a = p0.to::<f32>() + (p2-p0).to::<f32>()*alpha;
            let mut b = if second_half { p1.to::<f32>() + (p2-p1).to::<f32>()*beta } else { p0.to::<f32>() + (p1-p0).to::<f32>()*beta };
            if a.x>b.x{
                mem::swap(&mut a, &mut b);
            }
            for j in a.x as i32..b.x as i32+1 {
                let phi = if b.x == a.x { 1. } else { (j as f32 - a.x)/(b.x - a.x) };
                let p = (a + (b-a)*phi).to::<i32>();
                if self.zbuffer[p.x as usize][p.y as usize]<p.z {
                    self.zbuffer[p.x as usize][p.y as usize] = p.z;
                    self.set(p.x, p.y, color);
                }
            }
        }
    }

}

