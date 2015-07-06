use geometry::Vector3D;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;

pub struct Model {
    vertices: Vec<Vector3D>,
    faces : Vec<[i32; 3]>,
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
                    face[i] = 0;
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


/*Model::Model(const char *filename) : verts_(), faces_() {
    std::ifstream in;
    in.open (filename, std::ifstream::in);
    if (in.fail()) return;
    std::string line;
    while (!in.eof()) {
        std::getline(in, line);
        std::istringstream iss(line.c_str());
        char trash;
        if (!line.compare(0, 2, "v ")) {
            iss >> trash;
            Vec3f v;
            for (int i=0;i<3;i++) iss >> v.raw[i];
            verts_.push_back(v);
        } else if (!line.compare(0, 2, "f ")) {
            std::vector<int> f;
            int itrash, idx;
            iss >> trash;
            while (iss >> idx >> trash >> itrash >> trash >> itrash) {
                idx--; // in wavefront obj all indices start at 1, not zero
                f.push_back(idx);
            }
            faces_.push_back(f);
        }
    }
    std::cerr << "# v# " << verts_.size() << " f# "  << faces_.size() << std::endl;
}

Model::~Model() {
}

int Model::nverts() {
    return (int)verts_.size();
}

int Model::nfaces() {
    return (int)faces_.size();
}

std::vector<int> Model::face(int idx) {
    return faces_[idx];
}

Vec3f Model::vert(int i) {
    return verts_[i];
}*/
