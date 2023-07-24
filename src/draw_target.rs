use crate::vec::Vector2;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub const BLACK: Color = Color::new(0, 0, 0);
pub const WHITE: Color = Color::new(255, 255, 255);
pub const RED: Color = Color::new(255, 0, 0);
pub trait DrawTarget {
    fn draw(&mut self, x: usize, y: usize, color: Color);

    fn draw_line(&mut self, start: Vector2<usize>, end: Vector2<usize>, color: Color) {
        // 排序，使得start.x() <= end.x()
        let (start, end) = if start.x() > end.x() {
            (end, start)
        } else {
            (start, end)
        };

        let x0 = start.x() as i32;
        let y0 = start.y() as i32;
        let x1 = end.x() as i32;
        let y1 = end.y() as i32;

        let dx = x1 - x0;
        let dy = y1 - y0;

        // bresenham 直线算法
        if dx.abs() > dy.abs() {
            // 绘制一个不陡峭的线段，即斜率小于1的线段，即dx>dy
            let mut y = y0;
            let derror2 = dy.abs() * 2;
            let mut error2 = 0;

            for x in start.x()..=end.x() {
                error2 += derror2;
                if error2 > dx {
                    y += if y1 > y0 { 1 } else { -1};
                    error2 -= 2 * dx;
                }
                self.draw(x, y as usize, color);
            }
        } else {
            // 绘制一个陡峭的线段，即斜率大于1的线段，即dx<dy
            let mut x = x0;
            let derror2 = dx.abs() * 2;
            let mut error2 = 0;

            for y in start.y()..=end.y() {
                error2 += derror2;
                if error2 > dy {
                    x += if x1 > x0 { 1 } else { -1};
                    error2 -= 2 * dy;
                }
                self.draw(x as usize, y, color);
            }
        }
    }
}

pub struct FrameBuffer {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![Color::new(0, 0, 0); (width * height) as usize],
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = Color::new(0, 0, 0);
        }
    }

    pub fn get_data(&self) -> &Vec<Color> {
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut Vec<Color> {
        &mut self.data
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}

impl DrawTarget for FrameBuffer {
    fn draw(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = (y * self.width + x) as usize;
        self.data[index] = color;
    }
}
