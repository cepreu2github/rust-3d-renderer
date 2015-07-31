// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
use std;
use std::mem;
use canvas::Canvas;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io;

pub struct TgaCanvas {
    canvas: Vec<Vec<u32>>,
    zbuffer: Vec<Vec<i32>>,
    xsize: usize,
    ysize: usize,
}

impl TgaCanvas {
    fn create_canvas(xsize: usize, ysize: usize, bytespp: usize, buffer: &Vec<u8>) -> Vec<Vec<u32>>{
        let mut canvas = vec![vec![0;ysize];xsize];
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
        return canvas;
    }
    fn read_rle<T: Read>(pixelcount: usize, bytespp: usize, file: &mut T, encoded: &mut Vec<u8>){
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let mut pos = 0;
        let mut pix = 0;
        debug!("pixelcount = {}", pixelcount);
        while pix < pixelcount {
            debug!("pos = {}, pix = {}", pos, pix);
            let mut chunkheader = buffer[pos] as usize;
            debug!("chunkheader = {}", chunkheader);
            pos+=1;
            if chunkheader<128 {
                chunkheader+=1;
                let endpos = pos+chunkheader*bytespp;
                while pos < endpos {
                    encoded.push(buffer[pos]);
                    pos+=1;
                }
            } else {
                chunkheader -= 127;
                for _i in 0..chunkheader{
                    for j in 0..bytespp{
                        encoded.push(buffer[pos+j]);
                    }
                }
                pos+=bytespp;
            }
            pix+=chunkheader;
        }
    }
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
        let mut buffer: Vec<u8> = Vec::with_capacity(xsize*ysize*bytespp);
        if 3==header.datatypecode || 2==header.datatypecode{
            info!("read RAW");
            file.read_to_end(&mut buffer).unwrap();
        } else if 10==header.datatypecode||11==header.datatypecode{
            info!("read RLE");
            TgaCanvas::read_rle(xsize*ysize, bytespp, &mut file, &mut buffer);
        }
        info!("unpack to canvas");
        let mut canvas = TgaCanvas::create_canvas(xsize, ysize, bytespp, &buffer);
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
        TgaCanvas {
            canvas: canvas,
            zbuffer: vec![vec![std::i32::MIN; ysize]; xsize],
            xsize: xsize,
            ysize: ysize,
        }
    }

    fn out(&mut self) {
        let mut file = BufWriter::new(File::create("out.tga").unwrap());
        let header = TgaHeader {
        	idlength: 0,
        	colormaptype: 0,
	        datatypecode: 2,
	        colormaporigin: 0,
	        colormaplength: 0,
	        colormapdepth: 0,
	        x_origin: 0,
	        y_origin: 0,
	        width: self.xsize as i16,
	        height: self.ysize as i16,
	        bitsperpixel: 4<<3,
	        imagedescriptor: 0,
        };
        let header_bytes: [u8; HEADERSIZE] = unsafe { mem::transmute::<TgaHeader, [u8; HEADERSIZE]>(header) };
        file.write(&header_bytes).unwrap();
        for iy in 0..self.ysize{
            for ix in 0..self.xsize{
                let mut bytes: [u8; 4] = [0; 4];
                let point = self.canvas[ix][iy];
                bytes[0] = point as u8;
                bytes[1] = (point >> 8) as u8;
                bytes[2] = (point >> 16) as u8;
                bytes[3] = 255;
                file.write(&bytes).unwrap();
            }
        }
        let footer: [u8; 26] = [0, 0, 0, 0, // developer_area
                                0, 0, 0, 0, //extension_area
                                0x54, 0x52, 0x55, 0x45, 0x56, 0x49, 0x53, 0x49, 0x4f, 0x4e, 0x2d, 0x58, 0x46, 0x49, 0x4c, 0x45, 0x2e, 0x00];
        file.write(&footer).unwrap();
    }
    
    fn wait_for_enter(&mut self) {
        println!("press ENTER to continue");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        debug!("{}", guess);
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
