use std::{f32::consts::PI, ops::Sub, thread, time::Instant};

use draw_target::{Color, DrawTarget, FrameBuffer, GREEN, RED, WHITE};

use model::Model;
use util::DisplayWindow;
use vec::{Vector2, Vector3};

mod draw_target;
mod mat;
mod model;
mod util;
mod vec;

fn main() {
    let obj = Model::load_from_obj("assets/african_head.obj");
    let (w, h) = (800, 800);
    let mut window = DisplayWindow::new(w, h);

    let mut fps = 0.0;
    let mut last_time = Instant::now();
    for angle in (0..1000).cycle() {
        fps += 1.0;
        if fps > 120.0 {
            let now = Instant::now();
            fps = fps as f32 / now.sub(last_time).as_secs_f32();
            println!("fps: {fps}");
            fps = 0.0f32;
            last_time = now;
        }
        let r = (angle as f32 / 1000.0) * 2.0 * PI;
        window.fb.clear();
        let light_dir = Vector3::new([0.0, 0.0, -1.0]);
        for i in 0..obj.faces_count() {
            let face = obj.get_face(i);
            let t = (0..3)
                .map(|j| {
                    // 世界坐标
                    let wc = obj.get_vertex(face[j]);

                    // 模型变换
                    let wc = Vector3::new([
                        wc.z() * r.sin() + wc.x() * r.cos(),
                        wc.y(),
                        wc.z() * r.cos() - wc.x() * r.sin(),
                    ]);
                    // 投影变换（平行投影）
                    let sc = Vector2::new([wc.x(), wc.y()]);
                    // 视口变换
                    let x0 = ((sc.x() + 1.0) / 2.0 * w as f32) as i32;
                    let y0 = ((sc.y() + 1.0) / 2.0 * h as f32) as i32;
                    (wc, Vector2::new([x0, y0]))
                })
                .collect::<Vec<_>>();
            // 计算三角形平面法向量
            let n = (t[2].0 - t[0].0).cross(t[1].0 - t[0].0).normalize();
            // 计算平面法向量与光线向量的点乘，越接近1则cos夹角越接近0，即光线越强
            let intensity = n.dot(light_dir);
            // 若<0，则为不可见平面，这里进行背面剔除
            if intensity > 0.0 {
                window.fb.draw_triangle(
                    t[0].1,
                    t[1].1,
                    t[2].1,
                    Color {
                        r: (intensity * 255.0) as u8,
                        g: (intensity * 255.0) as u8,
                        b: (intensity * 255.0) as u8,
                    },
                );
            }
        }
        let e = window.update();
        match e {
            util::Event::Nothing => {}
            util::Event::Exit => return,
        }
    }
}
