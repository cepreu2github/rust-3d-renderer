// Copyright (C) Cepreu <cepreu.mail@gmail.com> under GPLv2 and higher
use geometry::Point3D;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;

pub struct Model {
    pub vertices: Vec<Point3D>,
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
                vertices.push(Point3D::new(words[1].parse().unwrap(), 
                                            words[2].parse().unwrap(),
                                            words[3].parse().unwrap()));
                debug!("readed vertex: {}", vertices.last().unwrap());
            } else if line.starts_with("f ") {
                let mut face: [i32; 3] = [-1, -1, -1];
                let words: Vec<&str> = line.split_whitespace().collect();
                for i in 0..3 {
                    face[i] = words[i+1].split("/").next().unwrap().parse().unwrap();
                    face[i] -= 1;
                    debug!("face[{}] = {}", i, face[i]);
                }
                faces.push(face);
            }
        }
        Model {
            vertices: vertices,
            faces: faces,
        }
    }
}

