use std::{fs::File, io::Read};

use embedded_graphics::{
    geometry::Dimensions,
    pixelcolor::{Rgb888, RgbColor},
};
use tinytga::Tga;

use crate::{
    draw_target::{Color, FrameBuffer},
    vec::Vector3,
};

pub struct Model {
    vertexs: Vec<Vector3<f32>>,
    texture_vertexs: Vec<Vector3<f32>>,
    faces: Vec<Vec<(usize, usize, usize)>>,
}

impl Model {
    pub fn load_from_obj(filename: &str) -> Self {
        let mut vertexs = Vec::new();
        let mut texture_vertexs = Vec::new();
        let mut faces = Vec::new();

        let mut s = String::new();

        File::open(filename)
            .unwrap()
            .read_to_string(&mut s)
            .unwrap();
        s.split_terminator('\n').for_each(|line| {
            let mut whitespace_split_res = line.split_whitespace();
            let first_flag = whitespace_split_res.next();
            match first_flag {
                // 顶点解析
                Some("v") | Some("vn") | Some("vt") => {
                    let x = whitespace_split_res.next().unwrap().parse::<f32>().unwrap();
                    let y = whitespace_split_res.next().unwrap().parse::<f32>().unwrap();
                    let z = whitespace_split_res.next().unwrap().parse::<f32>().unwrap();
                    let tuple = Vector3::new([x, y, z]);
                    match first_flag {
                        Some("v") => vertexs.push(tuple),
                        Some("vt") => texture_vertexs.push(tuple),
                        _ => {}
                    }
                }
                // 面片解析
                Some("f") => {
                    let mut face = Vec::new();
                    for s in whitespace_split_res {
                        let mut iter = s.split('/');
                        let vi = iter.next().unwrap().parse::<usize>().unwrap();
                        let uvi = iter.next().unwrap().parse::<usize>().unwrap();
                        let vni = iter.next().unwrap().parse::<usize>().unwrap();
                        face.push((vi - 1, uvi - 1, vni - 1));
                    }
                    faces.push(face);
                }
                _ => {}
            }
        });
        Self {
            vertexs,
            faces,
            texture_vertexs,
        }
    }

    pub fn get_vertex(&self, index: usize) -> Vector3<f32> {
        self.vertexs[index]
    }

    pub fn get_uv(&self, index: usize) -> Vector3<f32> {
        self.texture_vertexs[index]
    }

    // 平面顶点列表，顶点由（坐标序号，UV坐标，法向量序号所表示）
    pub fn get_face(&self, index: usize) -> &Vec<(usize, usize, usize)> {
        &self.faces[index]
    }

    pub fn vertexs_count(&self) -> usize {
        self.vertexs.len()
    }

    pub fn faces_count(&self) -> usize {
        self.faces.len()
    }
}

pub type Texture = FrameBuffer<Color>;

impl Texture {
    pub fn get_color(&self, x: f32, y: f32) -> Color {
        let x = (x * self.get_width() as f32) as i32;
        let y = ((1.0 - y) * self.get_height() as f32) as i32;
        *self.get(x, y)
    }

    pub fn load_from_tga(filename: &str) -> Self {
        let mut buf = Vec::new();
        File::open(filename).unwrap().read_to_end(&mut buf).unwrap();
        let tga: Tga<Rgb888> = Tga::from_slice(&buf).unwrap();
        let (w, h) = tga.bounding_box().size.into();
        let mut fb = Self::new(w as i32, h as i32);

        for p in tga.pixels() {
            let (x, y) = p.0.into();
            let (r, g, b) = (p.1.r(), p.1.g(), p.1.b());
            fb.set(x, y, Color { r, g, b });
        }
        fb
    }
}
