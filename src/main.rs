use draw_target::{FrameBuffer, DrawTarget, Color, WHITE, RED};

use model::Model;
use vec::Vector2;
use util::show_frame_buffer;


mod vec;
mod mat;
mod draw_target;
mod util;
mod model;


fn main() {
    let mut fb = FrameBuffer::new(800, 800);
    let obj = Model::load_from_obj("assets/african_head.obj");
    for i in 0..obj.faces_count() {
        let face = obj.get_face(i);
        for j in 0..3 {
            let v0 = obj.get_vertex(face[j]);
            let v1 = obj.get_vertex(face[(j + 1) % 3]);
            let x0 = (v0.x() + 1.0) / 2.0;
            let y0 = (v0.y() + 1.0) / 2.0;
            let x1 = (v1.x() + 1.0) / 2.0;
            let y1 = (v1.y() + 1.0) / 2.0;

            let y0 = 1.0 - y0;
            let y1 = 1.0 - y1;

            let x0 = (x0 * fb.get_width() as f32) as usize;
            let y0 = (y0 * fb.get_height() as f32) as usize;
            let x1 = (x1 * fb.get_width() as f32) as usize;
            let y1 = (y1 * fb.get_height() as f32) as usize;

            fb.draw_line(Vector2::new([x0, y0]), Vector2::new([x1, y1]), WHITE);
        }
    }
    
    show_frame_buffer(fb);
}
