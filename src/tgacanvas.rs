// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
use std;
use std::mem;
use canvas::Canvas;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

pub struct TgaCanvas {
    pub canvas: Vec<Vec<u32>>,
    zbuffer: Vec<Vec<i32>>,
    xsize: usize,
    ysize: usize,
}

impl Canvas for TgaCanvas {
    fn new(x: usize, y: usize) -> TgaCanvas {
        TgaCanvas {
            canvas: vec![vec![0;y];x],
            zbuffer: vec![vec![std::i32::MIN; y]; x],
            xsize: x,
            ysize: y,
        }
    }

    fn read(path: &str) -> TgaCanvas{
        let path = Path::new(path);
        let mut file = BufReader::new(File::open(&path).unwrap());
        let mut header_bytes: [u8; HEADERSIZE] = [0; HEADERSIZE];
        file.read(&mut header_bytes).unwrap();
        let header = unsafe { mem::transmute::<[u8; HEADERSIZE], TgaHeader>(header_bytes) };
        let xsize = header.width as usize;
        let ysize = header.height as usize;
        info!("read header: width = {}, height = {}", xsize, ysize);
        let bytespp = (header.bitsperpixel>>3) as usize;
        info!("bytes per pixel - {}", bytespp);
        let mut canvas = vec![vec![0;ysize];xsize];
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        for iy in 0..ysize{
            for ix in 0..xsize{
                if bytespp == 1 {
                    let intensity = buffer[iy*xsize+ix] as u32;
                    canvas[ix][iy] = intensity + (intensity << (8*1)) + (intensity << (8*2));
                } else if bytespp == 3 {
                    let bytes = &buffer[(iy*xsize+ix)*3..(iy*xsize+ix+1)*3];
                    canvas[ix][iy] = bytes[0] as u32 + ((bytes[1] as u32) << (8*1)) + ((bytes[2] as u32) << (8*2));
                } else if bytespp == 4 {
                    let bytes = &buffer[(iy*xsize+ix)*4..(iy*xsize+ix+1)*4];
                    canvas[ix][iy] = bytes[0] as u32 + ((bytes[1] as u32) << (8*1)) + ((bytes[2] as u32) << (8*2));
                }
            }
        }
    	if header.imagedescriptor&0x20 > 0 {
    	    info!("vertical flip");
            for iy in 0..ysize/2{
                for ix in 0..xsize{
                    let point1 = canvas[ix][iy];
                    let point2 = canvas[ix][ysize-1-iy];
                    canvas[ix][iy] = point2;
                    canvas[ix][ysize-1-iy] = point1;
                }
            }
        }
        // проверить количество бит на пиксель и рассчитать размер буфера
        // если рле
            // прочитать рле
            // считать содержимое файла в буфер
        
        TgaCanvas {
            canvas: canvas,
            zbuffer: vec![vec![std::i32::MIN; ysize]; xsize],
            xsize: xsize,
            ysize: ysize,
        }
    }

    fn out(&mut self) {
    
    }
    
    fn wait_for_enter(&mut self) {
        
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

const HEADERSIZE: usize = 18; // 18 = sizeof(TgaHeader)
#[repr(C, packed)]
struct TgaHeader {
	idlength: i8,
	colormaptype: i8,
	datatypecode: i8,
	colormaporigin: i16,
	colormaplength: i16,
	colormapdepth: i8,
	x_origin: i16,
	y_origin: i16,
	width: i16,
	height: i16,
	bitsperpixel: i8,
	imagedescriptor: i8,
	
}
