use std::{collections::HashMap, f32::consts::PI, ops::Sub, time::Instant};

use draw_target::{Color, DrawTarget, FrameBuffer, Triangle2D};

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

    // let obj = Model::load_from_obj("assets/可莉.obj");
    // let pic_map = HashMap::from([
    //     ("spa_h.png", Texture::load_from("assets/可莉/spa_h.png")),
    //     ("体.png", Texture::load_from("assets/可莉/体.png")),
    //     ("颜.png", Texture::load_from("assets/可莉/颜.png")),
    //     ("髮.png", Texture::load_from("assets/可莉/髮.png")),
    //     ("肌.png", Texture::load_from("assets/可莉/肌.png")),
    // ]);
    // let texture_map = HashMap::from([
    //     ("颜", "颜.png"),
    //     ("颜2", "颜.png"),
    //     ("白目", "颜.png"),
    //     ("二重", "颜.png"),
    //     ("目", "体.png"),
    //     ("睫", "颜.png"),
    //     ("口舌", "颜.png"),
    //     ("齿", "颜.png"),
    //     ("眉", "颜.png"),
    //     ("星目", "颜.png"),
    //     ("髮", "髮.png"),
    //     ("帽", "髮.png"),
    //     ("饰", "髮.png"),
    //     ("头饰", "髮.png"),
    //     ("体", "体.png"),
    //     ("裙", "体.png"),
    //     ("裙饰", "髮.png"),
    //     ("裙饰2", "髮.png"),
    //     ("裤", "体.png"),
    //     ("神之眼框", "体.png"),
    //     ("神之眼AL", "体.png"),
    //     ("肌", "体.png"),
    //     ("肌2", "肌.png"),
    //     ("饰2", "髮.png"),
    //     ("帽饰alpha", "髮.png"),
    //     ("裙边alpha", "髮.png"),
    //     ("髮+", "spa_h.png"),
    // ]);

    let (w, h) = (1000, 1000);
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
        window.fb.clear();
        let light_dir = Vector3::new([0.0, 0.0, -1.0]);
        let mut zbuffer = FrameBuffer::<f32>::new(w, h);
        zbuffer.fill(-f32::MAX);
        let mvp = // to
            transform::scale(w as f32, h as f32, 1.0) // Viewport视口变换到屏幕坐标系
                * transform::scale(0.5, 0.5, 0.5) * transform::translate(Vector3::new([1.0, 1.0, 1.0])) // Scale规范化坐标系
                * transform::persp_by_fov(PI * 0.5, w as f32 / h as f32, -0.1, 50.0)  // Project投影变换到规范化坐标系
                * transform::camera(eye, look_at, up) // View相机变换到相机坐标系
                * transform::translate(Vector3::new([0.0, 0.0, -1.0])) * transform::rotate(Vector3::new([0.0, 1.0, 0.0]), r); // Model模型变换到世界坐标系
        for i in 0..obj.faces_count() {
            // 获取[(坐标序号，UV坐标序号，法向量序号)]
            let (face, mtl) = obj.get_face(i);
            let pic = &pic_map[texture_map[obj.get_mtl(mtl)]];
            // 分别对三个顶点做变换
            let t = (0..3)
                .map(|j| {
                    // 模型坐标系中得到模型坐标
                    let wc = obj.get_vertex(face[j].0).to_homo_coord();

                    // 法向量计算
                    let norm_src = Vector3::from_homo_coord(
                        transform::rotate(Vector3::new([0.0, 1.0, 0.0]), r)
                            * obj.get_normal(face[j].2).to_homo_coord(),
                    );

                    // 光照计算
                    let intensity = (1.0 - norm_src.dot(light_dir)) * 0.5;

                    // 齐次坐标系映射到笛卡尔坐标系
                    let wc = Vector3::from_homo_coord(mvp * wc); // 顶点各种变换

                    // 屏幕坐标，屏幕深度，uv坐标, 光照强度
                    (
                        Vector2::new([wc.x() as i32, wc.y() as i32]),
                        wc.z(),
                        obj.get_uv(face[j].1),
                        intensity,
                    )
                })
                .collect::<Vec<_>>();

            window.fb.draw_trangle_with_zbuffer(
                Triangle2D {
                    a: t[0].0,
                    b: t[1].0,
                    c: t[2].0,
                    depth: Vector3::new([t[0].1, t[1].1, t[2].1]),

                    uv_a: t[0].2,
                    uv_b: t[1].2,
                    uv_c: t[2].2,

                    intensity: Vector3::new([t[0].3, t[1].3, t[2].3]),
                },
                &mut zbuffer,
                |uv| pic.get_color(uv),
            );
        }
        let e = window.update();
        {
            use util::Event::*;
            match e {
                Nothing => {}
                Go => {
                    eye += look_at.normalize() * 0.2;
                }
                Back => {
                    eye -= look_at.normalize() * 0.2;
                }
                Up => {
                    eye += up.normalize() * 0.2;
                }
                Down => {
                    eye -= up.normalize() * 0.2;
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
