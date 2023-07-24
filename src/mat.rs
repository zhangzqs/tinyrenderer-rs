use std::ops;
use num_traits::{Num, Float};
use crate::vec::Vector;

struct Matrix<T: Num, const R: usize, const C: usize> {
    data: [Vector<T, C>; R],
}

impl <T: Num+Copy, const R: usize, const C: usize> Matrix<T, R, C> {
    fn new(data: [Vector<T, C>; R]) -> Self {
        Self { data }
    }

    fn new_zero() -> Self {
        Self { data: [Vector::new_zero(); R] }
    }
}

impl <T: Num+Copy, const R: usize, const C: usize> ops::Add for Matrix<T, R, C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = [Vector::new_zero(); R];
        for i in 0..R {
            data[i] = self.data[i] + rhs.data[i];
        }
        Self { data }
    }
}

impl <T: Num+Copy, const R: usize, const C: usize> ops::Sub for Matrix<T, R, C> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = [Vector::new_zero(); R];
        for i in 0..R {
            data[i] = self.data[i] - rhs.data[i];
        }
        Self { data }
    }
}

impl <T: Num+Copy, const R: usize, const C: usize> ops::Index<usize> for Matrix<T, R, C> {
    type Output = Vector<T, C>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

/// R行C列的矩阵*长度为C的列向量=长度为R的列向量
impl <T: Num+Copy, const R: usize, const C: usize> ops::Mul<Vector<T, C>> for Matrix<T, R, C> {
    type Output = Vector<T, R>;

    fn mul(self, rhs: Vector<T, C>) -> Self::Output {
        let mut data = [T::zero(); R];
        for i in 0..R {
            data[i] = self.data[i] * rhs;
        }
        Vector::new(data)
    }
}

/// R行C列的矩阵*C行D列的矩阵=R行D列的矩阵
impl <T: Num+Copy, const R: usize, const C: usize, const D: usize> ops::Mul<Matrix<T, C, D>> for Matrix<T, R, C> {
    type Output = Matrix<T, R, D>;

    fn mul(self, rhs: Matrix<T, C, D>) -> Self::Output {
        let mut data = [Vector::new_zero(); R];
        for i in 0..R {
            for j in 0..D {
                for k in 0..C {
                    data[i][j] = data[i][j] + self.data[i][k] * rhs.data[k][j];
                }
            }
        }
        Matrix::new(data)
    }
}