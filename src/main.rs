use std::{f32::consts::PI, ops::Sub, time::Instant};

use draw_target::{DrawTarget, FrameBuffer};

use mat::Matrix;
use model::{Model, Texture};
use util::DisplayWindow;
use vec::{Vector3, Vector4};

mod draw_target;
mod mat;
mod model;
mod transform;
mod util;
mod vec;

fn main() {
    let obj = Model::load_from_obj("assets/african_head.obj");
    let texture = Texture::load_from_tga("assets/african_head_diffuse.tga");
    let (w, h) = (800, 800);
    let mut window = DisplayWindow::new(w, h);

    let mut eye = Vector3::new([0.0, 0.0, 20.0]);
    let mut look_at = Vector3::new([0.0, 0.0, -1.0]);
    let mut up = Vector3::new([0.0, 1.0, 0.0]);

    let mut fps = 0.0;
    let mut last_time = Instant::now();
    for angle in (0..1000).cycle() {
        fps += 1.0;
        if fps > 120.0 {
            let now = Instant::now();
            fps /= now.sub(last_time).as_secs_f32();
            println!("fps: {fps}");
            fps = 0.0f32;
            last_time = now;
        }
        let r = (angle as f32 / 1000.0) * 2.0 * PI;
        window.fb.clear();
        let light_dir = Vector3::new([0.0, 0.0, -1.0]);
        let mut zbuffer = FrameBuffer::<f32>::new(w, h);
        zbuffer.fill(-f32::MAX);
        for i in 0..obj.faces_count() {
            let face = obj.get_face(i);
            // 分别对三个顶点做变换
            let t = (0..3)
                .map(|j| {
                    // 模型坐标系中得到模型坐标
                    let wc = obj.get_vertex(face[j].0).to_homo_coord();

                    // 模型变换到世界坐标系
                    let wc = transform::translate(Vector3::new([0.0, 0.0, -3.0]))
                        * transform::rotate(Vector3::new([0.0, 1.0, 0.0]), r)
                        * wc;

                    // 相机变换到相机坐标系
                    let wc = transform::camera(eye, look_at, up) * wc;

                    // 投影变换到规范化坐标系
                    let wc = transform::persp(-1.0, 1.0, -1.0, 1.0, 100.0, -10.0) * wc;

                    // 齐次坐标系映射到笛卡尔坐标系
                    let wc = Vector3::from_homo_coord(wc);

                    // 视口变换到屏幕坐标系
                    let x0 = ((wc.x() + 1.0) / 2.0 * w as f32) as i32;
                    let y0 = ((wc.y() + 1.0) / 2.0 * h as f32) as i32;

                    // 世界坐标，屏幕坐标，uv坐标
                    (
                        wc,
                        Vector3::new([x0, y0, (wc.z() * 1000.0) as i32]),
                        obj.get_uv(face[j].1),
                    )
                })
                .collect::<Vec<_>>();
            // 计算三角形平面法向量
            let n = (t[2].0 - t[0].0).cross(t[1].0 - t[0].0).normalize();
            // // 计算平面法向量与光线向量的点乘，越接近1则cos夹角越接近0，即光线越强
            let intensity = n.dot(light_dir);
            // // 若<0，则为不可见平面，这里进行背面剔除
            if intensity > 0.0 {
                window.fb.draw_trangle_with_zbuffer(
                    [t[0].1, t[1].1, t[2].1],
                    &mut zbuffer,
                    [
                        texture.get_color(t[0].2[0], t[0].2[1]).scale(intensity),
                        texture.get_color(t[1].2[0], t[1].2[1]).scale(intensity),
                        texture.get_color(t[2].2[0], t[2].2[1]).scale(intensity),
                    ],
                );
            }
        }
        let e = window.update();
        {
            use util::Event::*;
            match e {
                Nothing => {}
                Go => {
                    eye = eye + look_at.normalize() * 0.5;
                }
                Back => {
                    eye = eye - look_at.normalize() * 0.5;
                }
                TurnLeft => {
                    look_at = Vector3::from_homo_coord(
                        transform::rotate(Vector3::new([0.0, 1.0, 0.0]), 0.01)
                            * look_at.to_homo_coord(),
                    );
                }
                TurnRight => {
                    look_at = Vector3::from_homo_coord(
                        transform::rotate(Vector3::new([0.0, 1.0, 0.0]), -0.01)
                            * look_at.to_homo_coord(),
                    );
                }
                Exit => return,
                _ => {}
            }
        }
    }
}
