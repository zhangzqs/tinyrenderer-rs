use crate::{
    mat::Matrix,
    vec::{Vector3, Vector4},
};

/// 定义一个左乘的旋转矩阵
pub fn rotate(n: Vector3<f32>, a: f32) -> Matrix<f32, 4, 4> {
    let mn = Matrix::new([n]);
    let mnt = mn.transpose();

    let m = Matrix::identity() * a.cos()
        + mnt * mn * (1.0 - a.cos())
        + Matrix::new([
            Vector3::new([0.0, -n.z(), n.y()]),
            Vector3::new([n.z(), 0.0, -n.x()]),
            Vector3::new([-n.y(), n.x(), 0.0]),
        ]) * a.sin();
    let mut ret = Matrix::<f32, 4, 4>::new_zero();
    for r in 0..3 {
        for c in 0..3 {
            ret.set(r, c, m.get(r, c));
        }
    }
    ret.set(3, 3, 1.0);
    ret
}

/// 定义一个平移矩阵
pub fn translate(delta: Vector3<f32>) -> Matrix<f32, 4, 4> {
    let mut ret = Matrix::identity();
    ret.set(0, 3, delta.x());
    ret.set(1, 3, delta.y());
    ret.set(2, 3, delta.z());
    ret.set(3, 3, 1.0);
    ret
}

/// 定义一个缩放矩阵
pub fn scale(sx: f32, sy: f32, sz: f32) -> Matrix<f32, 4, 4> {
    let mut ret = Matrix::identity();
    ret.set(0, 0, sx);
    ret.set(1, 1, sy);
    ret.set(2, 2, sz);
    ret
}

/// 定义一个相机变换矩阵
pub fn camera(
    position: Vector3<f32>,
    look_at: Vector3<f32>,
    up: Vector3<f32>,
) -> Matrix<f32, 4, 4> {
    let t_view = Matrix::new([
        Vector4::new([1.0, 0.0, 0.0, -position.x()]),
        Vector4::new([0.0, 1.0, 0.0, -position.y()]),
        Vector4::new([0.0, 0.0, 1.0, -position.z()]),
        Vector4::new([0.0, 0.0, 0.0, 1.0]),
    ]);
    let (g, t) = (look_at, up);
    let gt = g.cross(t);
    let r_view = Matrix::new([
        Vector4::new([gt.x(), gt.y(), gt.z(), 0.0]),
        Vector4::new([t.x(), t.y(), t.z(), 0.0]),
        Vector4::new([-g.x(), -g.y(), -g.z(), 0.0]),
        Vector4::new([0.0, 0.0, 0.0, 1.0]),
    ]);
    r_view * t_view
}

/// 定义一个平行投影变换矩阵
pub fn ortho(l: f32, r: f32, b: f32, t: f32, f: f32, n: f32) -> Matrix<f32, 4, 4> {
    Matrix::new([
        Vector4::new([2.0 / (r - l), 0.0, 0.0, 0.0]),
        Vector4::new([0.0, 2.0 / (t - b), 0.0, 0.0]),
        Vector4::new([0.0, 0.0, 2.0 / (n - f), 0.0]),
        Vector4::new([0.0, 0.0, 0.0, 1.0]),
    ]) * Matrix::new([
        Vector4::new([1.0, 0.0, 0.0, -(r + l) / 2.0]),
        Vector4::new([0.0, 1.0, 0.0, -(t + b) / 2.0]),
        Vector4::new([0.0, 0.0, 1.0, -(n + f) / 2.0]),
        Vector4::new([0.0, 0.0, 0.0, 1.0]),
    ])
}

/// 定义一个透视投影变换矩阵
pub fn persp(l: f32, r: f32, b: f32, t: f32, f: f32, n: f32) -> Matrix<f32, 4, 4> {
    ortho(l, r, b, t, f, n)
        * Matrix::new([
            Vector4::new([n, 0.0, 0.0, 0.0]),
            Vector4::new([0.0, n, 0.0, 0.0]),
            Vector4::new([0.0, 0.0, n + f, -n * f]),
            Vector4::new([0.0, 0.0, 1.0, 0.0]),
        ])
}

/// 通过一些角度定义透视投影变换矩阵
pub fn persp_by_fov(fov_y: f32, aspect_radio: f32, z_near: f32, z_far: f32) -> Matrix<f32, 4, 4> {
    let t = z_near.abs() * (fov_y / 2.0).tan();
    let b = -t;
    let r = t * aspect_radio;
    let l = -r;
    persp(l, r, b, t, z_far, z_near)
}
