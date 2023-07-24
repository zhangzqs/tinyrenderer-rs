use std::{fs::File, io::Read};

use crate::vec::Vector3;

pub struct Model {
    vertexs: Vec<Vector3<f32>>,
    faces: Vec<Vec<usize>>,
}

impl Model {
    pub fn new(vertexs: Vec<Vector3<f32>>, faces: Vec<Vec<usize>>) -> Self {
        Self { vertexs, faces }
    }

    pub fn load_from_obj(filename: &str) -> Self {
        let mut vertexs = Vec::new();
        let mut faces = Vec::new();

        let mut s = String::new();
        
        File::open(filename).unwrap().read_to_string(&mut s).unwrap();
        s.split_terminator('\n').for_each(|line| {
            let mut iter = line.split_whitespace();
            match iter.next() {
                Some("v") => {
                    let x = iter.next().unwrap().parse::<f32>().unwrap();
                    let y = iter.next().unwrap().parse::<f32>().unwrap();
                    let z = iter.next().unwrap().parse::<f32>().unwrap();
                    vertexs.push(Vector3::new([x, y, z]));
                },
                Some("f") => {
                    let mut face = Vec::new();
                    for s in iter {
                        let mut iter = s.split('/');
                        let i = iter.next().unwrap().parse::<usize>().unwrap();
                        face.push(i - 1);
                    }
                    faces.push(face);
                },
                _ => {},
            }
        });
        Self::new(vertexs, faces)
    }

    pub fn get_vertex(&self, index: usize) -> Vector3<f32> {
        self.vertexs[index]
    }

    pub fn get_face(&self, index: usize) -> &Vec<usize> {
        &self.faces[index]
    }

    pub fn vertexs_count(&self) -> usize {
        self.vertexs.len()
    }

    pub fn faces_count(&self) -> usize {
        self.faces.len()
    }
}