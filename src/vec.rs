use std::ops;

use num_traits::{Num, Float};

#[derive(Debug, Clone, Copy)]
pub struct Vector<T: Num, const S: usize> {
    data: [T; S],
}

impl <T: Num+Copy, const S: usize> Vector<T, S> {
    pub fn new(data: [T; S]) -> Self {
        Self { data }
    }

    pub fn new_zero() -> Self {
        Self { data: [T::zero(); S] }
    }
}

/// + operator
impl <T: Num+Copy, const S: usize> ops::Add for Vector<T, S> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = [T::zero(); S];
        for i in 0..S {
            data[i] = self.data[i] + rhs.data[i];
        }
        Self { data }
    }
}

/// - operator
impl <T: Num+Copy, const S: usize> ops::Sub for Vector<T, S> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = [T::zero(); S];
        for i in 0..S {
            data[i] = self.data[i] - rhs.data[i];
        }
        Self { data }
    }
}

/// Dot product
impl <T: Num+Copy, const S: usize> ops::Mul<Vector<T, S>> for Vector<T, S> {
    type Output = T;

    fn mul(self, rhs: Vector<T, S>) -> Self::Output {
        let mut result = T::zero();
        for i in 0..S {
            result = result + self.data[i] * rhs.data[i];
        }
        result 
    }
}

/// Scalar product
impl <T: Num+Copy, const S: usize> ops::Mul<T> for Vector<T, S> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut data = [T::zero(); S];
        for i in 0..S {
            data[i] = self.data[i] * rhs;
        }
        Self { data }
    }
}

impl <T: Num+Copy, const S: usize> ops::Div<T> for Vector<T, S> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let mut data = [T::zero(); S];
        for i in 0..S {
            data[i] = self.data[i] / rhs;
        }
        Self { data }
    }
}

impl <T: Num+Copy, const S: usize> Vector<T, S> {
    /// Norm square of the vector
    pub fn norm2(&self) -> T {
        let mut result = T::zero();
        for i in 0..S {
            result = result + self.data[i] * self.data[i];
        }
        result
    }
}

impl <T: Float, const S: usize> Vector<T, S> {
    /// Norm of the vector
    pub fn norm(&self) -> T {
        self.norm2().sqrt()
    }

    /// Normalize the vector
    pub fn normalize(&self) -> Self {
        let norm = self.norm();
        self.clone() / norm
    }
}

impl <T: Num+Copy, const S: usize> ops::Index<usize> for Vector<T, S> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl <T: Num+Copy, const S: usize> ops::IndexMut<usize> for Vector<T, S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl <T: Num+Copy> Vector<T, 2> {
    pub fn x(&self) -> T {
        self.data[0]
    }

    pub fn y(&self) -> T {
        self.data[1]
    }
}

impl <T: Num+Copy> Vector<T, 3> {
    pub fn x(&self) -> T {
        self.data[0]
    }

    pub fn y(&self) -> T {
        self.data[1]
    }

    pub fn z(&self) -> T {
        self.data[2]
    }
}

impl <T: Num+Copy> Vector<T, 4> {
    pub fn x(&self) -> T {
        self.data[0]
    }

    pub fn y(&self) -> T {
        self.data[1]
    }

    pub fn z(&self) -> T {
        self.data[2]
    }

    pub fn w(&self) -> T {
        self.data[3]
    }
}

pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;
pub type Vector4<T> = Vector<T, 4>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vector3::new([1, 2, 3]);
        let v2 = Vector3::new([4, 5, 6]);
        let v3 = v1 + v2;
        assert_eq!(v3.data, [5, 7, 9]);
    }

    #[test]
    fn test_sub() {
        let v1 = Vector3::new([1, 2, 3]);
        let v2 = Vector3::new([4, 5, 6]);
        let v3 = v1 - v2;
        assert_eq!(v3.data, [-3, -3, -3]);
    }

    #[test]
    fn test_mul() {
        let v1 = Vector3::new([1, 2, 3]);
        let v2 = Vector3::new([4, 5, 6]);
        let v3 = v1 * v2;
        assert_eq!(v3, 32);
    }

    #[test]
    fn test_mul_scalar() {
        let v1 = Vector3::new([1, 2, 3]);
        let v2 = v1 * 2;
        assert_eq!(v2.data, [2, 4, 6]);
    }

    #[test]
    fn test_div_scalar() {
        let v1 = Vector3::new([1, 2, 3]);
        let v2 = v1 / 2;
        assert_eq!(v2.data, [0, 1, 1]);
    }

    #[test]
    fn test_norm2() {
        let v1 = Vector3::new([1.0, 2.0, 3.0]);
        assert_eq!(v1.norm2(), 14.0);
    }

    #[test]
    fn test_norm() {
        let v1 = Vector3::new([1.0, 2.0, 3.0]);
        assert_eq!(v1.norm(), 14.0f64.sqrt());
    }

    #[test]
    fn test_normalize() {
        let v1 = Vector3::new([1.0, 2.0, 3.0]);
        let v2 = v1.normalize();
        assert_eq!(v2.data, [1.0/14.0f64.sqrt(), 2.0/14.0f64.sqrt(), 3.0/14.0f64.sqrt()]);
    }
}