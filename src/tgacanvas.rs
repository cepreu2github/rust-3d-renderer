// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
use std;
use std::mem;
use canvas::Canvas;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct TgaCanvas {
    canvas: Vec<Vec<u32>>,
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
        let mut file = File::open(&path).unwrap();
        let mut header_bytes: [u8; HEADERSIZE] = [0; HEADERSIZE];
        file.read(&mut header_bytes);
        let header = unsafe { mem::transmute::<[u8; HEADERSIZE], TgaHeader>(header_bytes) };
        debug!("read header: width = {}, height = {}", header.width, header.height);
        let bytespp = header.bitsperpixel>>3;
        
        // проверить количество бит на пиксель и рассчитать размер буфера
        // если рле
            // прочитать рле
            // считать содержимое файла в буфер
        
        panic!("Not implemented");
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
