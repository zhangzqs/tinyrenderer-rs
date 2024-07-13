use std::{fs::File, io::Read};

use embedded_graphics::{
    geometry::Dimensions,
    pixelcolor::{Rgb888, RgbColor},
};
use image::{GenericImageView, ImageBuffer};
use tinytga::Tga;

use crate::{
    draw_target::{Color, FrameBuffer},
    vec::{Vector2, Vector3},
};

pub struct Model {
    /// 三维顶点坐标列表
    vertexs: Vec<Vector3<f32>>,
    /// 法向量列表
    normals: Vec<Vector3<f32>>,
    /// 纹理UV坐标列表
    texture_vertexs: Vec<Vector2<f32>>,
    /// 面片解析[[(三维坐标序号, UV坐标序号, 法向量序号);3];n]
    faces: Vec<(Vec<(usize, usize, usize)>, isize)>,
    /// mtl
    mtls: Vec<String>,
}

impl Model {
    pub fn load_from_obj(filename: &str) -> Self {
        let mut vertexs = Vec::new();
        let mut texture_vertexs = Vec::new();
        let mut faces = Vec::new();
        let mut normals = Vec::new();
        let mut mtls = Vec::new();
        let mut s = String::new();

        File::open(filename)
            .unwrap()
            .read_to_string(&mut s)
            .unwrap();

        let mut current_mtl_id = -1;
        s.split_terminator('\n').for_each(|line| {
            let mut whitespace_split_res = line.split_whitespace();
            let first_flag = whitespace_split_res.next();
            match first_flag {
                Some("usemtl") => {
                    let mtl = whitespace_split_res.next().unwrap().to_string();
                    current_mtl_id += 1;
                    mtls.push(mtl);
                }
                // 顶点和法向量解析
                Some("v") | Some("vn") => {
                    let x = whitespace_split_res.next().unwrap().parse::<f32>().unwrap();
                    let y = whitespace_split_res.next().unwrap().parse::<f32>().unwrap();
                    let z = whitespace_split_res.next().unwrap().parse::<f32>().unwrap();
                    let tuple = Vector3::new([x, y, z]);
                    match first_flag {
                        Some("v") => vertexs.push(tuple),
                        Some("vn") => normals.push(tuple),
                        _ => unreachable!(),
                    }
                }
                // uv坐标解析
                Some("vt") => {
                    let x = whitespace_split_res.next().unwrap().parse::<f32>().unwrap();
                    let y = whitespace_split_res.next().unwrap().parse::<f32>().unwrap();
                    let tuple = Vector2::new([x, y]);
                    texture_vertexs.push(tuple);
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
                    faces.push((face, current_mtl_id));
                }
                _ => {}
            }
        });
        Self {
            vertexs,
            faces,
            texture_vertexs,
            normals,
            mtls,
        }
    }

    /// 获取顶点坐标
    pub fn get_vertex(&self, index: usize) -> Vector3<f32> {
        self.vertexs[index]
    }

    pub fn get_normal(&self, index: usize) -> Vector3<f32> {
        self.normals[index]
    }

    // 获取uv坐标
    pub fn get_uv(&self, index: usize) -> Vector2<f32> {
        self.texture_vertexs[index]
    }

    // 平面顶点列表，顶点由(坐标序号，UV坐标序号，法向量序号, 材质id)所表示
    pub fn get_face(&self, index: usize) -> ([(usize, usize, usize); 3], isize) {
        (
            [
                self.faces[index].0[0],
                self.faces[index].0[1],
                self.faces[index].0[2],
            ],
            self.faces[index].1,
        )
    }

    pub fn get_mtl(&self, index: isize) -> &str {
        &self.mtls[index as usize]
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
    pub fn get_color(&self, uv: Vector2<f32>) -> Color {
        let (u, v) = (uv.x(), uv.y());
        let x = (u * self.get_width() as f32) as i32;
        let y = ((1.0 - v) * self.get_height() as f32) as i32;
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

    pub fn load_from(filename: &str) -> Self {
        let img = image::io::Reader::open(filename).unwrap().decode().unwrap();
        let (w, h) = (img.width(), img.height());
        let mut fb = Self::new(w as i32, h as i32);

        for (x, y, c) in img.pixels() {
            let (r, g, b) = (c.0[0], c.0[1], c.0[2]);
            fb.set(x as i32, y as i32, Color { r, g, b });
        }
        fb
    }
}
