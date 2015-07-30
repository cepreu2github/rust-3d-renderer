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
        file.read(&mut header_bytes);
        let header = unsafe { mem::transmute::<[u8; HEADERSIZE], TgaHeader>(header_bytes) };
        let xsize = header.width as usize;
        let ysize = header.height as usize;
        debug!("read header: width = {}, height = {}", xsize, ysize);
        let bytespp = header.bitsperpixel>>3;
        debug!("bytes per pixel - {}", bytespp);
        let mut canvas = vec![vec![0;ysize];xsize];
        for iy in 0..ysize{
            for ix in 0..xsize{
                if bytespp == 1 {
                    let mut bytes: [u8; 1] = [0; 1];
                    file.read(&mut bytes);
                    let intensity = bytes[0] as u32;
                    canvas[ix][iy] = intensity + intensity*256 + intensity*256*256;
                } else if bytespp == 3 {
                    let mut bytes: [u8; 3] = [0; 3];
                    file.read(&mut bytes);
                    canvas[ix][iy] = bytes[2] as u32 + bytes[1] as u32*256 + bytes[0] as u32*256*256;
                } else if bytespp == 4 {
                    let mut bytes: [u8; 4] = [0; 4];
                    file.read(&mut bytes);
                    if ix == 0 { debug!("{} {} {} {}", bytes[0], bytes[1], bytes[2], bytes[3]); }
                    canvas[ix][iy] = bytes[2] as u32 + ((bytes[1] as u32) << (8*1)) + ((bytes[0] as u32) << (8*2));
                    //debug!("{}", canvas[ix][iy]);
                }
            }
            debug!("{}", canvas[0][iy]);
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
