use std::{collections::HashMap, f32::consts::PI, ops::Sub, time::Instant};

use draw_target::{Color, DrawTarget, FrameBuffer, Triangle2D, BLACK, RED, WHITE};

use mat::Matrix;
use model::{Model, Texture};
use util::DisplayWindow;
use vec::{Vector2, Vector3, Vector4};

mod draw_target;
mod mat;
mod model;
mod transform;
mod util;
mod vec;

fn main() {
    // let mut obj = Model::load_from_obj("assets/african_head.obj");
    // let diffuse_map = Texture::load_from_tga("assets/african_head_diffuse.tga");
    let obj = Model::load_from_obj("assets/芙宁娜.obj");

    let pic_map = HashMap::from([
        ("spa_h.png", Texture::load_from("assets/芙宁娜/spa_h.png")),
        ("体.png", Texture::load_from("assets/芙宁娜/体.png")),
        ("颜.png", Texture::load_from("assets/芙宁娜/颜.png")),
        ("髮.png", Texture::load_from("assets/芙宁娜/髮.png")),
        ("髮2.png", Texture::load_from("assets/芙宁娜/髮2.png")),
    ]);
    let texture_map = HashMap::from([
        ("颜", "颜.png"),
        ("颜2", "颜.png"),
        ("二重", "颜.png"),
        ("睫", "颜.png"),
        ("口舌", "颜.png"),
        ("齿", "颜.png"),
        ("眉", "颜.png"),
        ("白目", "颜.png"),
        ("目", "髮.png"),
        ("星目", "颜.png"),
        ("髮", "髮.png"),
        ("髮2", "髮2.png"),
        ("体", "体.png"),
        ("服饰", "体.png"),
        ("体2", "髮.png"),
        ("蝴蝶结1", "髮.png"),
        ("服饰2", "髮.png"),
        ("裤", "髮.png"),
        ("神之眼", "髮.png"),
        ("流苏", "髮.png"),
        ("肌", "髮.png"),
        ("花边", "体.png"),
        ("裙", "体.png"),
        ("裙摆", "体.png"),
        ("裙1", "体.png"),
        ("裙2", "体.png"),
        ("裙3", "体.png"),
        ("裙饰", "体.png"),
        ("裙摆+", "体.png"),
        ("裙1+", "体.png"),
        ("裙2+", "体.png"),
        ("髮+", "spa_h.png"),
    ]);
    let (w, h) = (240, 240);
    let mut window = DisplayWindow::new(w, h);

    let mut eye = Vector3::new([0.0, 0.0, 1.0]);
    let mut look_at = Vector3::new([0.0, 0.0, -1.0]);
    let up = Vector3::new([0.0, 1.0, 0.0]);

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
        // let r = 0.0;
        window.fb.clear();
        let light_dir = Vector3::new([0.0, 0.0, -1.0]);
        let mut zbuffer = FrameBuffer::<f32>::new(w, h);
        zbuffer.fill(-f32::MAX);
        for i in 0..obj.faces_count() {
            // 获取[(坐标序号，UV坐标序号，法向量序号)]
            let (face, mtl) = obj.get_face(i);
            let pic = &pic_map[texture_map[obj.get_mtl(mtl)]];
            // 分别对三个顶点做变换
            let t = (0..3)
                .map(|j| {
                    // 模型坐标系中得到模型坐标
                    let wc = obj.get_vertex(face[j].0).to_homo_coord();

                    // 模型变换到世界坐标系
                    let model_matrix = transform::translate(Vector3::new([0.0, 0.0, -3.0]))
                        * transform::rotate(Vector3::new([0.0, 1.0, 0.0]), r);
                    let wc = model_matrix * wc;
                    let norm = transform::rotate(Vector3::new([0.0, 1.0, 0.0]), r)
                        * obj.get_normal(face[j].2).to_homo_coord();

                    let wc_src = Vector3::from_homo_coord(wc);
                    let norm_src = Vector3::from_homo_coord(norm);

                    let intensity = 1.0 - norm_src.dot(light_dir);
                    let intensity = intensity * 0.5;
                    // println!("intensity: {}", intensity);
                    // 相机变换到相机坐标系
                    let wc = transform::camera(eye, look_at, up) * wc;

                    // 投影变换到规范化坐标系
                    let wc =
                        transform::persp_by_fov(PI / 4.0, w as f32 / h as f32, -0.1, 50.0) * wc;

                    // 齐次坐标系映射到笛卡尔坐标系
                    let wc = Vector3::from_homo_coord(wc);

                    // 视口变换到屏幕坐标系
                    let x0 = ((wc.x() + 1.0) / 2.0 * w as f32) as i32;
                    let y0 = ((wc.y() + 1.0) / 2.0 * h as f32) as i32;

                    // 世界坐标，屏幕坐标，屏幕深度，uv坐标, 法向量，光照强度
                    (
                        wc_src,
                        Vector2::new([x0, y0]),
                        wc.z(),
                        obj.get_uv(face[j].1),
                        norm_src,
                        intensity,
                    )
                })
                .collect::<Vec<_>>();

            // 若<0，则为不可见平面，这里进行背面剔除
            // if intensity > 0.0 {
            window.fb.draw_trangle_with_zbuffer(
                Triangle2D {
                    a: t[0].1,
                    b: t[1].1,
                    c: t[2].1,
                    depth: Vector3::new([t[0].2, t[1].2, t[2].2]),

                    uv_a: t[0].3,
                    uv_b: t[1].3,
                    uv_c: t[2].3,

                    intensity: Vector3::new([t[0].5, t[1].5, t[2].5]),
                },
                &mut zbuffer,
                |uv| pic.get_color(uv),
            );
            // }
        }
        let e = window.update();
        {
            use util::Event::*;
            match e {
                Nothing => {}
                Go => {
                    eye = eye + look_at.normalize() * 0.2;
                }
                Back => {
                    eye = eye - look_at.normalize() * 0.2;
                }
                Up => {
                    eye = eye + up.normalize() * 0.2;
                }
                Down => {
                    eye = eye - up.normalize() * 0.2;
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
