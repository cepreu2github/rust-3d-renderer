// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
use geometry::Vector3D;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;
use tgacanvas::TgaCanvas;
use canvas::Canvas;

pub struct Model {
    pub vertices: Vec<Vector3D<f32>>,
    pub faces : Vec<[[i32; 3]; 3]>,
    pub uv : Vec<[f32; 2]>,
    pub diffusemap : TgaCanvas,
}

// Изменения в модели: сохраняем "vt " и текстуру _diffuse
impl Model {
    pub fn new(file_path: &str) -> Model {
        let path = Path::new(file_path);
        let file = BufReader::new(File::open(&path).unwrap());
        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        let mut uv = Vec::new();
        for line in file.lines() {
            let line = line.unwrap();
            if line.starts_with("v ") {
                let words: Vec<&str> = line.split_whitespace().collect();
                vertices.push(Vector3D::new(words[1].parse().unwrap(), 
                                            words[2].parse().unwrap(),
                                            words[3].parse().unwrap()));
                debug!("readed vertex: {}", vertices.last().unwrap());
            } else if line.starts_with("f ") {
                let mut face: [[i32; 3]; 3] = [[-1; 3]; 3];
                let words: Vec<&str> = line.split_whitespace().collect();
                for i in 0..3 {
                    let mut j = 0;
                    for num in words[i+1].split("/") {
                        face[i][j] = num.parse::<i32>().unwrap() - 1;
                        j += 1;
                    } 
                    debug!("face[{}] = [{}, {}, {}]", i, face[i][0], face[i][1], face[i][2]);
                }
                faces.push(face);
            } else if line.starts_with("vt ") {
                let words: Vec<&str> = line.split_whitespace().collect();
                uv.push([words[1].parse().unwrap(), words[2].parse().unwrap()]);
                debug!("uv: [{}, {}]", uv.last().unwrap()[0], uv.last().unwrap()[1]);
            }
        }
        let texture_path = file_path.rsplitn(2, '.').last().unwrap().to_string() + "_diffuse.tga";
        info!("loading texture from path: {}", texture_path);
        Model {
            vertices: vertices,
            faces: faces,
            uv: uv,
            diffusemap: TgaCanvas::read(texture_path.split("*").next().unwrap()),
        }
    }
}

