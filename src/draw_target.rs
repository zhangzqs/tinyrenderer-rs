use crate::{
    mat::Matrix,
    vec::{Vector2, Vector3},
};

#[derive(Default, Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn scale(self, fx: f32) -> Self {
        Self {
            r: (fx * self.r as f32) as u8,
            g: (fx * self.g as f32) as u8,
            b: (fx * self.b as f32) as u8,
        }
    }
}

pub struct Triangle2D {
    // 三个顶点坐标
    pub a: Vector2<i32>,
    pub b: Vector2<i32>,
    pub c: Vector2<i32>,

    // 三个坐标的深度值
    pub depth: Vector3<f32>,

    // 三个顶点的uv值
    pub uv_a: Vector2<f32>,
    pub uv_b: Vector2<f32>,
    pub uv_c: Vector2<f32>,

    // 三个顶点的光照强度
    pub intensity: Vector3<f32>,
}

impl Triangle2D {
    /// 计算一个三角形的重心坐标
    /// 重心坐标表示了一个点所对的三条边形成的三角形的面积比
    pub fn barycentric(&self, p: Vector2<i32>) -> Vector3<f32> {
        let (a, b, c) = (self.a, self.b, self.c);
        let (ac, ab, pa) = (c - a, b - a, a - p);
        let xs = Vector3::new([ab.x(), ac.x(), pa.x()]);
        let ys = Vector3::new([ab.y(), ac.y(), pa.y()]);

        let n = xs.cross(ys);
        if n.z().abs() > 0 {
            let u = n.x() as f32 / n.z() as f32;
            let v = n.y() as f32 / n.z() as f32;
            Vector3::new([1.0 - u - v, u, v])
        } else {
            Vector3::new([-1.0, 1.0, 1.0])
        }
    }

    /// 对三角形内部进行depth插值计算
    pub fn get_depth(&self, bc: Vector3<f32>) -> f32 {
        self.depth.dot(bc)
    }

    /// 对三角形内部进行uv插值计算
    /// 给定三角形内的一点，求出插值后的uv坐标
    pub fn get_uv(&self, bc: Vector3<f32>) -> Vector2<f32> {
        Matrix::new([self.uv_a, self.uv_b, self.uv_c]).transpose() * bc
    }

    pub fn get_instensity(&self, bc: Vector3<f32>) -> f32 {
        self.intensity.dot(bc)
    }

    pub fn bounding_box(&self, window_width: i32, window_height: i32) -> (i32, i32, i32, i32) {
        // 确定三角形的最小包围盒
        let (w, h) = (window_width, window_height);

        let xs = [self.a, self.b, self.c]
            .into_iter()
            .map(|x| x.x())
            .collect::<Vec<_>>();
        let ys = [self.a, self.b, self.c]
            .into_iter()
            .map(|x| x.y())
            .collect::<Vec<_>>();

        let (x_min, x_max) = (
            (*xs.iter().min().unwrap()).max(0),
            (*xs.iter().max().unwrap()).min(w - 1),
        );
        let (y_min, y_max) = (
            (*ys.iter().min().unwrap()).max(0),
            (*ys.iter().max().unwrap()).min(h - 1),
        );
        (x_min, x_max, y_min, y_max)
    }
}

pub trait DrawTarget {
    fn get_size(&self) -> (i32, i32);
    fn draw(&mut self, x: i32, y: i32, color: Color);

    fn draw_line_float(&mut self, start: Vector2<i32>, end: Vector2<i32>, color: Color) {
        let delta = end - start;
        let (dx, dy) = (delta.x(), delta.y());
        if dx.abs() >= dy.abs() {
            // 排序，使得start.x() <= end.x()
            let (start, end) = if start.x() > end.x() {
                (end, start)
            } else {
                (start, end)
            };

            let x0 = start.x();
            let y0 = start.y();
            let x1 = end.x();
            let y1 = end.y();
            // 由于是以x为基准遍历，所以需要dx >= dy
            for x in x0..=x1 {
                let t = (x - x0) as f32 / (x1 - x0) as f32;
                let y = (y0 as f32 * (1.0 - t) + y1 as f32 * t) as i32;
                self.draw(x, y, color);
            }
        } else {
            // 排序，使得start.y() <= end.y()
            let (start, end) = if start.y() > end.y() {
                (end, start)
            } else {
                (start, end)
            };

            let x0 = start.x();
            let y0 = start.y();
            let x1 = end.x();
            let y1 = end.y();
            // 反之需要是以y为基准遍历
            for y in y0..=y1 {
                let t = (y - y0) as f32 / (y1 - y0) as f32;
                let x = (x0 as f32 * (1.0 - t) + x1 as f32 * t) as i32;
                self.draw(x, y, color);
            }
        }
    }
    fn draw_line(&mut self, start: Vector2<i32>, end: Vector2<i32>, color: Color) {
        let delta = end - start;
        if delta.x().abs() >= delta.y().abs() {
            // 排序，使得start.x() <= end.x()
            let (start, end) = if start.x() > end.x() {
                (end, start)
            } else {
                (start, end)
            };

            let x0 = start.x();
            let y0 = start.y();
            let x1 = end.x();
            let y1 = end.y();
            let dx = x1 - x0;
            let dy = y1 - y0;
            // 绘制一个不陡峭的线段，即斜率小于1的线段，即dx>dy
            let mut y = y0;
            let derror2 = dy.abs() * 2;
            let mut error2 = 0;

            for x in start.x()..=end.x() {
                error2 += derror2;
                if error2 > dx {
                    y += if y1 > y0 { 1 } else { -1 };
                    error2 -= 2 * dx;
                }
                self.draw(x, y, color);
            }
        } else {
            // 绘制一个陡峭的线段，即斜率大于1的线段，即dx<dy
            // 排序，使得start.y() <= end.y()
            let (start, end) = if start.y() > end.y() {
                (end, start)
            } else {
                (start, end)
            };

            let x0 = start.x();
            let y0 = start.y();
            let x1 = end.x();
            let y1 = end.y();
            let dx = x1 - x0;
            let dy = y1 - y0;

            let mut x: i32 = x0;
            let derror2 = dx.abs() * 2;
            let mut error2 = 0;

            for y in start.y()..=end.y() {
                error2 += derror2;
                if error2 > dy {
                    x += if x1 > x0 { 1 } else { -1 };
                    error2 -= 2 * dy;
                }
                self.draw(x, y, color);
            }
        }
    }

    fn draw_triangle_strip(
        &mut self,
        t0: Vector2<i32>,
        t1: Vector2<i32>,
        t2: Vector2<i32>,
        color: Color,
    ) {
        self.draw_line(t0, t1, color);
        self.draw_line(t1, t2, color);
        self.draw_line(t2, t0, color);
    }

    fn draw_triangle(
        &mut self,
        t0: Vector2<i32>,
        t1: Vector2<i32>,
        t2: Vector2<i32>,
        color: Color,
    ) {
        let cross = |u: Vector2<i32>, v: Vector2<i32>| u.x() * v.y() - u.y() * v.x();
        let xs = [t0.x(), t1.x(), t2.x()];
        let ys = [t0.y(), t1.y(), t2.y()];
        let (x_min, x_max) = (xs.into_iter().min().unwrap(), xs.into_iter().max().unwrap());
        let (y_min, y_max) = (ys.into_iter().min().unwrap(), ys.into_iter().max().unwrap());
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let v = Vector2::new([x, y]);
                let p0 = t0 - v;
                let p1 = t1 - v;
                let p2 = t2 - v;
                let c1 = cross(p0, p1);
                let c2 = cross(p1, p2);
                let c3 = cross(p2, p0);
                // c1, c2, c3同号则在三角形内部
                if c1 * c2 >= 0 && c1 * c3 >= 0 && c2 * c3 >= 0 {
                    self.draw(x, y, color);
                }
            }
        }
    }

    fn draw_trangle_with_zbuffer(
        &mut self,
        t: Triangle2D,
        zbuffer: &mut FrameBuffer<f32>,
        texture_getter: impl Fn(Vector2<f32>) -> Color,
    ) {
        let (w, h) = self.get_size();
        let (x_min, x_max, y_min, y_max) = t.bounding_box(w, h);
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let p = Vector2::new([x, y]);
                let bc = t.barycentric(p);

                // 剔除三角形之外的点
                if bc.x() < 0.0 || bc.y() < 0.0 || bc.z() < 0.0 {
                    continue;
                }
                let z = t.get_depth(bc);

                let color = texture_getter(t.get_uv(bc));

                let intensity = t.get_instensity(bc);
                // let color = Color::new(128, 128, 128);
                if *zbuffer.get(x, y) < z {
                    zbuffer.set(x, y, z);
                    self.draw(x, y, color.scale(intensity));
                }
            }
        }
    }
}

#[derive(Default)]
pub struct FrameBuffer<D> {
    width: i32,
    height: i32,
    data: Vec<D>,
}

impl<D: Default + Clone + Copy> FrameBuffer<D> {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            data: vec![D::default(); (width * height) as usize],
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = D::default();
        }
    }

    pub fn fill(&mut self, data: D) {
        for i in 0..self.data.len() {
            self.data[i] = data;
        }
    }

    pub fn set(&mut self, x: i32, y: i32, data: D) {
        self.data[(self.width * y + x) as usize] = data;
    }

    pub fn get(&self, x: i32, y: i32) -> &D {
        // println!("get {x} {y}");
        &self.data[(self.width * y + x) as usize]
    }

    pub fn get_data(&self) -> &Vec<D> {
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut Vec<D> {
        &mut self.data
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }
}

impl DrawTarget for FrameBuffer<Color> {
    fn draw(&mut self, x: i32, y: i32, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }
        let y = self.height - y - 1;
        let index = (y * self.width + x) as usize;
        if index < self.data.len() {
            self.data[index] = color;
        }
    }

    fn get_size(&self) -> (i32, i32) {
        (self.width, self.height)
    }
}
