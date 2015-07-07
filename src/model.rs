// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
use geometry::Vector3D;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;

pub struct Model {
    pub vertices: Vec<Vector3D>,
    pub faces : Vec<[i32; 3]>,
}

impl Model {
    pub fn new(file_path: &str) -> Model {
        let path = Path::new(file_path);
        let file = BufReader::new(File::open(&path).unwrap());
        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        for line in file.lines() {
            let line = line.unwrap();
            if line.starts_with("v ") {
                let words: Vec<&str> = line.split_whitespace().collect();
                vertices.push(Vector3D::new(words[1].parse().unwrap(), 
                                            words[2].parse().unwrap(),
                                            words[3].parse().unwrap()));
                debug!("Readed vertex: {}", line);
            } else if line.starts_with("f ") {
                let mut face: [i32; 3] = [-1, -1, -1];
                let words: Vec<&str> = line.split_whitespace().collect();
                for i in 0..3 {
                    for num in words[i+1].split("/"){
                        face[i] = num.parse().unwrap();
                        face[i] -= 1; // in wavefront obj all indices start at 1, not zero
                        break;
                    }
                    debug!("Face[{}] = {}", i, face[i]);
                }
            }
        }
        Model {
            vertices: vertices,
            faces: faces,
        }
    }
}
